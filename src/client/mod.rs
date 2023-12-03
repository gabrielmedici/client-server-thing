use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use bevy::{
    app::{AppExit, RunFixedUpdateLoop},
    prelude::*,
};
use bevy_egui::EguiPlugin;
use steamworks::{networking_sockets::NetConnection, Client, ClientManager, SingleClient, networking_types::{NetworkingConfigEntry, NetworkingConfigValue, NetConnectionStatusChanged}};

use crate::ShouldExit;

pub struct ClientPlugin;

#[derive(Resource)]
pub struct SteamClientResource(pub Arc<Mutex<(Client, SingleClient)>>);

#[derive(Resource)]
struct ServerConnection(NetConnection<ClientManager>);

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        info!("Building Client!");

        app.add_plugins(EguiPlugin);
        app.add_systems(RunFixedUpdateLoop, fixed_system);
        app.add_systems(Update, (watch_exit, steam_client_callbacks_runner));
    }
}

fn steam_client_callbacks_runner(client: Res<SteamClientResource>) {
    client.0.lock().unwrap().1.run_callbacks();
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
