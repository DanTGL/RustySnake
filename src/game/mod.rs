use std::time::Duration;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy::time::common_conditions::on_timer;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::{prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod food;
pub mod snake;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn size_scaling(primary_query: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Size, &mut Transform)>) {
    let window = primary_query.get_single().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(primary_query: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = primary_query.get_single().unwrap();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}