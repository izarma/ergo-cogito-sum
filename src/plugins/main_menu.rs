use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct MainMenuPlugin;

// Component for marking buttons (Host or Join)

#[derive(Component)]
struct HostButton;

#[derive(Component)]
struct JoinButton;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main_menu);
        //.add_systems(Update, button_interaction_system)
        //.add_systems(OnExit, cleanup_menu);
    }
}

// System to setup the main menu UI
fn setup_main_menu(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let window_center_x = window.width() / 2.0;
    let window_center_y = window.height() / 2.0;
    // UI setup with Host and Join buttons
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                // Host Button
                .spawn(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
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
                        ..Default::default()
                    },
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

// // System to handle button interaction
// fn button_interaction_system(
//     mut state: ResMut<State<GameState>>,
//     mut interaction_query: Query<
//         (&Interaction, &mut UiColor, Option<&HostButton>, Option<&JoinButton>),
//         (Changed<Interaction>, With<Button>),
//     >,
// ) {
//     for (interaction, mut color, host_button, join_button) in interaction_query.iter_mut() {
//         match *interaction {
//             Interaction::Clicked => {
//                 if host_button.is_some() {
//                     println!("Host Game Button Clicked");
//                     state.set(GameState::Lobby).unwrap(); // Switch to Lobby state
//                 } else if join_button.is_some() {
//                     println!("Join Game Button Clicked");
//                     state.set(GameState::Lobby).unwrap(); // Switch to Lobby state
//                 }
//             }
//             Interaction::Hovered => {
//                 *color = Color::rgb(0.8, 0.8, 0.8).into();
//             }
//             Interaction::None => {
//                 *color = Color::rgb(0.25, 0.25, 0.25).into();
//             }
//         }
//     }
// }

// // System to cleanup menu when exiting MainMenu state
// fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<Node>>) {
//     for entity in query.iter() {
//         commands.entity(entity).despawn_recursive();
//     }
// }
