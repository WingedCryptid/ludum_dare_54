use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::AssetServer;
use bevy::core::Zeroable;
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{BuildChildren, Color, Commands, Component, KeyCode, Query, Res, SpatialBundle, TextStyle, Transform, With};
use bevy::sprite::SpriteBundle;
use bevy::text::{Text, Text2dBundle};
use bevy::time::Time;
use bevy::utils::default;
use crate::common::{MovingObject, SpeedLabel};

pub struct PlayerPlugin;

    impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, move_player);
    }
}

#[derive(Component)]
struct Player{
    speed : Vec3
}

fn spawn_player(
        mut commands : Commands,
        asset_server: Res<AssetServer>) {

    commands.spawn((
        SpatialBundle::default(), 
        Player {
            speed: Vec3 {
                x: 50f32,
                y: 100f32,
                z: 0.0,
            },
        },
        MovingObject{ speed: 0.9f32 }
    ))
        .with_children(|parent| {
            parent.spawn(
                SpriteBundle {
                    transform: Transform::from_scale(Vec3 {
                        x: 0.1f32,
                        y: 0.1f32,
                        z: 0.1f32,
                    }),
                    texture: asset_server.load("gosling.png"),
                    ..default()
                });
        })
        .with_children(|parent| {
            parent.spawn((Text2dBundle {
                text: Text::from_section("KOQ", TextStyle {
                    font_size: 24.0,
                    color: Color::AZURE,
                    ..default()
                }),
                transform: Transform::from_xyz(0f32, 0f32, 1f32),
                ..default()
            },SpeedLabel));
        });
}


fn move_player(
        keys : Res<Input<KeyCode>>,
        time : Res<Time>,
        player_q : Query<& Player>,
        mut player_trans_q : Query<&mut Transform, With<Player>>){

    let mut player_transform = player_trans_q.get_single_mut().unwrap();
    let direction = get_direction(&keys);
    player_transform.translation += direction * player_q.get_single().unwrap().speed * time.delta_seconds();
}

fn get_direction(keys : &Input<KeyCode>) -> Vec3{
    let mut direction = Vec3::zeroed();

    if keys.pressed(KeyCode::D){
        direction += Vec3{x:1f32, y:0f32, z:0f32};
    }
    if keys.pressed(KeyCode::W){
        direction += Vec3{x:0f32, y:1f32, z:0f32};
    }
    if keys.pressed(KeyCode::A){
        direction += Vec3{x:-1f32, y:0f32, z:0f32};
    }
    if keys.pressed(KeyCode::S) {
        direction += Vec3 { x: 0f32, y: -1f32, z: 0f32 };
    }

    return direction;
}