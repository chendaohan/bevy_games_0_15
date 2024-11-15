mod game;
mod start_menu;
pub mod ui_utils;

use avian3d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::WindowMode};
// use bevy_remote_inspector::RemoteInspectorPlugins;

fn main() -> AppExit {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "assets/simple_gun_battle".to_string(),
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
        .add_plugins(PhysicsPlugins::default())
        // .add_plugins((PhysicsDebugPlugin::default(), RemoteInspectorPlugins))
        .add_plugins((ui_utils::plugin, start_menu::plugin, game::plugin))
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .init_resource::<DefaultFont>()
        .add_systems(
            Update,
            pressed_esc_exit.run_if(input_just_pressed(KeyCode::Escape)),
        )
        .run()
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, States)]
pub enum AppState {
    #[default]
    StartMenu,
    Game,
}

#[derive(Resource)]
struct DefaultFont(Handle<Font>);

impl FromWorld for DefaultFont {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("NotoSansSC-Bold.ttf"))
    }
}

fn pressed_esc_exit(mut exit_writer: EventWriter<AppExit>) {
    exit_writer.send_default();
}
