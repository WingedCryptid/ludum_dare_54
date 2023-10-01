use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle};

pub struct TrafficPlugin;

impl Plugin for TrafficPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_traffic);
    }
}

fn spawn_traffic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let traffic = MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
            x: 200.0,
            y: 200.0
        }))).into(),
        material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };

    commands.spawn(traffic);
}