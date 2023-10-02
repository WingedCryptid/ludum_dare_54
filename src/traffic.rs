use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::common::MovingObject;
use crate::game::Game;
use std::ops::Range;
use rand::Rng;

pub struct TrafficPlugin;

impl Plugin for TrafficPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_traffic)
            .add_systems(Update, (move_traffic, try_cross_dead_zone));
    }
}

#[derive(Component)]
struct TrafficUnit;

#[derive(Component)]
struct TrafficSettings {
    unit_speed: Range<f32>,
    unit_spawn_x: Range<f32>,
    unit_spawn_y: Range<f32>,
}

fn spawn_traffic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mut rng = rand::thread_rng();
    let traffic_settings = TrafficSettings {
        unit_speed: (0.75..1.0),
        unit_spawn_x: (0.0..50.0),
        unit_spawn_y: (-250.0..250.0),
    };

    for _ in 0..5 {
        let unit_speed = rng.gen_range(traffic_settings.unit_speed.clone());
        let position = Vec3::new(
            rng.gen_range(traffic_settings.unit_spawn_x.clone()), 
            rng.gen_range(traffic_settings.unit_spawn_y.clone()), 
            0.0);
        let traffic_unit = (MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 {
                x: 25.0,
                y: 25.0
            }))).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(position),
            ..default()
        },
        TrafficUnit,
        MovingObject {
            speed: unit_speed
        },
        );

        commands.spawn(traffic_unit);
    }
}

fn move_traffic(
    time: Res<Time>,
    mut traffic_units_q: Query<(&mut Transform, &MovingObject), With<TrafficUnit>>
) {
    let direction = Vec3::X;
    let mut movement = direction * 100.0 * time.delta_seconds();
    for mut traffic_unit in traffic_units_q.iter_mut() {
        movement *= traffic_unit.1.speed;
        traffic_unit.0.translation += movement;
    }
}

fn try_cross_dead_zone(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut traffic_units_q: Query<(Entity, &mut Transform), With<TrafficUnit>>,
) {
    for traffic_unit in traffic_units_q.iter_mut() {
        if traffic_unit.1.translation.x > 1280f32 {
            commands.entity(traffic_unit.0).despawn_recursive();
            game.on_dead_zone_crossed(); 
        }
    }
}