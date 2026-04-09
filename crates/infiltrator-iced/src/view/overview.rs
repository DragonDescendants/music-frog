use crate::locales::{Lang, Localizer};
use crate::types::RuntimeStatus;
use crate::view::components::{card, premium_card, status_dot};
use crate::view::icons;
use crate::{AppState, Message};
use iced::widget::{Space, button, column, container, row, text};
use iced::{Alignment, Color, Element, Font, Length};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = text(lang.tr("nav_overview")).size(24).font(bold_font);

    // 1. Profile Card (Premium)
    let active_profile = state.profiles.iter().find(|p| p.active);
    let profile_section = premium_card(column![
        text(lang.tr("nav_profiles"))
            .font(bold_font)
            .size(14)
            .style(|_| text::Style {
                color: Some(Color::from_rgb(0.3, 0.6, 1.0))
            }),
        Space::new().height(10),
        row![
            column![
                text(active_profile.map(|p| p.name.as_str()).unwrap_or("None"))
                    .size(20)
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
            button(
                row![
                    text(icons::REFRESH).size(12),
                    text(lang.tr("refresh")).size(12)
                ]
                .spacing(8)
            )
            .on_press(Message::LoadProfiles)
            .padding([8, 16])
            .style(button::secondary)
        ]
        .align_y(Alignment::Center)
    ]);

    // 2. Status & Traffic Card
    let traffic_section = card(column![
        text(lang.tr("overview_traffic")).font(bold_font).size(14),
        Space::new().height(15),
        if let Some(traffic) = &state.traffic {
            row![
                column![
                    text(format!("↑ {}", crate::utils::format_bytes(traffic.up)))
                        .size(16)
                        .font(bold_font),
                    text("Upload").size(10).style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                    }),
                ]
                .width(Length::FillPortion(1)),
                column![
                    text(format!("↓ {}", crate::utils::format_bytes(traffic.down)))
                        .size(16)
                        .font(bold_font),
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

    // 3. Core Status Card (Premium)
    let core_status = match &state.status {
        RuntimeStatus::Starting => lang.tr("status_starting"),
        RuntimeStatus::Running => lang.tr("status_running"),
        RuntimeStatus::Error(_) => lang.tr("status_error"),
        RuntimeStatus::Stopped => lang.tr("status_stopped"),
    };

    let core_section = premium_card(column![
        text(lang.tr("overview_core"))
            .font(bold_font)
            .size(14)
            .style(|_| text::Style {
                color: Some(Color::from_rgb(0.3, 0.6, 1.0))
            }),
        Space::new().height(10),
        row![
            column![
                row![
                    status_dot(matches!(state.status, RuntimeStatus::Running)),
                    Space::new().width(8),
                    text(core_status).size(20).font(bold_font),
                ]
                .align_y(Alignment::Center),
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
            match state.status {
                RuntimeStatus::Running => {
                    button(
                        row![
                            text(icons::CLOSE).size(12),
                            text(lang.tr("stop_proxy")).size(12)
                        ]
                        .spacing(8),
                    )
                    .on_press(Message::StopProxy)
                    .padding([8, 16])
                    .style(button::danger)
                }
                RuntimeStatus::Starting => {
                    button(text(lang.tr("status_starting")).size(12))
                        .padding([8, 16])
                        .style(button::secondary)
                }
                _ => {
                    button(
                        row![
                            text(icons::UPDATE).size(12),
                            text(lang.tr("start_proxy")).size(12)
                        ]
                        .spacing(8),
                    )
                    .on_press(Message::StartProxy)
                    .padding([8, 16])
                    .style(button::primary)
                }
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
        text(lang.tr("nav_proxies")).font(bold_font).size(14),
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
            button(
                row![
                    text(icons::PROXY).size(12),
                    text(lang.tr("btn_switch")).size(12)
                ]
                .spacing(8)
            )
            .on_press(Message::Navigate(crate::types::Route::Proxies))
            .padding([8, 16])
            .style(button::secondary)
        ]
        .align_y(Alignment::Center)
    ]);

    let content = column![
        header,
        Space::new().height(24),
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
