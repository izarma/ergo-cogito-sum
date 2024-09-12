use bevy::prelude::*;
use crate::components::person::Name;
use crate::resources::selection_timer::SelectionTimer;

pub fn greeting_system(time: Res<Time>, mut timer: ResMut<SelectionTimer>, query: Query<&Name, With<crate::components::person::Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}