use avian3d::prelude::*;
use bevy::{
    prelude::*,
    scene::{SceneInstance, SceneInstanceReady},
};

use crate::game::{GameState, SceneIndex};

pub const SCENE_PATHS: [&str; 2] = ["scene_0.glb", "scene_1.glb"];

pub fn plugin(app: &mut App) {
    app.add_observer(despawn_scene_player)
        .add_observer(spawn_to_play)
        .add_systems(OnEnter(GameState::Spawn), (spawn_scene, spawn_player));
}

#[derive(Component)]
pub struct GameScene;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerModel;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Event)]
pub struct DespawnScenePlayer;

fn spawn_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scene_index: Res<SceneIndex>,
) {
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(SCENE_PATHS[scene_index.0])),
        ),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        GameScene,
    ));
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Transform::from_xyz(0., 1.5, 0.),
            Visibility::Visible,
            RigidBody::Dynamic,
            Collider::capsule(0.5, 1.5),
            LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            LinearDamping(1.),
            Restitution::new(0.),
            Player,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("gun.glb"))),
                    PlayerModel,
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Camera3d::default(),
                            Camera {
                                order: 1,
                                ..default()
                            },
                            Transform::from_xyz(0.2, 0.2, -0.3).looking_to(Vec3::Z, Vec3::Y),
                            PlayerCamera,
                        ))
                        .with_child((Transform::default(), SpatialListener::new(0.15)));
                });
        });
}

fn spawn_to_play(
    trigger: Trigger<SceneInstanceReady>,
    game_scene: Single<&SceneInstance, With<GameScene>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if trigger.instance_id == ***game_scene {
        next_state.set(GameState::Play);
    }
}

fn despawn_scene_player(
    _trigger: Trigger<DespawnScenePlayer>,
    mut commands: Commands,
    game_scene: Single<Entity, With<GameScene>>,
    player: Single<Entity, With<Player>>,
) {
    commands.entity(*game_scene).despawn_recursive();
    commands.entity(*player).despawn_recursive();
}
