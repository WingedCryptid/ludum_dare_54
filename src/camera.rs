use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, -10.0),
        ..default()
    };

    commands.spawn(camera);
}