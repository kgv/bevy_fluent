use crate::{
    components::{
        tags::{Menu, NextButton, PreviousButton},
        ColorMaterials, DefaultFont, Locales,
    },
    to_sentence_case::ToSentenceCase,
    GameState,
};
use bevy::prelude::*;
use bevy_fluent::prelude::*;

type InteractionBundle<'a> = (&'a Interaction, &'a mut Handle<ColorMaterial>);

pub fn setup(
    mut commands: Commands,
    assets: Res<Assets<Localization>>,
    color_materials: Res<ColorMaterials>,
    default_font: Res<DefaultFont>,
    handle: ResMut<Handle<Localization>>,
    locales: Res<Locales>,
) {
    let localization = assets.get(handle.id).unwrap();
    let request = locales.current().to_string().to_lowercase();
    let locale = localization.content(&request).unwrap().to_sentence_case();
    let choose_language = localization
        .content("choose-language")
        .unwrap()
        .to_sentence_case();
    // camera
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(Menu);
    // ui
    commands
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
        .insert(Menu)
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
        });
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn interaction(
    color_materials: Res<ColorMaterials>,
    mut query: Query<InteractionBundle, (Changed<Interaction>, With<Button>)>,
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

pub fn next(
    mut game_state: ResMut<State<GameState>>,
    mut locales: ResMut<Locales>,
    query: Query<&Interaction, (Changed<Interaction>, With<NextButton>)>,
) {
    if let Ok(Interaction::Clicked) = query.single() {
        locales.shift(1);
        game_state.set(GameState::Load).unwrap();
    }
}

pub fn previous(
    mut game_state: ResMut<State<GameState>>,
    mut locales: ResMut<Locales>,
    query: Query<&Interaction, (Changed<Interaction>, With<PreviousButton>)>,
) {
    if let Ok(Interaction::Clicked) = query.single() {
        locales.shift(-1);
        game_state.set(GameState::Load).unwrap();
    }
}
