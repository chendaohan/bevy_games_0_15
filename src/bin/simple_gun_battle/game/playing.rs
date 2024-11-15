use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind, input::mouse::MouseMotion, prelude::*,
    render::camera::CameraUpdateSystem, window::PrimaryWindow,
};

use crate::{ui_utils::UiCamera, DefaultFont};

use super::{GameState, SceneIndex};

pub const GUN_PATH: &str = "gun.glb";
const SCENE_PATHS: [&str; 2] = ["scene_0.glb", "scene_1.glb"];

const MOVEMENT_SPEED: f32 = 14.;
const JUMP_SPEED: f32 = 17.;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Playing),
        (
            spawn_scene,
            spawn_remaining_enemy_text,
            disable_ui_camera,
            spawn_player,
            hide_cursor,
            spawn_front_sight,
        ),
    )
    .add_systems(
        Update,
        (
            playing_to_passed,
            update_remaining_enemy_text,
            spawn_scene_enemy_rigid_bodies,
            enable_shadows,
            gunfire.never_param_warn(),
            move_and_jump,
            locked_cursor,
            rotate,
        )
            .run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        PostUpdate,
        attack_enemy
            .never_param_warn()
            .run_if(in_state(GameState::Playing))
            .after(CameraUpdateSystem)
            .after(TransformSystem::TransformPropagate),
    );
}

#[derive(Component)]
pub struct GameScene;

#[derive(Component)]
pub struct RemainingEnemyText;

#[derive(Component)]
pub struct PlayerRigidBody;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct Enemy(pub usize);

fn spawn_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scene_index: Res<SceneIndex>,
) {
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(SCENE_PATHS[scene_index.0])),
        ),
        GameScene,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
    ));
}

fn spawn_remaining_enemy_text(mut commands: Commands, default_font: Res<DefaultFont>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                ..default()
            },
            StateScoped(GameState::Playing),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(25.),
                        height: Val::Percent(10.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::BLACK.with_alpha(0.5)),
                ))
                .with_children(|parent| {
                    let text_font = TextFont {
                        font: default_font.0.clone(),
                        font_size: 60.,
                        ..default()
                    };
                    parent
                        .spawn((
                            Text::new("剩余敌人："),
                            text_font.clone(),
                            TextColor(tailwind::BLUE_600.into()),
                        ))
                        .with_child((
                            TextSpan::new(""),
                            text_font,
                            TextColor(tailwind::RED_600.into()),
                            RemainingEnemyText,
                        ));
                });
        });
}

fn update_remaining_enemy_text(
    mut text: Single<&mut TextSpan, With<RemainingEnemyText>>,
    remaining_enemy: Query<(), With<Enemy>>,
) {
    text.0 = remaining_enemy.iter().count().to_string();
}

fn playing_to_passed(
    mut is_start: Local<bool>,
    enemies: Query<(), With<Enemy>>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if enemies.is_empty() {
        if *is_start {
            window.cursor_options.visible = true;
            next_state.set(GameState::Passed);
            *is_start = false;
        }
    } else {
        *is_start = true;
    }
}

fn disable_ui_camera(mut ui_camera: Single<&mut Camera, With<UiCamera>>) {
    ui_camera.is_active = false;
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
            PlayerRigidBody,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GUN_PATH))),
                    Visibility::Inherited,
                    Player,
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

fn hide_cursor(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    window.cursor_options.visible = false;
}

fn spawn_front_sight(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            StateScoped(GameState::Playing),
        ))
        .with_child((
            UiImage::new(asset_server.load("front_sight.png")).with_color(Color::BLACK),
            Node {
                width: Val::Px(45.),
                height: Val::Px(45.),
                ..default()
            },
        ));
}

fn spawn_scene_enemy_rigid_bodies(
    mut commands: Commands,
    rigid_bodies: Query<(Entity, &Name), (Added<Collider>, Without<RigidBody>, With<Mesh3d>)>,
) {
    for (entity, name) in &rigid_bodies {
        if name.as_str() == "enemy_mesh" {
            commands.entity(entity).insert((
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Enemy(2),
            ));
        } else {
            commands.entity(entity).insert(RigidBody::Static);
        }
    }
}

fn enable_shadows(mut light: Single<&mut DirectionalLight, Added<DirectionalLight>>) {
    light.shadows_enabled = true;
}

fn attack_enemy(
    mut commands: Commands,
    spatial_query: SpatialQuery,
    camera: Single<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut enemies: Query<&mut Enemy>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let (camera, transform) = *camera;
        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(ray3d) = camera.viewport_to_world(transform, cursor_position) {
                let mut hits = spatial_query.ray_hits(
                    ray3d.origin,
                    ray3d.direction,
                    200.,
                    10,
                    true,
                    &SpatialQueryFilter::default(),
                );
                hits.sort_by(|data1, data2| {
                    data1
                        .time_of_impact
                        .partial_cmp(&data2.time_of_impact)
                        .unwrap()
                });
                if let Some(hit_data) = hits.get(1) {
                    if let Ok(mut enemy) = enemies.get_mut(hit_data.entity) {
                        if enemy.0 >= 2 {
                            enemy.0 -= 1;
                        } else {
                            commands.entity(hits[1].entity).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}

fn gunfire(
    mut commands: Commands,
    player: Single<Entity, With<Player>>,
    keyboard: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if keyboard.just_pressed(MouseButton::Left) {
        commands.entity(*player).with_child((
            Transform::from_xyz(0., 0., 1.5),
            AudioPlayer::<AudioSource>(asset_server.load("gunfire.mp3")),
            PlaybackSettings::DESPAWN.with_spatial(true),
        ));
    }
}

fn move_and_jump(
    player: Single<(Entity, &Transform, &mut LinearVelocity), With<PlayerRigidBody>>,
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
        if let Some(normal) = normal {
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
}

fn rotate(
    mut player_rigid_body: Single<&mut AngularVelocity, (With<PlayerRigidBody>, Without<Player>)>,
    mut player: Single<&mut Transform, (With<Player>, Without<PlayerRigidBody>)>,
    mut mouse_motion_reader: EventReader<MouseMotion>,
) {
    if mouse_motion_reader.is_empty() {
        player_rigid_body.0.y = 0.;
    }

    for MouseMotion { delta } in mouse_motion_reader.read() {
        player_rigid_body.0.y = -delta.x / 5.;
        let euler_rotation = player.rotation.to_euler(EulerRot::XYZ);
        if euler_rotation.0.abs() < 45.0_f32.to_radians() {
            player.rotate_local_x(delta.y / 600.);
        } else {
            let x = euler_rotation
                .0
                .clamp(-44.5_f32.to_radians(), 44.5_f32.to_radians());
            player.rotation =
                Quat::from_euler(EulerRot::XYZ, x, euler_rotation.1, euler_rotation.2);
        }
    }
}

fn locked_cursor(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    let cursor_position = Vec2::new(window.width() / 2., window.height() / 2.);
    window.set_cursor_position(Some(cursor_position));
}
