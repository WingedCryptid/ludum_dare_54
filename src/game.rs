use bevy::{prelude::*, app::AppExit};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game)
        .add_systems(Update, try_exit);
    }
}

#[derive(Resource)]
pub struct Game {
    points: u8
}

impl Game {
    pub fn on_dead_zone_crossed(&mut self) {
        self.points -= 1;
    }

    fn check_end_condition(&mut self) -> bool {
        return self.points == 0
    }
}

fn spawn_game(mut commands: Commands) {
        commands.insert_resource(Game { points: 5u8 });
}

fn try_exit(mut game: ResMut<Game>, mut exit: EventWriter<AppExit>) {
    if game.check_end_condition() {
        exit.send(AppExit);
    }
}