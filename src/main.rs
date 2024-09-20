use bevy::prelude::*;
use plugins::game_runner::GameRunnerPlugin;
use plugins::main_menu::MainMenuPlugin;

mod components;
mod resources;
mod systems;
mod plugins;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Resource)]
enum GameState {
    MainMenu,
    Lobby,
    InGame,
}
 
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState::MainMenu)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GameRunnerPlugin)
        .run();
}