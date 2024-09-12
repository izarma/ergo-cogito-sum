use bevy::prelude::*;

#[derive(Resource)]
pub struct AmActions {
    pub active_modifiers: Vec<crate::components::modifier::Modifier>,
}