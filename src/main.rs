mod common;
mod game;
mod traffic;
mod camera;

use bevy::prelude::*;
use crate::traffic::TrafficPlugin;
use crate::camera::CameraPlugin;
use crate::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, GamePlugin, TrafficPlugin))
        .run();
}
