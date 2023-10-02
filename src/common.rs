use bevy::prelude::*;

#[derive(Component)]
pub struct MovingObject {
    pub speed: f32
}

#[derive(Component)]
pub struct SpeedLabel;