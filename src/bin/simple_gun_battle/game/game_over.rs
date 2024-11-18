use bevy::prelude::*;

use crate::{ui_utils::Widgets, AppState};

use super::{spawn::DespawnScenePlayer, GameState, SceneIndex};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), spawn_game_over)
        .add_systems(
            Update,
            pressed_game_over.run_if(in_state(GameState::GameOver)),
        );
}

#[derive(Component)]
enum GameOverButton {
    Restart,
    ReturnStartMenu,
}

fn spawn_game_over(mut commands: Commands) {
    commands
        .column(
            Val::Percent(10.),
            Color::BLACK.with_alpha(0.5),
            GameState::GameOver,
        )
        .with_children(|parent| {
            parent.title("游戏结束！");
            parent.button("重新开始", GameOverButton::Restart);
            parent.button("开始菜单", GameOverButton::ReturnStartMenu);
        });
}

fn pressed_game_over(
    mut commands: Commands,
    buttons: Query<(&Interaction, &GameOverButton), Changed<Interaction>>,
    mut game_next_state: ResMut<NextState<GameState>>,
    mut app_next_state: ResMut<NextState<AppState>>,
    mut scene_index: ResMut<SceneIndex>,
) {
    for (interaction, game_over_button) in &buttons {
        if let Interaction::Pressed = interaction {
            commands.trigger(DespawnScenePlayer);
            match game_over_button {
                GameOverButton::Restart => {
                    scene_index.0 = 0;
                    game_next_state.set(GameState::Spawn);
                }
                GameOverButton::ReturnStartMenu => {
                    app_next_state.set(AppState::StartMenu);
                }
            }
        }
    }
}
