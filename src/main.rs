use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use bevy::app::{AppExit, ScheduleRunnerPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use clap::Parser;

use client::SteamClientResource;
use steamworks::{Client, SingleClient};

mod server;
use crate::server::runner::ServerRunnerPlugin;
use crate::server::ServerPlugin;

mod client;
use crate::client::{ClientPlugin};

#[derive(Parser, Debug, Copy, Clone, Resource)]
#[command(author, version, about, long_about = None)]
struct CMDArgs {
    #[arg(long, default_value_t = false)]
    dedicated: bool,

    #[arg(long, default_value_t = false)]
    no_server: bool,

    #[arg(long, default_value_t = 60)]
    tickrate: u16,
}

#[derive(Resource)]
pub struct ShouldExit(pub Arc<AtomicBool>);

fn main() {
    let args = match CMDArgs::try_parse() {
        Ok(a) => a,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    // If running in dedicated server mode, the server will be run on this app instance
    // If running in client mode, this app instance will be the client and we'll create another app for the server on another thread
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugins(LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,wgpu_core=warn,wgpu_hal=warn,idk=debug".into(),
    });

    #[cfg(not(debug_assertions))]
    app.add_plugins(LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    });

    app.insert_resource(args);

    let should_exit = Arc::new(AtomicBool::new(false));
    let app_should_exit = should_exit.clone();
    let ctrlc_should_exit = should_exit.clone();

    ctrlc::set_handler(move || {
        ctrlc_should_exit.store(true, std::sync::atomic::Ordering::Relaxed);
    });

    app.insert_resource(ShouldExit(app_should_exit));

    if !args.dedicated {
        let steam_client = match Client::init() {
            Ok(client) => client,
            Err(error) => {
                error!("{:?}", error);
                msgbox::create(
                    "Steam Error!",
                    format!("{:?}", error).as_str(),
                    msgbox::IconType::Error,
                );
                return;
            }
        };

        let client_arc = Arc::new(Mutex::new(steam_client));

        app.insert_resource(SteamClientResource(client_arc.clone()));
        app.insert_resource(WinitSettings {
            return_from_run: true,
            ..default()
        });

        app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>());
        app.add_plugins(ClientPlugin);

        let mut server_thread = None;

        let server_should_exit = should_exit.clone();

        if !args.no_server {
            server_thread = Some(thread::spawn(move || {
                let mut server_app = App::new();

                server_app.add_event::<AppExit>();

                server_app.insert_resource(SteamClientResource(client_arc.clone()));
                server_app.insert_resource(ShouldExit(server_should_exit));
                server_app.insert_resource(args);

                let wait_time_secs = 1.0 / args.tickrate as f64;
                server_app.add_plugins(ServerRunnerPlugin::run_loop(Duration::from_secs_f64(
                    wait_time_secs,
                )));
                server_app.add_plugins(MinimalPlugins.build().disable::<ScheduleRunnerPlugin>());

                server_app.add_plugins(ServerPlugin);

                server_app.run();

                info!("Server shutdown.");
            }));
        }

        app.run();

        // Ensure should_exit is set to true if the window is closed.
        should_exit.store(true, std::sync::atomic::Ordering::Relaxed);

        if let Some(sv_thread) = server_thread {
            sv_thread.join().unwrap();
        }
    } else if !args.no_server {
        let wait_time_secs = 1.0 / args.tickrate as f64;
        app.add_plugins(ServerRunnerPlugin::run_loop(Duration::from_secs_f64(
            wait_time_secs,
        )));
        app.add_plugins(MinimalPlugins.build().disable::<ScheduleRunnerPlugin>());

        app.add_plugins(ServerPlugin);

        app.run();

        info!("Server shutdown.");
    }
}
