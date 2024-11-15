use bevy::{prelude::*, text::FontSmoothing};

use crate::{
    ui_utils::{spawn_button, UiCamera},
    AppState, DefaultFont,
};

use super::{
    playing::{GameScene, PlayerRigidBody},
    GameState, SceneIndex,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Passed), spawn_passed_page)
        .add_systems(
            Update,
            (
                pressed_next_level.never_param_warn(),
                pressed_return_start_menu.never_param_warn(),
            )
                .run_if(in_state(GameState::Passed)),
        );
}

#[derive(Component)]
pub struct NextLevelButton;

#[derive(Component)]
pub struct ReturnStartMenuButton;

fn pressed_next_level(
    mut commands: Commands,
    button: Single<&Interaction, (Changed<Interaction>, With<NextLevelButton>)>,
    hunt_scene: Single<Entity, With<GameScene>>,
    player_rigid_body: Single<Entity, With<PlayerRigidBody>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut scene_index: ResMut<SceneIndex>,
) {
    if let Interaction::Pressed = *button {
        commands.entity(*hunt_scene).despawn_recursive();
        commands.entity(*player_rigid_body).despawn_recursive();
        scene_index.0 += 1;
        next_state.set(GameState::Playing);
    }
}

fn pressed_return_start_menu(
    mut commands: Commands,
    button: Single<&Interaction, (Changed<Interaction>, With<ReturnStartMenuButton>)>,
    hunt_scene: Single<Entity, With<GameScene>>,
    player_rigid_body: Single<Entity, With<PlayerRigidBody>>,
    mut ui_camera: Single<&mut Camera, With<UiCamera>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Interaction::Pressed = *button {
        commands.entity(*hunt_scene).despawn_recursive();
        commands.entity(*player_rigid_body).despawn_recursive();
        commands.remove_resource::<SceneIndex>();
        ui_camera.is_active = true;
        next_state.set(AppState::StartMenu);
    }
}

fn spawn_passed_page(
    mut commands: Commands,
    default_font: Res<DefaultFont>,
    scene_index: Res<SceneIndex>,
) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(100.),
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            StateScoped(GameState::Passed),
        ))
        .with_children(|parent| {
            let text = if scene_index.0 < 1 {
                "恭喜通关！"
            } else {
                "游戏结束！"
            };
            parent.spawn((
                Text::new(text),
                TextFont {
                    font: default_font.0.clone(),
                    font_size: 120.,
                    font_smoothing: FontSmoothing::default(),
                },
            ));

            if scene_index.0 < 1 {
                spawn_button(parent, "下一关", NextLevelButton, default_font.0.clone());
            } else {
                spawn_button(
                    parent,
                    "开始菜单",
                    ReturnStartMenuButton,
                    default_font.0.clone(),
                );
            }
        });
}
