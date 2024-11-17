mod ui_utils;
mod start_menu;
mod load_assets;
mod alter_assets;
mod play_game;

use bevy::{prelude::*, window::WindowMode};
use avian3d::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "assets/simple_gun_battle".into(),
            ..default()
        }).set(WindowPlugin {
            primary_window: Some(Window {
                title: "简单枪战".into(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins((ui_utils::plugin, start_menu::plugin))
        .add_plugins(PhysicsDebugPlugin::default())
        .init_state::<AppState>()
        .run()
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, States)]
enum AppState {
    #[default]
    StartMenu,
    LoadAssets,
    AlterAssets,
    PlayGame,
}