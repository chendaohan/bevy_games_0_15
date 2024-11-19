use avian3d::prelude::*;
use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use rand::{thread_rng, Rng};

use crate::game::{
    spawn::{Player, PlayerCamera, PlayerModel},
    GameState,
};

use super::{Enemy, HealthPoints, PassLevelDetection, UpdateEnemyCount, UpdateHealthBar};

const MOVEMENT_SPEED: f32 = 14.;
const JUMP_SPEED: f32 = 17.;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (move_and_jump, rotate, spawn_gunfire, attack_enmey).run_if(in_state(GameState::Play)),
    );
}

fn move_and_jump(
    player: Single<(Entity, &Transform, &mut LinearVelocity), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut collision_reader: EventReader<Collision>,
    time: Res<Time>,
) {
    let (entity, transform, mut linear_velocity) = player.into_inner();
    for Collision(contacts) in collision_reader.read() {
        let normal = if contacts.entity1 == entity {
            Some(contacts.manifolds[0].normal1)
        } else if contacts.entity2 == entity {
            Some(contacts.manifolds[0].normal2)
        } else {
            None
        };
        let Some(normal) = normal else {
            continue;
        };
        if normal.dot(Vec3::NEG_Y) >= 20.0_f32.to_radians().cos() {
            let speed = time.delta_secs() * MOVEMENT_SPEED;
            let mut velocity = Vec3::ZERO;
            if keyboard.pressed(KeyCode::KeyA) {
                velocity += transform.right() * speed;
            }
            if keyboard.pressed(KeyCode::KeyD) {
                velocity += transform.left() * speed;
            }
            if keyboard.pressed(KeyCode::KeyW) {
                velocity += transform.back() * speed;
            }
            if keyboard.pressed(KeyCode::KeyS) {
                velocity += transform.forward() * speed;
            }
            velocity = velocity.clamp_length_max(MOVEMENT_SPEED);
            if keyboard.just_pressed(KeyCode::Space) {
                velocity += transform.up() * JUMP_SPEED;
            }
            linear_velocity.0 += velocity;
        }
    }
}

fn rotate(
    mut player: Single<&mut AngularVelocity, With<Player>>,
    mut player_model: Single<&mut Transform, With<PlayerModel>>,
    mut mouse_reader: EventReader<MouseMotion>,
) {
    if mouse_reader.is_empty() {
        player.0.y = 0.;
    }

    for MouseMotion { delta } in mouse_reader.read() {
        player.0.y = -delta.x / 5.;
        let euler_rotation = player_model.rotation.to_euler(EulerRot::XYZ);
        if euler_rotation.0.abs() < 45.0_f32.to_radians() {
            player_model.rotate_local_x(delta.y / 600.);
            continue;
        }
        let x = euler_rotation
            .0
            .clamp(-44.9_f32.to_radians(), 44.9_f32.to_radians());
        player_model.rotation =
            Quat::from_euler(EulerRot::XYZ, x, euler_rotation.1, euler_rotation.2);
    }
}

fn spawn_gunfire(
    mut commands: Commands,
    player_model: Single<Entity, With<PlayerModel>>,
    keyboard: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if keyboard.just_pressed(MouseButton::Left) {
        commands.entity(*player_model).with_child((
            AudioPlayer::<AudioSource>(asset_server.load("gunfire.mp3")),
            PlaybackSettings::DESPAWN.with_spatial(true),
            Transform::from_xyz(0., 0., 1.5),
        ));
    }
}

fn attack_enmey(
    mut commands: Commands,
    spatial_query: SpatialQuery,
    camera: Single<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut enemies: Query<&mut HealthPoints, With<Enemy>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }
    let (camera, transform) = *camera;
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(transform, cursor_position) else {
        return;
    };
    let mut hits = spatial_query.ray_hits(
        ray.origin,
        ray.direction,
        200.,
        20,
        true,
        &SpatialQueryFilter::default(),
    );
    hits.sort_by(|data1, data2| {
        data1
            .time_of_impact
            .partial_cmp(&data2.time_of_impact)
            .unwrap()
    });
    let Some(hit_data) = hits.get(1) else {
        return;
    };
    let Ok(mut health_points) = enemies.get_mut(hit_data.entity) else {
        return;
    };
    health_points.current -= thread_rng().gen_range(12.0..25.0);
    if health_points.current <= 0.0 {
        commands.entity(hits[1].entity).despawn_recursive();
    }
    commands.trigger(UpdateEnemyCount);
    commands.trigger(PassLevelDetection);
    commands.trigger(UpdateHealthBar(hits[1].entity));
}
