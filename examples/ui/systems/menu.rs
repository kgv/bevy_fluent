use crate::{
    components::{Menu, NextButton, PreviousButton},
    resources::Font,
    systems::parameters::Swiper,
    to_sentence_case::ToSentenceCase,
    GameState,
};
use bevy::{color::palettes::css, prelude::*};
use bevy_fluent::prelude::*;
use fluent_content::Content;

pub fn setup(
    mut commands: Commands,
    font: Res<Font>,
    locale: Res<Locale>,
    localization: Res<Localization>,
) {
    let request = locale.requested.to_string().to_lowercase();
    let locale = localization.content(&request).unwrap().to_sentence_case();
    let choose_language = localization
        .content("choose-language")
        .unwrap()
        .to_sentence_case();
    // camera
    commands.spawn(Camera2dBundle::default()).insert(Menu);
    // ui
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            // Header
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(25.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: css::DARK_GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            &choose_language,
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 64.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });
                });
            // Content
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(75.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            PreviousButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(10.0),
                                    height: Val::Percent(20.0),
                                    min_width: Val::Px(64.0),
                                    min_height: Val::Px(64.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: css::GRAY.into(),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "<",
                                    TextStyle {
                                        font: font.0.clone(),
                                        font_size: 64.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(80.0),
                                height: Val::Percent(20.0),
                                min_width: Val::Px(256.0),
                                min_height: Val::Px(64.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: css::GRAY.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    &locale,
                                    TextStyle {
                                        font: font.0.clone(),
                                        font_size: 64.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });
                    parent
                        .spawn((
                            NextButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(10.0),
                                    height: Val::Percent(20.0),
                                    min_width: Val::Px(64.0),
                                    min_height: Val::Px(64.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: css::GRAY.into(),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    ">",
                                    TextStyle {
                                        font: font.0.clone(),
                                        font_size: 64.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
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
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        *color = match interaction {
            Interaction::Pressed => css::DARK_GRAY.into(),
            Interaction::Hovered => css::SILVER.into(),
            Interaction::None => css::GRAY.into(),
        }
    }
}

pub fn next(
    mut swiper: Swiper,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<NextButton>)>,
) {
    if let Ok(Interaction::Pressed) = query.get_single() {
        swiper.next();
        next_state.set(GameState::Load);
    }
}

pub fn previous(
    mut swiper: Swiper,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<PreviousButton>)>,
) {
    if let Ok(Interaction::Pressed) = query.get_single() {
        swiper.previous();
        next_state.set(GameState::Load);
    }
}

// const LOCALES: &[LanguageIdentifier] = &[de::DE, en::US, ru::BY, ru::RU];

// /// Shift to one of the next or previous locale
// trait Shift {
//     fn shift(&mut self, count: isize);
// }

// impl Shift for Locale {
//     fn shift(&mut self, count: isize) {
//         error!(%count);
//         if let Some(mut position) = LOCALES.iter().position(|locale| locale == self.requested()) {
//             error!(%position);
//             if count.is_positive() {
//                 let count = count as _;
//                 position = position.saturating_add(count).min(LOCALES.len() - 1);
//             } else if count.is_negative() {
//                 let count = count.abs() as _;
//                 position = position.saturating_sub(count);
//             }
//             error!(%position);
//             *self =
//                 Self::new(LOCALES[position].clone()).with_default(self.default().unwrap().clone());
//         }
//     }
// }
