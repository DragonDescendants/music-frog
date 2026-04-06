use crate::locales::{Lang, Localizer};
use crate::view::components::{nav_button, status_dot};
use crate::{AppState, Message, Route};
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Color, Element, Length};

pub fn sidebar(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = iced::Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    container(column![
        row![
            status_dot(state.runtime.is_some()),
            Space::new().width(8),
            text(lang.tr("app_title")).size(18).font(bold_font),
        ]
        .align_y(Alignment::Center)
        .padding(20),
        Space::new().height(10),
        nav_button(
            lang.tr("nav_profiles").into_owned(),
            Route::Profiles,
            &state.current_route
        ),
        nav_button(
            lang.tr("nav_runtime").into_owned(),
            Route::Runtime,
            &state.current_route
        ),
        nav_button(
            lang.tr("nav_rules").into_owned(),
            Route::Rules,
            &state.current_route
        ),
        nav_button(
            lang.tr("nav_dns").into_owned(),
            Route::Dns,
            &state.current_route
        ),
        nav_button(
            lang.tr("nav_settings").into_owned(),
            Route::Settings,
            &state.current_route
        ),
        Space::new().height(Length::Fill),
        // Bottom version info
        container(
            text(format!("v{}", env!("CARGO_PKG_VERSION")))
                .size(10)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.4, 0.4, 0.4))
                })
        )
        .padding(20)
    ])
    .width(220)
    .height(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(Color::from_rgb(0.08, 0.08, 0.08).into()),
        ..Default::default()
    })
    .into()
}
