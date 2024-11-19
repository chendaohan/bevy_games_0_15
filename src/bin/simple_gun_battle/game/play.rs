mod action;
mod spawn;

use core::f32;

use avian3d::prelude::*;
use bevy::{pbr::NotShadowCaster, prelude::*, window::PrimaryWindow};
use rand::{thread_rng, Rng};
use spawn::EnemyCountText;

use super::{
    spawn::{Player, SCENE_PATHS},
    EnemyCount, GameState, HealthBarMaterial, SceneIndex,
};

pub fn plugin(app: &mut App) {
    app.add_plugins((spawn::plugin, action::plugin))
        .add_observer(update_enemy_count)
        .add_observer(pass_level)
        .add_observer(update_health_bar)
        .add_systems(
            OnEnter(GameState::Play),
            (hide_cursor, enter_update_enemy_count),
        )
        .add_systems(OnExit(GameState::Play), show_cursor)
        .add_systems(
            Update,
            (
                lock_cursor,
                update_enemy_count_text,
                play_to_menu,
                health_bar_align_player_camera.never_param_warn(),
            )
                .run_if(in_state(GameState::Play)),
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
struct Enemy;

#[derive(Component)]
struct HealthPoints {
    maximum: f32,
    current: f32,
}

#[derive(Component)]
struct HealthBar;

#[derive(Event)]
struct PassLevelDetection;

#[derive(Event)]
struct UpdateEnemyCount;

#[derive(Event)]
struct UpdateHealthBar(Entity);

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
    _trigger: Trigger<PassLevelDetection>,
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
    mut rectangle: Local<Handle<Mesh>>,
    mut commands: Commands,
    rigid_bodies: Populated<(Entity, &Name), Added<Collider>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<HealthBarMaterial>>,
) {
    if *rectangle == Handle::default() {
        *rectangle = meshes.add(
            Rectangle::new(1.5, 0.15)
                .mesh()
                .build()
                .rotated_by(Quat::from_rotation_y(f32::consts::PI)),
        );
    }

    let mut thread_rng = thread_rng();
    for (entity, name) in rigid_bodies.iter() {
        if name.as_str() == "enemy_mesh" {
            let health_points = thread_rng.gen_range(27.0..53.0);
            commands
                .entity(entity)
                .insert((
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED,
                    Enemy,
                    HealthPoints {
                        maximum: health_points,
                        current: health_points,
                    },
                ))
                .with_child((
                    Mesh3d(rectangle.clone()),
                    MeshMaterial3d(materials.add(HealthBarMaterial { ratio: 1. })),
                    NotShadowCaster,
                    Transform::from_xyz(0., 1.25, 0.),
                    HealthBar,
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
    _trigger: Trigger<UpdateEnemyCount>,
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

fn health_bar_align_player_camera(
    mut health_bars: Query<(&mut Transform, &GlobalTransform), With<HealthBar>>,
    player: Single<&GlobalTransform, With<Player>>,
) {
    for (mut health_transform, health_global_transform) in &mut health_bars {
        let camera_translation = health_transform.transform_point(
            health_global_transform
                .affine()
                .inverse()
                .transform_point3(player.translation()),
        );
        health_transform.look_at(camera_translation, Vec3::Y);
    }
}

fn update_health_bar(
    trigger: Trigger<UpdateHealthBar>,
    enemies: Query<(&Children, &HealthPoints), With<Enemy>>,
    health_bars: Query<&MeshMaterial3d<HealthBarMaterial>, With<HealthBar>>,
    mut materials: ResMut<Assets<HealthBarMaterial>>,
) {
    let Ok((children, health_points)) = enemies.get(trigger.0) else {
        return;
    };
    let Ok(material) = health_bars.get(children[0]) else {
        return;
    };
    let Some(material) = materials.get_mut(material) else {
        return;
    };
    material.ratio = health_points.current / health_points.maximum;
}
