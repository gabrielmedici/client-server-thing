use std::{sync::{Arc, Mutex}, time::Duration};

use bevy::{prelude::*, app::{RunFixedUpdateLoop, AppExit}};
use bevy_egui::EguiPlugin;
use steamworks::{Client, SingleClient, networking_sockets::NetConnection, ClientManager};

use crate::ShouldExit;

pub struct ClientPlugin;

#[derive(Resource)]
pub struct SteamClientResource(pub Arc<Mutex<(Client, SingleClient)>>);

#[derive(Resource)]
struct NetConn(Option<NetConnection<ClientManager>>);

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        info!("Building Client!");

        app.insert_resource(NetConn(None));

        app.add_plugins(EguiPlugin);
        app.add_systems(RunFixedUpdateLoop, fixed_system);
        app.add_systems(Update, (watch_exit, steam_client_callbacks_runner));
        app.add_systems(PostStartup, connect_to_server);
    }
}

fn steam_client_callbacks_runner(client: Res<SteamClientResource>) {
    client.0.lock().unwrap().1.run_callbacks();
}

fn connect_to_server(client: Res<SteamClientResource>, mut net_conn: ResMut<NetConn> ) {
    std::thread::sleep(Duration::from_secs(1));

    let socks = client.0.lock().unwrap().0.networking_sockets();

    let conn = match socks.connect_by_ip_address("192.168.1.7:27000".parse().unwrap(), []) {
        Ok(netcon) => netcon,
        Err(err) => {
            error!("Something went wrong connecting to the server");
            return;
        },
    };

    info!("Connecting to the server");

    net_conn.0 = Some(conn);
}

fn watch_exit(should_exit: Res<ShouldExit>, mut events: EventWriter<AppExit>) {
    if should_exit.0.load(std::sync::atomic::Ordering::Relaxed) {
        info!("Client shutdown requested.");
        events.send(AppExit);
    }
}

fn fixed_system(time: Res<Time>) {
    //info!("Fixed running. delta time: {} / fps: {}", time.delta_seconds(), 1.0 / time.delta_seconds());
}