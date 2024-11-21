use std::time::Duration;

use avian3d::prelude::*;
use bevy::{
    core_pipeline::bloom::Bloom, prelude::*, scene::{SceneInstance, SceneInstanceReady}
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
pub struct Gun;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct GunFlame(pub Timer);

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

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
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
                    Gun,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Mesh3d(meshes.add(Rectangle::new(0.7, 0.7))),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::WHITE.with_alpha(0.).with_luminance(2.),
                            base_color_texture: Some(asset_server.load("explosion.png")),
                            unlit: true,
                            alpha_mode: AlphaMode::AlphaToCoverage,
                            ..default()
                        })),
                        Transform::from_xyz(0., 0., 1.4)
                            .with_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
                        GunFlame({
                            let mut timer = Timer::from_seconds(0.2, TimerMode::Once);
                            timer.set_elapsed(Duration::from_secs_f32(0.2));
                            timer
                        }),
                    ));
                    parent
                        .spawn((
                            Camera3d::default(),
                            Camera {
                                order: 1,
                                hdr: true,
                                ..default()
                            },
                            Bloom::NATURAL,
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
