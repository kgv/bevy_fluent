use self::{
    locales::{converters::menu, de, en, ru, DefaultLocale, LocaleBundle, Locales, Rotor},
    to_sentence_case::ToSentenceCase,
};
use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_fluent::prelude::*;

/// This example illustrates how to use States to control transitioning from a Menu state to an
/// InGame state.
fn main() {
    App::build()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::ERROR,
            filter: "bevy_fluent=trace".to_string(),
        })
        .insert_resource(AssetServerSettings {
            asset_folder: "examples/ui/assets".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FluentPlugin)
        .insert_resource(Locales::new(vec![de::DE, en::US, ru::BY, ru::RU]))
        .insert_resource(DefaultLocale(en::US))
        .init_resource::<ColorMaterials>()
        .init_resource::<DefaultFont>()
        .add_state(GameState::Load)
        .add_system_set(SystemSet::on_update(GameState::Load).with_system(load.system()))
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(interaction.system())
                .with_system(rotate_right.system())
                .with_system(rotate_left.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup.system()))
        .run();
}

fn load(
    mut commands: Commands,
    locale_bundle: LocaleBundle,
    fluent_server: FluentServer,
    mut game_state: ResMut<State<GameState>>,
    (localization_assets, localization_handle): (
        Res<Assets<Localization>>,
        Option<ResMut<Handle<Localization>>>,
    ),
) {
    match localization_handle {
        Some(localization_handle) => {
            if localization_assets.get(localization_handle.id).is_some() {
                game_state.set(GameState::Menu).unwrap();
            }
        }
        None => {
            let handle = fluent_server.load_pack(locale_bundle.pack(menu));
            commands.insert_resource(handle);
        }
    }
}

fn setup(
    mut commands: Commands,
    color_materials: Res<ColorMaterials>,
    default_font: Res<DefaultFont>,
    locales: Res<Locales>,
    (localization_assets, localization_handle): (
        Res<Assets<Localization>>,
        Res<Handle<Localization>>,
    ),
) {
    let localization = localization_assets.get(localization_handle.id).unwrap();
    let request = locales.locale().to_string().to_lowercase();
    let locale = localization.content(&request).unwrap().to_sentence_case();
    let choose_language = localization
        .content("choose-language")
        .unwrap()
        .to_sentence_case();
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    // ui
    let root_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            material: color_materials.none.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Header
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(128.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: color_materials.gray25.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            &choose_language,
                            TextStyle {
                                font: default_font.0.clone(),
                                font_size: 64.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            // Content
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Undefined),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: Rect::all(Val::Auto),
                        ..Default::default()
                    },
                    material: color_materials.none.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),
                                min_size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: color_materials.gray50.clone(),
                            ..Default::default()
                        })
                        .insert(PreviousButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "<",
                                    TextStyle {
                                        font: default_font.0.clone(),
                                        font_size: 64.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(80.0), Val::Percent(10.0)),
                                min_size: Size::new(Val::Px(256.0), Val::Px(64.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: color_materials.gray50.clone(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    &locale,
                                    TextStyle {
                                        font: default_font.0.clone(),
                                        font_size: 64.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),
                                min_size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: color_materials.gray50.clone(),
                            ..Default::default()
                        })
                        .insert(NextButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    ">",
                                    TextStyle {
                                        font: default_font.0.clone(),
                                        font_size: 64.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                });
        })
        .id();
    commands.insert_resource(MenuData { root_entity });
}

fn cleanup(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.root_entity).despawn_recursive();
}

#[allow(clippy::type_complexity)]
fn interaction(
    color_materials: Res<ColorMaterials>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color_material) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color_material = color_materials.gray25.clone();
            }
            Interaction::Hovered => {
                *color_material = color_materials.gray75.clone();
            }
            Interaction::None => {
                *color_material = color_materials.gray50.clone();
            }
        }
    }
}

fn rotate_right(
    mut commands: Commands,
    fluent_server: FluentServer,
    mut locale_bundle: LocaleBundle,
    mut game_state: ResMut<State<GameState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<PreviousButton>)>,
) {
    if let Ok(Interaction::Clicked) = query.single() {
        let handle = fluent_server.load_pack(locale_bundle.rotate_right(1).pack(menu));
        commands.insert_resource(handle);
        game_state.set(GameState::Load).unwrap();
    }
}

fn rotate_left(
    mut commands: Commands,
    fluent_server: FluentServer,
    mut locale_bundle: LocaleBundle,
    mut game_state: ResMut<State<GameState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<NextButton>)>,
) {
    if let Ok(Interaction::Clicked) = query.single() {
        let handle = fluent_server.load_pack(locale_bundle.rotate_left(1).pack(menu));
        commands.insert_resource(handle);
        game_state.set(GameState::Load).unwrap();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Load,
    Menu,
}

struct MenuData {
    root_entity: Entity,
}

struct NextButton;

struct PreviousButton;

struct ColorMaterials {
    gray25: Handle<ColorMaterial>,
    gray50: Handle<ColorMaterial>,
    gray75: Handle<ColorMaterial>,
    none: Handle<ColorMaterial>,
}

impl FromWorld for ColorMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut color_material = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let gray25 = color_material.add(Color::rgb(0.25, 0.25, 0.25).into());
        let gray50 = color_material.add(Color::rgb(0.50, 0.50, 0.50).into());
        let gray75 = color_material.add(Color::rgb(0.75, 0.75, 0.75).into());
        let none = color_material.add(Color::NONE.into());
        Self {
            gray25,
            gray50,
            gray75,
            none,
        }
    }
}

struct DefaultFont(Handle<Font>);

impl FromWorld for DefaultFont {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        Self(font)
    }
}

mod locales;
mod to_sentence_case;
