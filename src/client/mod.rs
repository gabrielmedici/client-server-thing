use bevy::{prelude::*, app::{RunFixedUpdateLoop, AppExit}};
use bevy_egui::EguiPlugin;

use crate::ShouldExit;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        info!("Building Client!");

        app.add_plugins(EguiPlugin);
        app.add_systems(RunFixedUpdateLoop, fixed_system);
        app.add_systems(Update, watch_exit);
    }
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