mod common;
mod traffic;
mod camera;
mod player;
mod text_rendering;

use bevy::prelude::*;
use crate::traffic::TrafficPlugin;
use crate::camera::CameraPlugin;
use crate::player::PlayerPlugin;
use crate::text_rendering::TextRenderingPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, TrafficPlugin, PlayerPlugin, TextRenderingPlugin))
        .run();
}
