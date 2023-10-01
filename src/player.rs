use bevy::app::{App, Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res, Transform};
use bevy::sprite::{SpriteBundle};
use bevy::utils::default;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands : Commands, asset_server: Res<AssetServer>){
    let player = SpriteBundle{
        transform: Transform::from_xyz(0f32, 0f32, 0f32),
        texture: asset_server.load("gosling.png"),
        ..default()
    };

    commands.spawn(player);
}

fn move_player(mut commands : Commands){

}
