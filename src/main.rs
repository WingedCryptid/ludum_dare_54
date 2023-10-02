mod common;
mod game;
mod traffic;
mod camera;
mod player;
mod text_rendering;

use bevy::prelude::*;
use crate::traffic::TrafficPlugin;
use crate::camera::CameraPlugin;
use crate::player::PlayerPlugin;
use crate::text_rendering::TextRenderingPlugin;
use crate::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, 
            CameraPlugin, 
            TrafficPlugin, 
            PlayerPlugin, 
            GamePlugin,
            TextRenderingPlugin))
            .run();
}
