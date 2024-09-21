use bevy::prelude::*;
use plugins::game_runner::GameRunnerPlugin;
use plugins::lobby::LobbyPlugin;
use plugins::main_menu::MainMenuPlugin;

mod components;
mod resources;
mod systems;
mod plugins;

#[derive(Debug, Eq, PartialEq, Hash, Resource, States, Default, Clone)]
enum GameState {
    #[default]
    MainMenu,
    Lobby,
}
 
fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GameRunnerPlugin)
        //.add_plugins(LobbyPlugin)
        .run();
}