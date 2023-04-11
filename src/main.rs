use std::time::Duration;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy::time::common_conditions::on_timer;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::{prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::Position;
use game::snake::SnakePlugin;


pub mod game;


fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}



fn main() {
    App::new()
        
        .register_type::<Position>()

        .add_startup_system(setup_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(SnakePlugin)
        .run();
}
