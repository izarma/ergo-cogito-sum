use bevy::prelude::*;
use crate::components::scenario::Scenario;
use crate::resources::{game_state::GameState, am_actions::AmActions};

pub fn scenario_manager(
    time: Res<Time>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut am_actions: ResMut<AmActions>,
) {
    // Initialize a scenario if none exists
    if game_state.current_scenario.is_none() {
        let new_scenario = Scenario {
            name: "The Pit".to_string(),
            objective: "Escape the pit using limited supplies.".to_string(),
        };
        game_state.current_scenario = Some(new_scenario);
        println!("New Scenario: {}", game_state.current_scenario.as_ref().unwrap().name);
        println!("Objective: {}", game_state.current_scenario.as_ref().unwrap().objective);
        
        // Example of adding a modifier
        let modifier = crate::components::modifier::Modifier {
            effect: "Limited visibility".to_string(),
        };
        am_actions.active_modifiers.push(modifier);
    }

    // Example of applying modifiers (you can expand this logic)
    if let Some(scenario) = &game_state.current_scenario {
        for modifier in &am_actions.active_modifiers {
            println!("Applying modifier: {}", modifier.effect);
        }
    }

    // For demonstration, reset the scenario after a certain time
    if time.delta_seconds() > 10.0 {
        println!("Scenario reset after 10 seconds.");
        game_state.current_scenario = None; // Reset scenario for now
        am_actions.active_modifiers.clear(); // Clear modifiers
    }
}