use crate::locales::{Lang, Localizer};
use crate::view::components::card;
use crate::{AppState, Message};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Color, Element, Font, Length};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = text(lang.tr("nav_overview")).size(24).font(bold_font);

    // 1. Profile Card
    let active_profile = state.profiles.iter().find(|p| p.active);
    let profile_section = card(column![
        text(lang.tr("nav_profiles")).font(bold_font),
        Space::new().height(10),
        row![
            column![
                text(active_profile.map(|p| p.name.as_str()).unwrap_or("None"))
                    .size(18)
                    .font(bold_font),
                text(
                    active_profile
                        .map(|p| p.path.to_string_lossy().to_string())
                        .unwrap_or_else(|| "No profile active".to_string())
                )
                .size(12)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                }),
            ]
            .width(Length::Fill),
            button(text(lang.tr("refresh")).size(12))
                .on_press(Message::LoadProfiles)
                .padding([6, 12])
        ]
        .align_y(Alignment::Center)
    ]);

    // 2. Status & Traffic Card
    let traffic_section = card(column![
        text(lang.tr("overview_traffic")).font(bold_font),
        Space::new().height(10),
        if let Some(traffic) = &state.traffic {
            row![
                column![
                    text(format!("↑ {}", crate::utils::format_bytes(traffic.up))).size(14),
                    text("Upload").size(10).style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                    }),
                ]
                .width(Length::FillPortion(1)),
                column![
                    text(format!("↓ {}", crate::utils::format_bytes(traffic.down))).size(14),
                    text("Download").size(10).style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                    }),
                ]
                .width(Length::FillPortion(1)),
            ]
        } else {
            row![text(lang.tr("waiting_traffic"))]
        }
    ]);

    // 3. Core Status Card
    let core_status = if state.is_starting {
        lang.tr("status_starting")
    } else if state.runtime.is_some() {
        lang.tr("status_running")
    } else {
        lang.tr("status_stopped")
    };

    let core_section = card(column![
        text(lang.tr("overview_core")).font(bold_font),
        Space::new().height(10),
        row![
            column![
                text(core_status).size(18).font(bold_font),
                text(format!(
                    "Mode: {}",
                    state.proxy_mode.as_deref().unwrap_or("rule")
                ))
                .size(12)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                }),
            ]
            .width(Length::Fill),
            if state.runtime.is_some() {
                button(text(lang.tr("stop_proxy")).size(12))
                    .on_press(Message::StopProxy)
                    .padding([8, 16])
                    .style(button::danger)
            } else if !state.is_starting {
                button(text(lang.tr("start_proxy")).size(12))
                    .on_press(Message::StartProxy)
                    .padding([8, 16])
                    .style(button::primary)
            } else {
                button(text(lang.tr("status_starting")).size(12))
                    .padding([8, 16])
                    .style(button::secondary)
            }
        ]
        .align_y(Alignment::Center)
    ]);

    // 4. Current Proxy Card
    let current_node = state
        .proxies
        .get("GLOBAL")
        .and_then(|g| g.now())
        .unwrap_or("Unknown");

    let proxy_section = card(column![
        text(lang.tr("nav_proxies")).font(bold_font),
        Space::new().height(10),
        row![
            column![
                text(current_node).size(18).font(bold_font),
                text(lang.tr("overview_selected_global"))
                    .size(12)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                    }),
            ]
            .width(Length::Fill),
            button(text(lang.tr("btn_switch")).size(12))
                .on_press(Message::Navigate(crate::types::Route::Proxies))
                .padding([6, 12])
        ]
        .align_y(Alignment::Center)
    ]);

    let content = column![
        header,
        Space::new().height(20),
        profile_section,
        Space::new().height(20),
        row![
            container(traffic_section).width(Length::FillPortion(1)),
            Space::new().width(20),
            container(core_section).width(Length::FillPortion(1)),
        ],
        Space::new().height(20),
        proxy_section,
    ]
    .spacing(10);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
