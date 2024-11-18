use bevy::prelude::*;

use crate::{ui_utils::Widgets, AppState};

use super::{spawn::DespawnScenePlayer, GameState};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(
            Update,
            pressed_menu_button.run_if(in_state(GameState::Menu)),
        );
}

#[derive(Component)]
enum MenuButton {
    ReturnGame,
    ReturnStartMenu,
}

fn spawn_menu(mut commands: Commands) {
    commands
        .column(
            Val::Percent(10.),
            Color::BLACK.with_alpha(0.5),
            GameState::Menu,
        )
        .with_children(|parent| {
            parent.title("菜单！");
            parent.button("返回游戏", MenuButton::ReturnGame);
            parent.button("开始菜单", MenuButton::ReturnStartMenu);
        });
}

fn pressed_menu_button(
    mut commands: Commands,
    buttons: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut game_next_state: ResMut<NextState<GameState>>,
    mut app_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button) in &buttons {
        if let Interaction::Pressed = interaction {
            match menu_button {
                MenuButton::ReturnGame => {
                    game_next_state.set(GameState::Play);
                }
                MenuButton::ReturnStartMenu => {
                    commands.trigger(DespawnScenePlayer);
                    app_next_state.set(AppState::StartMenu);
                }
            }
        }
    }
}
