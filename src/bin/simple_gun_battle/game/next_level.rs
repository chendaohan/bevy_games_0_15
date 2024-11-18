use bevy::prelude::*;

use crate::{ui_utils::Widgets, AppState};

use super::{spawn::DespawnScenePlayer, GameState, SceneIndex};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::NextLevel), spawn_next_level)
        .add_systems(
            Update,
            pressed_next_level_button.run_if(in_state(GameState::NextLevel)),
        );
}

#[derive(Component)]
enum NextLevelButton {
    NextLevel,
    ReturnStartMenu,
}

fn spawn_next_level(mut commands: Commands) {
    commands
        .column(
            Val::Percent(10.),
            Color::BLACK.with_alpha(0.5),
            GameState::NextLevel,
        )
        .with_children(|parent| {
            parent.title("结算！");
            parent.button("下一关", NextLevelButton::NextLevel);
            parent.button("开始菜单", NextLevelButton::ReturnStartMenu);
        });
}

fn pressed_next_level_button(
    mut commands: Commands,
    buttons: Query<(&Interaction, &NextLevelButton), Changed<Interaction>>,
    mut game_next_state: ResMut<NextState<GameState>>,
    mut app_next_state: ResMut<NextState<AppState>>,
    mut scene_index: ResMut<SceneIndex>,
) {
    for (interaction, next_level_button) in &buttons {
        if let Interaction::Pressed = interaction {
            commands.trigger(DespawnScenePlayer);
            match next_level_button {
                NextLevelButton::NextLevel => {
                    scene_index.0 += 1;
                    game_next_state.set(GameState::Spawn);
                }
                NextLevelButton::ReturnStartMenu => {
                    app_next_state.set(AppState::StartMenu);
                }
            }
        }
    }
}
