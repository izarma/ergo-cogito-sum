use bevy::prelude::*;
use std::time::Duration;
use crate::GameState;

pub struct PlayerInGamePlugin;

#[derive(Component)]
struct SpriteAnimState {
    start_index: usize,
    end_index: usize,
    timer: Timer,
}

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
    
    let explosion_img = asset_server.load("sprites/explosion.png");
    
    // Create the TextureAtlas from the sprite sheet (8 columns, 6 rows)
    let layout = TextureAtlasLayout::from_grid(UVec2::new(67.0 as u32, 67.0 as u32), 8, 6, None, None);
    let layout_handle = texture_atlases.add(layout);

    commands.spawn((
        SpriteBundle {
            texture: explosion_img,
            ..Default::default()
        },
        TextureAtlas {
            layout: layout_handle,
            index: 0,
        },
        SpriteAnimState {
            start_index: 0,
            end_index: 47, // Assuming there are 48 frames (8 * 6)
            timer: Timer::new(Duration::from_secs_f64(1.0 / 12.0), TimerMode::Repeating),
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