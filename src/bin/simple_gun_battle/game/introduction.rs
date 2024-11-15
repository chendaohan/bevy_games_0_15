use bevy::prelude::*;

use crate::{ui_utils::spawn_button, DefaultFont};

use super::GameState;

const INTRODUCTION: &str = "\
游戏目标：消灭所有敌人
操作方式：ADWS 控制行走，Space 控制跳跃";

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Introduction), spawn_introduction)
        .add_systems(
            Update,
            pressed_start_game
                .never_param_warn()
                .run_if(in_state(GameState::Introduction)),
        );
}

#[derive(Component)]
struct StartGameButton;

fn spawn_introduction(mut commands: Commands, default_font: Res<DefaultFont>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(60.),
                ..default()
            },
            StateScoped(GameState::Introduction),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(INTRODUCTION),
                TextFont {
                    font: default_font.0.clone(),
                    font_size: 60.,
                    ..default()
                },
            ));

            spawn_button(parent, "开始游戏", StartGameButton, default_font.0.clone());
        });
}

fn pressed_start_game(
    button: Single<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Interaction::Pressed = *button {
        next_state.set(GameState::Playing);
    }
}
