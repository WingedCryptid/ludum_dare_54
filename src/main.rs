mod traffic;
mod camera;

use bevy::prelude::*;
use crate::traffic::TrafficPlugin;
use crate::camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, TrafficPlugin))
        .run();
}
