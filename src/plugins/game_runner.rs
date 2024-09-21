use bevy::prelude::*;
use crate::{components, GameState};
use crate::systems::greeting_system::greeting_system;
use crate::resources::selection_timer::SelectionTimer;

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectionTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .add_systems(Startup, (add_players, welcome_monologue, start_main_menu))
            .add_systems(Update, greeting_system);
    }
}

fn add_players(mut commands: Commands) {
    commands.spawn((components::person::Person, components::person::Name("Gorrister".to_string())));
    commands.spawn((components::person::Person, components::person::Name("Benny".to_string())));
    commands.spawn((components::person::Person, components::person::Name("Ellen".to_string())));
    commands.spawn((components::person::Person, components::person::Name("Nimdok".to_string())));
    commands.spawn((components::person::Person, components::person::Name("Ted".to_string())));
    commands.spawn((components::person::Person, components::person::Name("AM".to_string()))); // AI player
}

fn welcome_monologue() {
    println!("Hate Monologue");
}

fn start_main_menu(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::MainMenu);
}