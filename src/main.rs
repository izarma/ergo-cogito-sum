use bevy::prelude::*;
use plugins::game_runner::GameRunnerPlugin;

mod components;
mod resources;
mod systems;
mod plugins;
 
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameRunnerPlugin)
        .run();
}