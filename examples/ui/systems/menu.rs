use crate::{
    components::{Menu, NextButton, PreviousButton},
    resources::Font,
    systems::parameters::Swiper,
    to_sentence_case::ToSentenceCase,
    GameState,
};
use bevy::{
    color::palettes::css,
    picking::hover::Hovered,
    prelude::*,
    ui::Pressed,
    ui_widgets::{Activate, Button},
};
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
    commands.spawn(Camera2d).insert(Menu);
    // ui
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .insert(Menu)
        .with_children(|parent| {
            // Header
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(25.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(css::DARK_GRAY.into()),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text(choose_language),
                        TextFont {
                            font: font.0.clone().into(),
                            font_size: FontSize::Px(64.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            // Content
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(75.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Auto),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            PreviousButton,
                            Node {
                                width: Val::Percent(10.0),
                                height: Val::Percent(20.0),
                                min_width: Val::Px(64.0),
                                min_height: Val::Px(64.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(css::GRAY.into()),
                            Button,
                            Hovered::default(),
                        ))
                        .observe(on_activate_previous)
                        .with_children(|parent| {
                            parent.spawn((
                                Text("<".to_string()),
                                TextFont {
                                    font: font.0.clone().into(),
                                    font_size: FontSize::Px(64.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(80.0),
                                height: Val::Percent(20.0),
                                min_width: Val::Px(256.0),
                                min_height: Val::Px(64.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(css::GRAY.into()),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text(locale),
                                TextFont {
                                    font: font.0.clone().into(),
                                    font_size: FontSize::Px(64.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                    parent
                        .spawn((
                            NextButton,
                            Node {
                                width: Val::Percent(10.0),
                                height: Val::Percent(20.0),
                                min_width: Val::Px(64.0),
                                min_height: Val::Px(64.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(css::GRAY.into()),
                            Button,
                            Hovered::default(),
                        ))
                        .observe(on_activate_next)
                        .with_children(|parent| {
                            parent.spawn((
                                Text(">".to_string()),
                                TextFont {
                                    font: font.0.clone().into(),
                                    font_size: FontSize::Px(64.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_button_style(
    mut buttons: Query<
        (Has<Pressed>, &Hovered, &mut BackgroundColor),
        (Or<(Changed<Pressed>, Changed<Hovered>)>, With<Button>),
    >,
) {
    for (pressed, hovered, mut color) in &mut buttons {
        set_button_style(hovered.get(), pressed, &mut color);
    }
}

/// Supplementary system to detect removed marker components
pub fn update_button_style2(
    mut buttons: Query<(Has<Pressed>, &Hovered, &mut BackgroundColor), With<Button>>,
    mut removed_depressed: RemovedComponents<Pressed>,
) {
    removed_depressed.read().for_each(|entity| {
        if let Ok((pressed, hovered, mut color)) = buttons.get_mut(entity) {
            set_button_style(hovered.get(), pressed, &mut color);
        }
    });
}

fn set_button_style(hovered: bool, pressed: bool, color: &mut BackgroundColor) {
    *color = if pressed {
        css::DARK_GRAY.into()
    } else if hovered {
        css::SILVER.into()
    } else {
        css::GRAY.into()
    };
}

pub fn on_activate_next(
    _: On<Activate>,
    mut swiper: Swiper,
    mut next_state: ResMut<NextState<GameState>>,
) {
    swiper.next();
    next_state.set(GameState::Load);
}

pub fn on_activate_previous(
    _: On<Activate>,
    mut swiper: Swiper,
    mut next_state: ResMut<NextState<GameState>>,
) {
    swiper.previous();
    next_state.set(GameState::Load);
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
