use core::f32;

use avian3d::prelude::*;
use bevy::{color::palettes::css, input::mouse::MouseMotion, prelude::*};
use rand::{thread_rng, Rng};

use crate::game::{
    spawn::{Gun, GunFlame, Player, PlayerCamera},
    GameState,
};

use super::{Enemy, HealthPoints, PassLevelDetection, UpdateEnemyCount, UpdateHealthBar};

const MOVEMENT_SPEED: f32 = 14.;
const JUMP_SPEED: f32 = 17.;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            move_and_jump,
            rotate,
            spawn_gunfire,
            spawn_bullet,
            detect_bullet_collision,
            bullet_beyond_the_limit,
            spawn_gun_flame,
        )
            .run_if(in_state(GameState::Play)),
    );
}

#[derive(Component)]
struct Bullet;

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
    mut gun: Single<&mut Transform, With<Gun>>,
    mut mouse_reader: EventReader<MouseMotion>,
) {
    if mouse_reader.is_empty() {
        player.0.y = 0.;
    }

    for MouseMotion { delta } in mouse_reader.read() {
        player.0.y = -delta.x / 5.;
        let euler_rotation = gun.rotation.to_euler(EulerRot::XYZ);
        if euler_rotation.0.abs() < 45.0_f32.to_radians() {
            gun.rotate_local_x(delta.y / 600.);
            continue;
        }
        let x = euler_rotation
            .0
            .clamp(-44.9_f32.to_radians(), 44.9_f32.to_radians());
        gun.rotation = Quat::from_euler(EulerRot::XYZ, x, euler_rotation.1, euler_rotation.2);
    }
}

fn spawn_gunfire(
    mut commands: Commands,
    gun: Single<Entity, With<Gun>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        commands.entity(*gun).with_child((
            AudioPlayer::<AudioSource>(asset_server.load("gunfire.mp3")),
            PlaybackSettings::DESPAWN.with_spatial(true),
            Transform::from_xyz(0., 0., 1.5),
        ));
    }
}

fn spawn_bullet(
    mut bullet_mesh: Local<Handle<Mesh>>,
    mut bullet_material: Local<Handle<StandardMaterial>>,
    mut commands: Commands,
    player_camera: Single<&GlobalTransform, With<PlayerCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        if *bullet_mesh == Handle::<Mesh>::default() {
            *bullet_mesh = meshes.add(Capsule3d::new(0.03, 0.1));
        }
        if *bullet_material == Handle::<StandardMaterial>::default() {
            *bullet_material = materials.add(StandardMaterial {
                emissive: Color::Srgba(css::ORANGE_RED).to_linear().with_luminance(3.),
                ..default()
            });
        }

        let mut bullet_transform = player_camera.compute_transform();
        bullet_transform.rotate_local_x(-f32::consts::PI / 2.);
        bullet_transform.translation += bullet_transform.up() * 1.5;
        commands.spawn((
            Mesh3d(bullet_mesh.clone()),
            MeshMaterial3d(bullet_material.clone()),
            bullet_transform,
            RigidBody::Kinematic,
            LinearVelocity(bullet_transform.up() * 70.),
            Collider::capsule(0.03, 0.1),
            SweptCcd::LINEAR,
            Sensor,
            Bullet,
        ));
    }
}

fn detect_bullet_collision(
    mut commands: Commands,
    bullets: Query<(), With<Bullet>>,
    mut enemies: Query<&mut HealthPoints, With<Enemy>>,
    mut collision_reader: EventReader<Collision>,
) {
    for Collision(contacts) in collision_reader.read() {
        let (bullet, hit_object) = if bullets.contains(contacts.entity1) {
            (Some(contacts.entity1), Some(contacts.entity2))
        } else if bullets.contains(contacts.entity2) {
            (Some(contacts.entity2), Some(contacts.entity1))
        } else {
            (None, None)
        };
        let Some(bullet) = bullet else {
            continue;
        };
        commands.entity(bullet).try_despawn();
        let Some(hit_object) = hit_object else {
            continue;
        };
        let Ok(mut health_points) = enemies.get_mut(hit_object) else {
            return;
        };
        health_points.current -= thread_rng().gen_range(12.0..25.0);
        if health_points.current <= 0.0 {
            commands.entity(hit_object).try_despawn_recursive();
        }
        commands.trigger(UpdateEnemyCount);
        commands.trigger(PassLevelDetection);
        commands.trigger(UpdateHealthBar(hit_object));
    }
}

fn bullet_beyond_the_limit(
    mut commands: Commands,
    bullets: Query<(Entity, &Position), With<Bullet>>,
) {
    for (entity, position) in &bullets {
        if position.length() > 300. {
            commands.entity(entity).try_despawn();
        }
    }
}

fn spawn_gun_flame(
    mut gun_flame: Single<(&MeshMaterial3d<StandardMaterial>, &mut GunFlame)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let material = gun_flame.0;
    let gun_flame = &mut gun_flame.1;
    if !gun_flame.0.tick(time.delta()).finished() {
        if let Some(material) = materials.get_mut(&material.0) {
            material
                .base_color
                .set_alpha(gun_flame.0.fraction_remaining());
        }
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        gun_flame.0.reset();
    }
}
