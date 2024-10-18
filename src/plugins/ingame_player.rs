use bevy::prelude::*;
use std::time::Duration;
use crate::GameState;

pub struct PlayerInGamePlugin;

#[derive(Component)]
struct SpriteAnimState {
    start_index: usize,
    end_index: usize,
    frame_size: UVec2,
    texture_size: Vec2,
    timer: Timer,
}

#[derive(Component)]
struct Player;

#[derive(Component, PartialEq, Eq, Debug, Clone, Copy)]
enum PlayerState {
    Idle,
    Walking,
    Running,
    Attacking,
    Hurt,
    Dead,
}

#[derive(Component)]
struct PlayerInputState {
    movement_velocity: Vec2,
    speed_multiplier: f32,
}

#[derive(Bundle)]
struct PlayerBundle {
    sprite_sheet_bundle: SpriteBundle,
    marker: Player,
    state: PlayerState,
    input_state: PlayerInputState,
    anim_state: SpriteAnimState,
}

#[derive(Resource)]
struct PlayerAnimations {
    idle: Animation,
    walk: Animation,
    attack: Animation,
}

struct Animation {
    frames: usize,
    frame_size: UVec2,
    texture_size: Vec2,
    texture_handle: Handle<Image>,
}

#[derive(Event, Debug)]
enum PlayerInputs {
    Move(Vec2),
    Attack,
}

impl Plugin for PlayerInGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), setup_sprite_animation)
            .add_systems(Update, (keyboard_input,player_movement_state,animate_sprite,update_player_animation).chain().run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), cleanup_animation);
    }
}

fn setup_sprite_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load textures for each animation
    let idle_texture_handle = asset_server.load("sprites/City_men_3/Idle.png");
    let walk_texture_handle = asset_server.load("sprites/City_men_3/Walk.png");
    let attack_texture_handle = asset_server.load("sprites/City_men_3/Attack.png");

    let idle_frames = 6;
    let walk_frames = 10;
    let _run_frames = 10;
    let attack_frames = 4;
    let _hurt_frames = 3;
    let _dead_frames = 5;

     // Define frame sizes
    let frame_size = UVec2::new(128, 128);

    // Create TextureAtlasLayouts
    let idle_layout = TextureAtlasLayout::from_grid(frame_size as UVec2, idle_frames, 1, None, None);
    let idle_layout_handle = texture_atlases.add(idle_layout);

    let walk_layout = TextureAtlasLayout::from_grid(frame_size, walk_frames, 1, None, None);
    let walk_layout_handle = texture_atlases.add(walk_layout);

    let attack_layout = TextureAtlasLayout::from_grid(frame_size, attack_frames, 1, None, None);
    let attack_layout_handle = texture_atlases.add(attack_layout);

     // Define texture sizes (assuming horizontal sprite sheets)
     let idle_texture_size = Vec2::new(idle_frames as f32 * frame_size.x as f32, frame_size.y as f32);
     let walk_texture_size = Vec2::new(walk_frames as f32 * frame_size.x as f32, frame_size.y as f32);
     let attack_texture_size = Vec2::new(attack_frames as f32 * frame_size.x as f32, frame_size.y as f32);

    // Store animations in a resource
    commands.insert_resource(PlayerAnimations {
        idle: Animation {
            frames: idle_frames as usize,
            frame_size,
            texture_size: idle_texture_size,
            texture_handle: idle_texture_handle.clone(),
        },
        walk: Animation {
            frames: walk_frames as usize,
            frame_size,
            texture_size: walk_texture_size,
            texture_handle: walk_texture_handle.clone(),
        },
        attack: Animation {
            frames: attack_frames as usize,
            frame_size,
            texture_size: attack_texture_size,
            texture_handle: attack_texture_handle.clone(),
        },
    });
    

    // Spawn player entity using PlayerBundle
    commands.spawn((PlayerBundle {
        sprite_sheet_bundle: SpriteBundle {
            texture: idle_texture_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        marker: Player,
        state: PlayerState::Idle,
        input_state: PlayerInputState {
            movement_velocity: Vec2::ZERO,
            speed_multiplier: 150.0,
        },
        anim_state: SpriteAnimState {
            start_index: 0,
            end_index: idle_frames as usize - 1,
            frame_size,
            texture_size: idle_texture_size,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
    },
    TextureAtlas {
        layout: idle_layout_handle,
        index: 0,
    },
));
}


fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut SpriteAnimState)>,
) {
    for (mut sprite, mut anim_state) in query.iter_mut() {
        anim_state.timer.tick(time.delta());
        if anim_state.timer.finished() {
            sprite.index += 1;
            if sprite.index > anim_state.end_index {
                sprite.index = anim_state.start_index;
            }
        }
    }
}

fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut evw_player: EventWriter<PlayerInputs>,
) {
    let mut movement = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        movement.x += 1.0;
    }
    if movement != Vec2::ZERO {
        evw_player.send(PlayerInputs::Move(movement.normalize()));
    } else {
        evw_player.send(PlayerInputs::Move(Vec2::ZERO));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        evw_player.send(PlayerInputs::Attack);
    }
}

fn player_movement_state(
    mut evr_player: EventReader<PlayerInputs>,
    mut q_player: Query<(&mut PlayerState, &mut PlayerInputState), With<Player>>,
) {
    for ev in evr_player.read() {
        match ev {
            PlayerInputs::Move(vel) => {
                for (mut state, mut input) in q_player.iter_mut() {
                    input.movement_velocity = *vel;
                    if *state != PlayerState::Attacking {
                        if *vel == Vec2::ZERO {
                            *state = PlayerState::Idle;
                        } else {
                            *state = PlayerState::Walking;
                        }
                    }
                }
            }
            PlayerInputs::Attack => {
                for (mut state, _) in q_player.iter_mut() {
                    *state = PlayerState::Attacking;
                }
            }
        }
    }
}

fn update_player_animation(
    player_animations: Res<PlayerAnimations>,
    mut query: Query<(
        &mut Handle<Image>,
        &mut SpriteAnimState,
        &mut Sprite,
        &PlayerState,
    ), With<Player>>,
) {
    for (mut texture_handle, mut anim_state, mut sprite, state) in query.iter_mut() {
        let animation = match *state {
            PlayerState::Idle => &player_animations.idle,
            PlayerState::Walking => &player_animations.walk,
            PlayerState::Attacking => &player_animations.attack,
            _ => continue,
        };

        if *texture_handle != animation.texture_handle {
            *texture_handle = animation.texture_handle.clone();
            anim_state.frames = animation.frames;
            anim_state.frame_size = animation.frame_size;
            anim_state.texture_size = animation.texture_size;
            anim_state.current_frame = 0;
            anim_state.timer = Timer::from_seconds(0.1, TimerMode::Repeating);

            // Reset sprite rect
            sprite.rect = Some(Rect {
                min: Vec2::ZERO,
                max: animation.frame_size.as_vec2(),
            });
        }
    }
}


fn cleanup_animation(mut commands: Commands, query: Query<Entity, With<SpriteAnimState>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}