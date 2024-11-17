use bevy::{asset::load_internal_binary_asset, color::palettes::tailwind, prelude::*};
use uuid::uuid;

const PRESSED: Color = Color::Srgba(tailwind::RED_600);
const HOVERED: Color = Color::Srgba(tailwind::PURPLE_300);
const NONE: Color = Color::Srgba(tailwind::ORANGE_500);

pub const DEFAULT_FONT: Handle<Font> = Handle::Weak(AssetId::Uuid { uuid:  uuid!("0efa080a-3128-4329-9cd1-76f1e116e824")});

pub fn plugin(app: &mut App) {
    app.add_systems(Update, change_button_background);
    
    load_internal_binary_asset!(
        app,
        DEFAULT_FONT,
        "NotoSansSC-Bold.ttf",
        |bytes: &[u8], _path: String| Font::try_from_bytes(bytes.to_vec()).unwrap()
    );
}

trait Spawn: Send + Sync {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        <Self as ChildBuild>::spawn(self, bundle)
    }
}

trait Widgets: Spawn {
    fn button(&mut self, text: impl Into<String>, marker: impl Component);

    fn column(&mut self, gap: Val, background_color: impl Into<Color>, state_scoped: impl States) -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    fn button(&mut self, text: impl Into<String>, marker: impl Component){
        self.spawn((
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
        )).with_child((
            Text::new(text),
            TextFont {
                font: DEFAULT_FONT,
                font_size: 65.,
                ..default()
            }
        ));
    }

    fn column(&mut self, gap: Val, background_color: impl Into<Color>, state_scoped: impl States) -> EntityCommands {
        self.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                row_gap: gap,
                ..default()
            },
            BackgroundColor(background_color.into()),
            StateScoped(state_scoped),
        ))
    }
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
