use std::{
    net::Ipv4Addr,
    process::exit,
    sync::{Arc, Mutex, atomic::AtomicBool},
    time::Instant, ffi::CString,
};
use steamworks::{Server, ServerManager, SingleClient, SteamServerConnectFailure, SteamServersConnected, SteamServersDisconnected};

use bevy::{
    app::{AppExit, AppLabel, RunFixedUpdateLoop},
    prelude::*,
};

use crate::ShouldExit;
pub mod runner;

pub struct ServerPlugin;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
pub struct ServerApp;

#[derive(Resource)]
struct SteamServerResource(Arc<Mutex<(Server, SingleClient<ServerManager>)>>);

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        info!("Building Server!");
        let steam_server = match Server::init(
            Ipv4Addr::new(192, 168, 1, 7),
            27001,
            27000,
            27010,
            steamworks::ServerMode::Authentication,
            "1.0.0",
        ) {
            Ok(server) => {
                info!("Steam server initialized successfully!");
                server
            },
            Err(error) => {
                error!("Error initializing steam server: {}", error);
                msgbox::create("Error initializing Steam Server!", format!("{}", error).as_str(), msgbox::IconType::Error);
                return;
            },
        };

        steam_server.0.register_callback(|failure: SteamServerConnectFailure| {
            error!("Failed connecting to Steam servers \"{}\". {}", failure.reason, if failure.still_retrying { "We'll try again." } else { "We won't be trying again." });
        });

        steam_server.0.register_callback(|_: SteamServersConnected| {
            info!("Connected to Steam servers!");
        });

        steam_server.0.register_callback(|disconnected: SteamServersDisconnected| {
            error!("We have been disconnected from the Steam servers \"{}\"", disconnected.reason);
        });

        steam_server.0.set_product("idk");
        steam_server.0.set_game_description("basic server test");
        steam_server.0.set_dedicated_server(true);
        steam_server.0.enable_heartbeats(true);
        steam_server.0.log_on_anonymous();

        info!("Server SteamID: {:?}", steam_server.0.steam_id());

        app.insert_resource(SteamServerResource(Arc::new(Mutex::new(steam_server))));
        app.add_systems(RunFixedUpdateLoop, steam_server_callbacks_runner);
        app.add_systems(Update, watch_exit);
    }
}

fn watch_exit(should_exit: Res<ShouldExit>, mut events: EventWriter<AppExit>) {
    if should_exit.0.load(std::sync::atomic::Ordering::Relaxed) {
        info!("Server shutdown requested.");
        events.send(AppExit);
    }
}

fn steam_server_callbacks_runner(server: Res<SteamServerResource>) {
    server.0.lock().unwrap().1.run_callbacks();
}

fn fixed_system(time: Res<Time>) {
    //info!("Fixed running. delta time: {} / fps: {}", time.delta_seconds(), 1.0 / time.delta_seconds());
}
