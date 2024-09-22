use bevy::prelude::*;
use crate::GameState;
use crate::consts;
use bevy::window::PrimaryWindow;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;

pub struct RoomCreator;

#[derive(Component)]
struct OnRoomCreatorScreen;


#[derive(Component)]
struct RoomTypeToggle;

#[derive(Component)]
struct RoomNameInputField;

#[derive(Component)]
struct RoomNameText;

#[derive(Component)]
struct ConfirmButton;

#[derive(Component)]
struct RoomTypeToggleText;

// Resource to store the room creation data
#[derive(Resource)]
struct RoomCreationData {
    is_private: bool,
    room_name: String,
}

impl Default for RoomCreationData {
    fn default() -> Self {
        Self {
            is_private: false,
            room_name: String::new(),
        }
    }
}

impl Plugin for RoomCreator {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<RoomCreationData>()
        .add_systems(OnEnter(GameState::CreateRoom),setup_room_selector)
        .add_systems(Update, (handle_button_interactions,handle_text_input).run_if(in_state(GameState::CreateRoom)))
        .add_systems(OnExit(GameState::CreateRoom),cleanup_room_creator_ui);
    }
}

// System to set up the room creator UI
fn setup_room_selector(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window: &Window = window_query.get_single().unwrap();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                ..Default::default()
            },
            OnRoomCreatorScreen,
        ))
        .with_children(|parent| {
            // Room Type Toggle Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            margin: UiRect::vertical(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: consts::NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    RoomTypeToggle,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "Room Type: Public",
                        TextStyle {
                            font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ),
                    RoomTypeToggleText));
                });

            // Room Name Input Field
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            margin: UiRect::vertical(Val::Px(5.0)),
                            padding: UiRect::horizontal(Val::Px(10.0)),
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: consts::INPUT_FIELD_BG.into(),
                        ..Default::default()
                    },
                    RoomNameInputField,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Room Name: ",
                        TextStyle {
                            font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                            font_size: 25.0,
                            color: Color::WHITE,
                        },
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                                font_size: 25.0,
                                color: consts::WHITE.into(),
                            },
                        ),
                        RoomNameText,
                    ));
                });

            // Confirm Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            margin: UiRect::vertical(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: consts::NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    ConfirmButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Create Room",
                        TextStyle {
                            font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

// System to handle button interactions
fn handle_button_interactions(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&RoomTypeToggle>,
            Option<&ConfirmButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut room_data: ResMut<RoomCreationData>,
    mut toggle_text_query: Query<&mut Text, With<RoomTypeToggleText>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, is_toggle, is_confirm) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if is_toggle.is_some() {
                    // Toggle room type
                    room_data.is_private = !room_data.is_private;
                    // Update the toggle button text
                    for mut text in &mut toggle_text_query {
                        text.sections[0].value = format!(
                            "Room Type: {}",
                            if room_data.is_private { "Private" } else { "Public" }
                        );
                    }
                } else if is_confirm.is_some() {
                    // Confirm room creation
                    if room_data.room_name.is_empty() {
                        println!("Please enter a room name.");
                    } else {
                        println!(
                            "Creating {} room: {}",
                            if room_data.is_private { "Private" } else { "Public" },
                            room_data.room_name
                        );
                        // Transition to the next game state or handle room creation logic
                        game_state.set(GameState::Lobby);
                    }
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

// System to handle text input for the room name
fn handle_text_input(
    mut keyboard_input: EventReader<KeyboardInput>,
    mut room_data: ResMut<RoomCreationData>,
    mut room_name_text_query: Query<&mut Text, With<RoomNameText>>,
) {
    // Handle character input
    for ev in keyboard_input.read() {
        // We don't care about key releases, only key presses
        if ev.state == ButtonState::Released {
            continue;
        }

        match &ev.logical_key {
            // Handle Pressing Enter to finish the input
            Key::Backspace => {
                room_data.room_name.pop();
            }
            Key::Character(input) => {
            // Ignore any input that contains control (special) characters
            if input.chars().any(|c| c.is_control()) {
                continue;
            }
            room_data.room_name.push_str(&input);
            }
            _ => {}
         }
         // Update the displayed room name
        for mut text in &mut room_name_text_query {
            text.sections[0].value = room_data.room_name.clone();
        }
    }
}

// System to clean up the UI when exiting the room creator state
fn cleanup_room_creator_ui(mut commands: Commands, query: Query<Entity, With<OnRoomCreatorScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}