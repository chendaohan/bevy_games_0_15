mod framepace;
mod game;
mod start_menu;
mod ui_utils;

use avian3d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::WindowMode};
use framepace::FramepacePlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "assets/simple_gun_battle".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "简单枪战".into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((PhysicsPlugins::default(), FramepacePlugin))
        .add_plugins((ui_utils::plugin, start_menu::plugin, game::plugin))
        // .add_plugins(PhysicsDebugPlugin::default())
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .add_systems(Startup, state_to_start_menu)
        .add_systems(Update, exit_app.run_if(input_just_pressed(KeyCode::Escape)))
        .run()
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, States)]
enum AppState {
    #[default]
    None,
    StartMenu,
    Game,
}

fn state_to_start_menu(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::StartMenu);
}

fn exit_app(mut exit_writer: EventWriter<AppExit>) {
    exit_writer.send_default();
}
