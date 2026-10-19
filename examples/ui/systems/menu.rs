use crate::{
    components::{NextButton, PreviousButton},
    resources::Font,
    systems::parameters::Swiper,
    to_sentence_case::ToSentenceCase,
    GameState,
};
use bevy::{
    color::palettes::css,
    picking::hover::Hovered,
    prelude::*,
    text::FontSourceTemplate,
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

    commands.spawn_scene(bsn! {
        #Camera
        Camera2d
        DespawnOnExit<GameState>(GameState::Menu)
    });

    commands.spawn_scene(bsn! {
        #UiRoot
        Node {
            width: percent(100),
            height: percent(100),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
        }
        BackgroundColor(Color::NONE)
        DespawnOnExit<GameState>(GameState::Menu)
        Children [
            #Header
            Node {
                width: percent(100),
                height: percent(25),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            BackgroundColor(css::DARK_GRAY)
            Children [
                Text(choose_language)
                TextFont {
                    font: FontSourceTemplate::Handle({font.clone()}),
                    font_size: px(64),
                }
                TextColor(Color::WHITE)
            ]
            --
            #Content
            Node {
                width: percent(100),
                height: percent(75),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: auto(),
            }
            Children [
                #PreviousButton
                PreviousButton
                Button
                Hovered
                Node {
                    width: percent(10),
                    height: percent(20),
                    min_width: px(64),
                    min_height: px(64),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                }
                BackgroundColor(css::GRAY)
                on(on_activate_menu_button)
                Children [
                    Text("<")
                    TextFont {
                        font: FontSourceTemplate::Handle({font.clone()}),
                        font_size: px(64),
                    }
                    TextColor(Color::WHITE)
                ]
                --
                Node {
                    width: percent(80),
                    height: percent(20),
                    min_width: px(256),
                    min_height: px(64),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                }
                BackgroundColor(css::GRAY)
                Children [
                    Text(locale)
                    TextFont {
                        font: FontSourceTemplate::Handle({font.clone()}),
                        font_size: px(64),
                    }
                    TextColor(Color::WHITE)
                ]
                --
                #NextButton
                NextButton
                Button
                Hovered
                Node {
                    width: percent(10),
                    height: percent(20),
                    min_width: px(64),
                    min_height: px(64),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                }
                BackgroundColor(css::GRAY)
                on(on_activate_menu_button)
                Children [
                    Text(">")
                    TextFont {
                        font: FontSourceTemplate::Handle({font.clone()}),
                        font_size: px(64),
                    }
                    TextColor(Color::WHITE)
                ]
            ]
        ]
    });
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

pub fn on_activate_menu_button(
    event: On<Activate>,
    mut swiper: Swiper,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<AnyOf<(&NextButton, &PreviousButton)>>,
) {
    let Ok((is_next_button, _)) = query
        .get(event.event_target())
        .map(|(next, prev)| (next.is_some(), prev.is_some()))
    else {
        return;
    };

    if is_next_button {
        swiper.next();
    } else {
        swiper.previous();
    }

    next_state.set(GameState::Load);
}
