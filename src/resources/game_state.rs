use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub current_scenario: Option<crate::components::scenario::Scenario>,
    pub rounds_completed: u32,
}