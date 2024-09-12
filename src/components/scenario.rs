use bevy::prelude::*;

#[derive(Component)]
pub struct Scenario {
    pub name: String,
    pub objective: String,
}