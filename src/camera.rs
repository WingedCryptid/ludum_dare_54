use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        /*projection: OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scale: 5.0,
            ..default()
        },*/
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    };

    commands.spawn(camera);
}