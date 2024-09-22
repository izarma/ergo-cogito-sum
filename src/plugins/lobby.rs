use bevy::prelude::*;
use crate::GameState;


pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Lobby), setup_lobby_ui)
            .add_systems(OnExit(GameState::Lobby), cleanup_lobby);
    }
}

// System to setup the lobby UI
fn setup_lobby_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/Debrosee-ALPnL.ttf");

    // Setup basic lobby UI with a scrollable list
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                position_type: PositionType::Relative,
                overflow: Overflow {
                    x: OverflowAxis::Visible,  // Set overflow behavior for the x-axis
                    y: OverflowAxis::Visible,  // Set overflow behavior for the y-axis
                },
                direction: Direction::Inherit, // Inherit the direction from the parent (default)
                left: Val::Auto,  // Use 'Auto' if positioning is flexible
                right: Val::Auto, // You can set these for absolute positioning if needed
                top: Val::Auto,
                bottom: Val::Auto,
                justify_content: JustifyContent::Center, // Centers the contents in the container
                align_items: AlignItems::Center,         // Aligns items along the cross axis (center in this case)
                flex_direction: FlexDirection::ColumnReverse, // Lays out the children from bottom to top
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            for i in 0..5 {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            display: Display::Flex,
                            position_type: PositionType::Relative,
                            overflow: Overflow {
                                x: OverflowAxis::Visible,  // Set overflow behavior for the x-axis
                                y: OverflowAxis::Visible,  // Set overflow behavior for the y-axis
                            },
                            direction: Direction::Inherit,
                            width: Val::Percent(100.0),   // 100% width for each button
                            height: Val::Px(50.0),        // Fixed height of 50px for each button
                            margin: UiRect::all(Val::Px(5.0)), // Margin around each button
                            justify_content: JustifyContent::Center, // Centers text inside the button
                            align_items: AlignItems::Center,         // Centers text vertically
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Lobby {}", i + 1),
                            TextStyle {
                                font: font.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
            }
        });
}

// System to cleanup the lobby UI when exiting the Lobby state
fn cleanup_lobby(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}