use bevy::prelude::*;
use std::ops::Range;
use rand::Rng;
use crate::common::MovingObject;
use crate::game::Game;

pub struct TrafficPlugin;

impl Plugin for TrafficPlugin {
    fn build(&self, app: &mut App) {
        let traffic_settings = TrafficSettings {
            unit_speed: (0.75..1.0),
            offender_speed: (1.1..1.5),
            unit_spawn_x: (0.0..50.0),
            unit_spawn_y: (-250.0..250.0),
            units_count: 8,
            offenders_count: 5,
        };

        app.insert_resource(traffic_settings)
        .add_systems(Startup, spawn_traffic)
        .add_systems(Update, (move_traffic, try_cross_dead_zone));
    }
}

#[derive(Component)]
struct Offender;

#[derive(Component)]
struct TrafficUnit;

#[derive(Bundle)]
struct OffenderBundle {
    traffic_unit_bundle: TrafficUnitBundle,
    offender: Offender,
}

#[derive(Bundle)]
struct TrafficUnitBundle {
    sprite_bundle: SpriteBundle,
    traffic_unit: TrafficUnit,
    moving_object: MovingObject,
}

#[derive(Resource)]
struct TrafficSettings {
    unit_speed: Range<f32>,
    offender_speed: Range<f32>,
    unit_spawn_x: Range<f32>,
    unit_spawn_y: Range<f32>,
    units_count: u8,
    offenders_count: u8,
}

fn spawn_traffic(
    mut commands: Commands,
    traffic_settings: Res<TrafficSettings>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..traffic_settings.units_count {
        let unit_speed = rng.gen_range(traffic_settings.unit_speed.clone());
        let unit_color = Color::YELLOW;
        let unit_size = Vec2::new(25f32, 25f32);
        let unit_position = Vec3::new(
            rng.gen_range(traffic_settings.unit_spawn_x.clone()),
            rng.gen_range(traffic_settings.unit_spawn_y.clone()),
            0.0);

        let traffic_unit = spawn_traffic_unit(
            unit_color,
            unit_size,
            unit_position,
            unit_speed,
        );

        commands.spawn(traffic_unit);
    }

    for _ in 0..traffic_settings.offenders_count {
        let unit_speed = rng.gen_range(traffic_settings.offender_speed.clone());
        let unit_color = Color::RED;
        let unit_size = Vec2::new(25f32, 25f32);
        let unit_position = Vec3::new(
            rng.gen_range(traffic_settings.unit_spawn_x.clone()),
            rng.gen_range(traffic_settings.unit_spawn_y.clone()),
            0.0);

        let offender = spawn_offender(
            unit_color,
            unit_size,
            unit_position,
            unit_speed,
        );

        commands.spawn(offender);
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
    mut offenders_q: Query<(Entity, &mut Transform), With<Offender>>,
) {
    for offender in offenders_q.iter_mut() {
        if offender.1.translation.x > 1280f32 {
            commands.entity(offender.0).despawn_recursive();
            game.on_dead_zone_crossed(); 
        }
    }
}

fn spawn_offender(
    color: Color,
    size: Vec2,
    position: Vec3,
    unit_speed: f32,
) -> OffenderBundle {
    return OffenderBundle {
        traffic_unit_bundle: spawn_traffic_unit(
            color, 
            size, 
            position, 
            unit_speed),
        offender: Offender,
    };
}

fn spawn_traffic_unit(
    color: Color,
    size: Vec2,
    position: Vec3,
    unit_speed: f32,
) -> TrafficUnitBundle {
    return TrafficUnitBundle {
        sprite_bundle: SpriteBundle {
            sprite: Sprite { 
                color: color,
                custom_size: Option::Some(size),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        traffic_unit: TrafficUnit,
        moving_object: MovingObject { speed: unit_speed }
    }
}