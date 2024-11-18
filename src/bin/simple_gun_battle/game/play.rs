mod action;
mod spawn;

use avian3d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::{thread_rng, Rng};
use spawn::EnemyCountText;

use super::{spawn::SCENE_PATHS, EnemyCount, GameState, SceneIndex};

pub fn plugin(app: &mut App) {
    app.add_plugins((spawn::plugin, action::plugin))
        .add_observer(update_enemy_count)
        .add_observer(pass_level)
        .add_systems(
            OnEnter(GameState::Play),
            (hide_cursor, enter_update_enemy_count),
        )
        .add_systems(OnExit(GameState::Play), show_cursor)
        .add_systems(
            Update,
            (lock_cursor, update_enemy_count_text, play_to_menu).run_if(in_state(GameState::Play)),
        )
        .add_systems(
            Update,
            (
                spawn_scene_enemy_rigid_bodies.never_param_warn(),
                enable_shadows.never_param_warn(),
            )
                .run_if(in_state(GameState::Play)),
        );
}

#[derive(Component)]
struct Enemy(f32);

#[derive(Event)]
struct PassLevelDetection;

#[derive(Event)]
struct UpdateEnemyCount;

fn hide_cursor(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    window.cursor_options.visible = false;
}

fn show_cursor(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    window.cursor_options.visible = true;
}

fn enter_update_enemy_count(mut commands: Commands) {
    commands.trigger(UpdateEnemyCount);
}

fn lock_cursor(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    let cursor_position = Vec2::new(window.width() / 2., window.height() / 2.);
    window.set_cursor_position(Some(cursor_position));
}

fn update_enemy_count_text(
    mut enemy_count_text: Single<&mut TextSpan, With<EnemyCountText>>,
    enemy_count: Res<EnemyCount>,
) {
    if enemy_count.is_changed() {
        enemy_count_text.0 = enemy_count.0.to_string();
    }
}

fn pass_level(
    _: Trigger<PassLevelDetection>,
    mut next_state: ResMut<NextState<GameState>>,
    scene_index: Res<SceneIndex>,
    enemy_count: Res<EnemyCount>,
) {
    if enemy_count.0 > 0 {
        return;
    }
    if scene_index.0 < SCENE_PATHS.len() - 1 {
        next_state.set(GameState::NextLevel);
    } else {
        next_state.set(GameState::GameOver);
    }
}

fn spawn_scene_enemy_rigid_bodies(
    mut commands: Commands,
    rigid_bodies: Populated<(Entity, &Name), Added<Collider>>,
) {
    let mut thread_rng = thread_rng();
    for (entity, name) in rigid_bodies.iter() {
        if name.as_str() == "enemy_mesh" {
            commands.entity(entity).insert((
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Enemy(thread_rng.gen_range(12.0..45.0)),
            ));
        } else {
            commands.entity(entity).insert(RigidBody::Static);
        }
    }
    commands.trigger(UpdateEnemyCount);
}

fn enable_shadows(mut light: Single<&mut DirectionalLight, Added<DirectionalLight>>) {
    light.shadows_enabled = true;
}

fn update_enemy_count(
    _: Trigger<UpdateEnemyCount>,
    enemies: Query<(), With<Enemy>>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    enemy_count.0 = enemies.iter().count();
}

fn play_to_menu(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        next_state.set(GameState::Menu);
    }
}
