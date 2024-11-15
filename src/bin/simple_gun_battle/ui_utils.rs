use bevy::{color::palettes::tailwind, prelude::*};

const PRESSED: Color = Color::Srgba(tailwind::RED_600);
const HOVERED: Color = Color::Srgba(tailwind::PURPLE_300);
const NONE: Color = Color::Srgba(tailwind::ORANGE_500);

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui_camera)
        .add_systems(Update, change_button_background);
}

#[derive(Component)]
pub struct UiCamera;

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((Camera2d, UiCamera));
}

fn change_button_background(
    mut buttons: Populated<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background) in buttons.iter_mut() {
        match interaction {
            Interaction::Pressed => background.0 = PRESSED,
            Interaction::Hovered => background.0 = HOVERED,
            Interaction::None => background.0 = NONE,
        }
    }
}

pub fn spawn_button(
    parent: &mut ChildBuilder,
    text: &str,
    marker: impl Component,
    font: Handle<Font>,
) {
    parent
        .spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(350.),
                height: Val::Px(120.),
                border: UiRect::all(Val::Px(8.)),
                ..default()
            },
            BackgroundColor(NONE),
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Percent(50.)),
            marker,
        ))
        .with_child((
            Text::new(text),
            TextFont {
                font,
                font_size: 65.,
                ..default()
            },
        ));
}
