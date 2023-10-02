use bevy::prelude::*;
use std::ops::Range;
use rand::Rng;
use crate::common::{MovingObject, SpeedLabel};
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
    spatial_bundle: SpatialBundle,
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
        let unit_speed = (unit_speed * 10f32).round() / 10f32;
        let unit_color = Color::YELLOW;
        let unit_size = Vec2::new(25f32, 25f32);
        let unit_position = Vec3::new(
            rng.gen_range(traffic_settings.unit_spawn_x.clone()),
            rng.gen_range(traffic_settings.unit_spawn_y.clone()),
            0.0);

        let traffic_unit = create_traffic_unit(
            unit_color,
            unit_size,
            unit_position,
            unit_speed,
        );
        
        commands.spawn(traffic_unit.0)
            .with_children(|parent| { parent.spawn(traffic_unit.1); })
                .with_children(|parent| { parent.spawn(traffic_unit.2); });
    }

    for _ in 0..traffic_settings.offenders_count {
        let unit_speed = rng.gen_range(traffic_settings.offender_speed.clone());
        let unit_speed = (unit_speed * 10f32).round() / 10f32;
        let unit_color = Color::RED;
        let unit_size = Vec2::new(25f32, 25f32);
        let unit_position = Vec3::new(
            rng.gen_range(traffic_settings.unit_spawn_x.clone()),
            rng.gen_range(traffic_settings.unit_spawn_y.clone()),
            0.0);

        let offender = create_offender(
            unit_color,
            unit_size,
            unit_position,
            unit_speed,
        );

        commands.spawn(offender.0)
            .with_children(|parent| { parent.spawn(offender.1); })
                .with_children(|parent| { parent.spawn(offender.2); });
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

fn create_offender(
    color: Color,
    size: Vec2,
    position: Vec3,
    unit_speed: f32,
) -> (OffenderBundle, SpriteBundle, (Text2dBundle, SpeedLabel)) {

    let traffic_unit = create_traffic_unit(
        color, 
        size, 
        position, 
        unit_speed);

    return (
        OffenderBundle {
            traffic_unit_bundle: traffic_unit.0,
            offender: Offender
        },
        traffic_unit.1,
        traffic_unit.2,
    );
}

fn create_traffic_unit(
    color: Color,
    size: Vec2,
    position: Vec3,
    unit_speed: f32,
) -> (TrafficUnitBundle, SpriteBundle, (Text2dBundle, SpeedLabel)) {

    let traffic_unit_bundle = TrafficUnitBundle {
        spatial_bundle: SpatialBundle::default(),
        traffic_unit: TrafficUnit,
        moving_object: MovingObject { speed: unit_speed }
    };

    let sprite_bundle = SpriteBundle {
        sprite: Sprite { 
            color,
            custom_size: Option::Some(size),
            ..default()
        },
        transform: Transform::from_translation(position),
        ..default()
    };

    let text_bundle = (
        Text2dBundle {
        text: Text::from_section(
            "KOQ", 
            TextStyle {
                font_size: 24.0,
                color: Color::AZURE,
                ..default()
            }),
            transform: Transform::from_translation(position + Vec3::Z),
            ..default()
        },
        SpeedLabel
    );
    
    return (traffic_unit_bundle, sprite_bundle, text_bundle);
}