use crate::locales::{Lang, Localizer};
use crate::view::components::nav_button;
use crate::{AppState, Message, Route};
use iced::widget::{Space, column, container, image, row, text};
use iced::{Alignment, Color, Element, Length};

pub fn sidebar(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = iced::Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let logo = container(
        row![
            image("src-tauri/icons/icon.png").width(32).height(32),
            Space::new().width(12),
            column![
                text("MusicFrog").size(16).font(bold_font),
                text("Infiltrator").size(10).style(|_| text::Style {
                    color: Some(Color::from_rgb(0.4, 0.4, 0.4))
                }),
            ]
        ]
        .align_y(Alignment::Center),
    )
    .padding(20);

    container(column![
        logo,
        Space::new().height(10),
        nav_button(
            lang.tr("nav_overview").into_owned(),
            Route::Overview,
            &state.current_route
        ),
        nav_button(
            lang.tr("nav_profiles").into_owned(),
            Route::Profiles,
            &state.current_route
        ),
        nav_button(
            lang.tr("nav_proxies").into_owned(),
            Route::Proxies,
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
            lang.tr("nav_sync").into_owned(),
            Route::Sync,
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
