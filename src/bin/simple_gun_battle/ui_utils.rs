use bevy::{asset::load_internal_binary_asset, color::palettes::tailwind, prelude::*};
use uuid::uuid;

const PRESSED: Color = Color::Srgba(tailwind::RED_600);
const HOVERED: Color = Color::Srgba(tailwind::PURPLE_300);
const NONE: Color = Color::Srgba(tailwind::ORANGE_500);

pub const DEFAULT_FONT: Handle<Font> = Handle::Weak(AssetId::Uuid {
    uuid: uuid!("0efa080a-3128-4329-9cd1-76f1e116e824"),
});

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui_camera)
        .add_systems(Update, change_button_background);

    load_internal_binary_asset!(
        app,
        DEFAULT_FONT,
        "NotoSansSC-Bold.ttf",
        |bytes: &[u8], _path: String| Font::try_from_bytes(bytes.to_vec()).unwrap()
    );
}

pub trait SpawnUi: Send + Sync {
    fn spawn_ui(&mut self, bundle: impl Bundle) -> EntityCommands;
}

pub trait Widgets: SpawnUi {
    fn column(
        &mut self,
        row_gap: Val,
        background: impl Into<Color>,
        state_scoped: impl States,
    ) -> EntityCommands;

    fn title(&mut self, text: impl Into<String>) -> EntityCommands;

    fn button(&mut self, text: impl Into<String>, marker: impl Component) -> EntityCommands;
}

impl SpawnUi for Commands<'_, '_> {
    fn spawn_ui(&mut self, bundle: impl Bundle) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl SpawnUi for ChildBuilder<'_> {
    fn spawn_ui(&mut self, bundle: impl Bundle) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl<T: SpawnUi> Widgets for T {
    fn column(
        &mut self,
        row_gap: Val,
        background: impl Into<Color>,
        state_scoped: impl States,
    ) -> EntityCommands {
        self.spawn_ui((
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap,
                ..default()
            },
            BackgroundColor(background.into()),
            StateScoped(state_scoped),
        ))
    }

    fn title(&mut self, text: impl Into<String>) -> EntityCommands {
        self.spawn_ui((
            Text(text.into()),
            TextFont {
                font: DEFAULT_FONT,
                font_size: 140.,
                ..default()
            },
        ))
    }

    fn button(&mut self, text: impl Into<String>, marker: impl Component) -> EntityCommands {
        let mut entity_commands = self.spawn_ui((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(350.),
                height: Val::Px(120.),
                border: UiRect::all(Val::Px(8.)),
                ..default()
            },
            BorderRadius::all(Val::Percent(50.)),
            BorderColor(Color::BLACK),
            marker,
        ));
        entity_commands.with_child((
            Text::new(text),
            TextFont {
                font: DEFAULT_FONT,
                font_size: 65.,
                ..default()
            },
        ));
        entity_commands
    }
}

#[derive(Component, Clone, Copy)]
#[require(Camera2d)]
pub struct UiCamera;

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn(UiCamera);
}

fn change_button_background(mut buttons: Query<(&Interaction, &mut BackgroundColor)>) {
    for (interaction, mut background_color) in &mut buttons {
        match interaction {
            Interaction::None => background_color.0 = NONE,
            Interaction::Hovered => background_color.0 = HOVERED,
            Interaction::Pressed => background_color.0 = PRESSED,
        }
    }
}
