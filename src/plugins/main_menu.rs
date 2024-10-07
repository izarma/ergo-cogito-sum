use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::GameState;
use crate::consts;

pub struct MainMenuPlugin;

// Component for marking buttons (Host or Join)

#[derive(Component)]
struct HostButton;

#[derive(Component)]
struct JoinButton;

#[derive(Component)]
struct OnMainMenuScreen;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_systems(OnEnter(GameState::MainMenu),setup_main_menu)
        .add_systems(Update, button_interaction_system.run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu),cleanup_menu);

    }
}

// System to setup the main menu UI
fn setup_main_menu(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window: &Window = window_query.get_single().unwrap();
    // UI setup with Host and Join buttons
    
    commands
        .spawn((NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        },
        OnMainMenuScreen,))
        .with_children(|parent| {
            parent
                // Host Button
                .spawn(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        ..Default::default()
                    },
                    background_color: consts::NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(HostButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Host Game",
                        TextStyle {
                            font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            parent
                // Join Button
                .spawn(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        ..Default::default()
                    },
                    background_color: consts::NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(JoinButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Join Game",
                        TextStyle {
                            font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

// System to handle button interaction
fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&HostButton>, Option<&JoinButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>
) {
    for (interaction, mut color, host_button, join_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if host_button.is_some() {
                    println!("Host Game Button Clicked");// Switch to Lobby state
                    game_state.set(GameState::CreateRoom);
                } else if join_button.is_some() {
                    println!("Join Game Button Clicked");// Switch to Lobby state
                    game_state.set(GameState::InGame);
                }
            }
            Interaction::Hovered => {
                *color = consts::HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = consts::NORMAL_BUTTON.into();
            }
        }
    }
}

// System to cleanup menu when exiting MainMenu state
fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<OnMainMenuScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
