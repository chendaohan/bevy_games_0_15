use bevy::prelude::*;

use crate::{
    ui_utils::Widgets,
    AppState,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::StartMenu), spawn_start_menu)
        .add_systems(Update, pressed_start_menu.run_if(in_state(AppState::StartMenu)));
}

#[derive(Component)]
enum StartMenuButton {
    StartGame,
    ExitGame,
}

fn spawn_start_menu(mut commands: Commands) {
    commands
        .column(Val::Percent(10.), Color::NONE, AppState::StartMenu)
        .with_children(|parent| {
            parent.title("简单枪战！");
            parent.button("开始游戏", StartMenuButton::StartGame);
            parent.button("退出游戏", StartMenuButton::ExitGame);
        });
}

fn pressed_start_menu(
    buttons: Query<(&Interaction, &StartMenuButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit_writer: EventWriter<AppExit>,
) {
    for (interaction, start_menu_button) in &buttons {
        if let Interaction::Pressed = interaction {
            match start_menu_button {
                StartMenuButton::StartGame => {
                    next_state.set(AppState::Game);
                }
                StartMenuButton::ExitGame => {
                    exit_writer.send_default();
                }
            }
        }
    }
}
