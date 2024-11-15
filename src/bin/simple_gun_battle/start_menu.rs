use bevy::prelude::*;

use crate::{ui_utils::spawn_button, AppState, DefaultFont};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::StartMenu), spawn_start_menu)
        .add_systems(
            Update,
            (
                pressed_hunt.never_param_warn(),
                pressed_exit_app.never_param_warn(),
            )
                .run_if(in_state(AppState::StartMenu)),
        );
}

#[derive(Component)]
struct EnterGameButton;

#[derive(Component)]
struct ExitAppButton;

fn spawn_start_menu(mut commands: Commands, default_font: Res<DefaultFont>) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                row_gap: Val::Px(40.),
                ..default()
            },
            StateScoped(AppState::StartMenu),
        ))
        .with_children(|parent| {
            spawn_button(parent, "进入游戏", EnterGameButton, default_font.0.clone());
            spawn_button(parent, "退出游戏", ExitAppButton, default_font.0.clone());
        });
}

fn pressed_hunt(
    button: Single<&Interaction, (Changed<Interaction>, With<EnterGameButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Interaction::Pressed = *button {
        next_state.set(AppState::Game);
    }
}

fn pressed_exit_app(
    button: Single<&Interaction, (Changed<Interaction>, With<ExitAppButton>)>,
    mut exit_writer: EventWriter<AppExit>,
) {
    if let Interaction::Pressed = *button {
        exit_writer.send_default();
    }
}
