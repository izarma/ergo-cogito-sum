use bevy::prelude::*;
use std::time::Duration;
use crate::GameState;
use leafwing_input_manager::prelude::*;

pub struct PlayerInGamePlugin;

#[derive(Component)]
struct SpriteAnimState {
    start_index: usize,
    end_index: usize,
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
    texture_atlas_handle: Handle<TextureAtlasLayout>,
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Walk,
    Run,
    Attack,}



impl Plugin for PlayerInGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), setup_sprite_animation)
            .add_systems(Update, animate_sprite.run_if(in_state(GameState::InGame)))
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
    //let walk_texture_handle = asset_server.load("sprites/City_men_3/Walk.png");
    //let attack_texture_handle = asset_server.load("sprites/City_men_3/Attack.png");

    let idle_frames = 6;
    let walk_frames = 10;
    let _run_frames = 10;
    let attack_frames = 4;
    let _hurt_frames = 3;
    let _dead_frames = 5;

     // Define frame sizes
    let frame_size = UVec2::new(128, 128);

    // Create TextureAtlasLayouts
    let idle_layout = TextureAtlasLayout::from_grid(frame_size, idle_frames, 1, None, None);
    let idle_layout_handle = texture_atlases.add(idle_layout);

    let walk_layout = TextureAtlasLayout::from_grid(frame_size, walk_frames, 1, None, None);
    let walk_layout_handle = texture_atlases.add(walk_layout);

    let attack_layout = TextureAtlasLayout::from_grid(frame_size, attack_frames, 1, None, None);
    let attack_layout_handle = texture_atlases.add(attack_layout);    
    
    // Store animations in a resource
    commands.insert_resource(PlayerAnimations {
        idle: Animation {
            frames: idle_frames as usize,
            texture_atlas_handle: idle_layout_handle.clone(),
        },
        walk: Animation {
            frames: walk_frames as usize,
            texture_atlas_handle: walk_layout_handle.clone(),
        },
        attack: Animation {
            frames: attack_frames as usize,
            texture_atlas_handle: attack_layout_handle.clone(),
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

fn cleanup_animation(mut commands: Commands, query: Query<Entity, With<SpriteAnimState>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}