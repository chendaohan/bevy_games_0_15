mod introduction;
mod passed;
mod playing;

use bevy::prelude::*;

use crate::AppState;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_plugins((introduction::plugin, playing::plugin, passed::plugin))
        .add_systems(OnEnter(AppState::Game), insert_scene_index_resource)
        .add_systems(OnExit(AppState::Game), remove_scene_index_resource);
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, SubStates)]
#[source(AppState = AppState::Game)]
pub enum GameState {
    #[default]
    Introduction,
    Playing,
    Passed,
    Menu,
}

#[derive(Resource, Default)]
pub struct SceneIndex(pub usize);

fn insert_scene_index_resource(mut commands: Commands) {
    commands.init_resource::<SceneIndex>();
}

fn remove_scene_index_resource(mut commands: Commands) {
    commands.remove_resource::<SceneIndex>();
}
