use bevy::{prelude::*, app::RunFixedUpdateLoop};
use bevy_egui::EguiPlugin;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        info!("Building Client!");

        app.add_plugins(EguiPlugin);
        app.add_systems(RunFixedUpdateLoop, fixed_system);
    }
}

fn fixed_system(time: Res<Time>) {
    //info!("Fixed running. delta time: {} / fps: {}", time.delta_seconds(), 1.0 / time.delta_seconds());
}