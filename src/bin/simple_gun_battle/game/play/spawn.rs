use bevy::{color::palettes::tailwind, prelude::*};

use crate::{
    game::GameState,
    ui_utils::{Widgets, DEFAULT_FONT},
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Play),
        (spawn_enemy_count_text, spawn_front_sight),
    );
}

#[derive(Component)]
pub struct EnemyCountText;

fn spawn_enemy_count_text(mut commands: Commands) {
    commands
        .spawn((
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            StateScoped(GameState::Play),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(25.),
                        height: Val::Percent(10.),
                        ..default()
                    },
                    BackgroundColor(Color::BLACK.with_alpha(0.5)),
                ))
                .with_children(|parent| {
                    let text_font = TextFont {
                        font: DEFAULT_FONT,
                        font_size: 60.,
                        ..default()
                    };
                    parent
                        .spawn((
                            Text::new("敌人总数："),
                            text_font.clone(),
                            TextColor(tailwind::BLUE_600.into()),
                        ))
                        .with_child((
                            TextSpan::new("0"),
                            text_font,
                            TextColor(tailwind::RED_600.into()),
                            EnemyCountText,
                        ));
                });
        });
}

fn spawn_front_sight(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .column(Val::Auto, Color::NONE, GameState::Play)
        .with_child((
            UiImage {
                image: asset_server.load("front_sight.png"),
                color: Color::BLACK,
                ..default()
            },
            Node {
                width: Val::Px(45.),
                height: Val::Px(45.),
                ..default()
            },
        ));
}
