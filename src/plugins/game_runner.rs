use bevy::prelude::*;
use crate::components;
use crate::systems::{greeting_system::greeting_system, scenario_management::scenario_manager};
use crate::resources::{game_state::GameState, am_actions::AmActions, selection_timer::SelectionTimer};

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectionTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .insert_resource(GameState { current_scenario: None , rounds_completed: 1})
            .insert_resource(AmActions { active_modifiers: Vec::new() })
            .add_systems(Startup, add_players)
            .add_systems(Update, (greeting_system, scenario_manager).chain());
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