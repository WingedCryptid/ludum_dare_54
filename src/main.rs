mod player;
mod traffic;
mod camera;

use bevy::prelude::*;
use crate::traffic::TrafficPlugin;
use crate::camera::CameraPlugin;
use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, TrafficPlugin, PlayerPlugin))
        .run();
}
