mod game_over;
mod menu;
mod next_level;
mod play;
mod spawn;

use bevy::prelude::*;

use crate::{ui_utils::UiCamera, AppState};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        spawn::plugin,
        play::plugin,
        menu::plugin,
        next_level::plugin,
        game_over::plugin,
    ))
    .add_sub_state::<GameState>()
    .enable_state_scoped_entities::<GameState>()
    .add_systems(
        OnEnter(AppState::Game),
        (insert_scene_index, insert_enemy_count, disable_ui_camera),
    )
    .add_systems(
        OnExit(AppState::Game),
        (remove_scene_index, remove_enemy_count, enable_ui_camera),
    );
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, SubStates)]
#[source(AppState = AppState::Game)]
enum GameState {
    #[default]
    Spawn,
    Play,
    Menu,
    NextLevel,
    GameOver,
}

#[derive(Resource, Default)]
struct SceneIndex(usize);

#[derive(Resource, Default)]
struct EnemyCount(usize);

fn insert_scene_index(mut commands: Commands) {
    commands.init_resource::<SceneIndex>();
}

fn remove_scene_index(mut commands: Commands) {
    commands.remove_resource::<SceneIndex>();
}

fn insert_enemy_count(mut commands: Commands) {
    commands.init_resource::<EnemyCount>();
}

fn remove_enemy_count(mut commands: Commands) {
    commands.remove_resource::<EnemyCount>();
}

fn disable_ui_camera(mut ui_camera: Single<&mut Camera, With<UiCamera>>) {
    ui_camera.is_active = false;
}

fn enable_ui_camera(mut ui_camera: Single<&mut Camera, With<UiCamera>>) {
    ui_camera.is_active = true;
}
