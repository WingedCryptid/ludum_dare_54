use std::process::Child;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::text::Text;
use crate::common::{MovingObject, SpeedLabel};

pub struct TextRenderingPlugin;

impl Plugin for TextRenderingPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_speed_labels);
    }
}

fn render_speed_labels(mut text_q : Query<(&mut Text, &Parent), With<SpeedLabel>>,
                        parent_q : Query<&MovingObject>){

    for mut speed_text in text_q.iter_mut(){
        match parent_q.get(speed_text.1.get()) {
            Err(_) => {},
            Ok(moving_object) => {
                speed_text.0.sections.get_mut(0).unwrap().value = moving_object.speed.to_string();
            }
        }
    }
}