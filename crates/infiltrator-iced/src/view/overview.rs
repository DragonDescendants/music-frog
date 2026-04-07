use crate::locales::{Lang, Localizer};
use crate::utils::format_bytes;
use crate::view::components::{card, TrafficChart};
use crate::{AppState, Message, Route};
use iced::widget::{Space, button, column, container, row, text, Canvas};
use iced::{Alignment, Color, Element, Length, Font};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = text(lang.tr("nav_overview")).size(24).font(bold_font);

    let active_profile = state.profiles.iter().find(|p| p.active);
    
    let profile_section = card(column![
        text(lang.tr("nav_profiles")).font(bold_font),
        Space::new().height(10),
        row![
            column![
                text(active_profile.map(|p| p.name.as_str()).unwrap_or("None")).size(18).font(bold_font),
                text(active_profile.map(|p| p.path.to_string_lossy().to_string()).unwrap_or_default())
                    .size(12)
                    .style(|_theme| text::Style { color: Some(Color::from_rgb(0.5, 0.5, 0.5)) }),
            ].width(Length::Fill),
            button(text(lang.tr("refresh")).size(12))
                .on_press(Message::Navigate(Route::Profiles))
                .padding([6, 12])
        ].align_y(Alignment::Center)
    ]);

    // Traffic Stats Card with Chart
    let traffic_section = card(column![
        row![
            text("TRAFFIC").size(10).font(bold_font).style(|_theme| text::Style { color: Some(Color::from_rgb(0.5, 0.5, 0.5)) }),
            Space::new().width(Length::Fill),
            if let Some(t) = &state.traffic {
                row![
                    text(format!("↑ {}", format_bytes(t.up))).size(10).style(|_theme| text::Style { color: Some(Color::from_rgb(0.4, 0.8, 0.4)) }),
                    Space::new().width(10),
                    text(format!("↓ {}", format_bytes(t.down))).size(10).style(|_theme| text::Style { color: Some(Color::from_rgb(0.4, 0.4, 0.8)) }),
                ]
            } else {
                row![text(lang.tr("waiting_traffic")).size(10).style(|_theme| text::Style { color: Some(Color::from_rgb(0.4, 0.4, 0.4)) })]
            }
        ],
        Space::new().height(10),
        container(
            Canvas::new(TrafficChart { history: state.traffic_history.clone() })
                .width(Length::Fill)
                .height(Length::Fixed(120.0))
        ).padding([5, 0]),
    ]);

    // Core Control
    let core_control = card(column![
        text("CORE").size(10).font(bold_font).style(|_theme| text::Style { color: Some(Color::from_rgb(0.5, 0.5, 0.5)) }),
        Space::new().height(8),
        row![
            column![
                text(if state.runtime.is_some() { "RUNNING" } else { "STOPPED" })
                    .size(18)
                    .font(bold_font)
                    .style(move |_theme| text::Style {
                        color: Some(if state.runtime.is_some() { Color::from_rgb(0.3, 0.8, 0.3) } else { Color::from_rgb(0.8, 0.3, 0.3) })
                    }),
                row![
                    text(state.proxy_mode.as_deref().unwrap_or("rule")).size(12).style(|_theme| text::Style { color: Some(Color::from_rgb(0.4, 0.4, 0.4)) }),
                    Space::new().width(10),
                    if let Some(m) = &state.memory {
                        text(format!("MEM: {}", format_bytes(m.inuse))).size(12).style(|_theme| text::Style { color: Some(Color::from_rgb(0.4, 0.4, 0.4)) })
                    } else {
                        text("").size(0)
                    },
                    Space::new().width(10),
                    if let Some(ip) = &state.public_ip {
                        text(format!("IP: {}", ip)).size(12).style(|_theme| text::Style { color: Some(Color::from_rgb(0.4, 0.4, 0.4)) })
                    } else {
                        text("").size(0)
                    }
                ].align_y(Alignment::Center),
            ].width(Length::Fill),
            if state.runtime.is_some() {
                button(text(lang.tr("stop_proxy")).size(12))
                    .on_press(Message::StopProxy)
                    .style(button::danger)
                    .padding([8, 16])
            } else if !state.is_starting {
                button(text(lang.tr("start_proxy")).size(12))
                    .on_press(Message::StartProxy)
                    .style(button::primary)
                    .padding([8, 16])
            } else {
                button(text(lang.tr("status_starting")).size(12))
                    .padding([8, 16])
            }
        ].align_y(Alignment::Center)
    ]);

    // Current Proxy Node
    let global_proxy = state.proxies.get("GLOBAL");
    let proxy_node_section: Element<'_, Message> = if let Some(_rt) = &state.runtime {
        let current_node = global_proxy.and_then(|p| p.now.as_ref()).cloned().unwrap_or_else(|| "Unknown".to_string());
        card(column![
            text(lang.tr("nav_proxies")).font(bold_font),
            Space::new().height(10),
            row![
                column![
                    text(current_node).size(18).font(bold_font),
                    text("Selected from GLOBAL group")
                        .size(12)
                        .style(|_theme| text::Style { color: Some(Color::from_rgb(0.5, 0.5, 0.5)) }),
                ].width(Length::Fill),
                button(text("Switch").size(12))
                    .on_press(Message::Navigate(Route::Proxies))
                    .padding([6, 12])
            ].align_y(Alignment::Center)
        ]).into()
    } else {
        container(Space::new().height(0)).into()
    };

    column![
        header,
        Space::new().height(24),
        profile_section,
        Space::new().height(20),
        traffic_section,
        Space::new().height(20),
        core_control,
        Space::new().height(20),
        proxy_node_section,
        if let Some(err) = &state.error_msg {
            column![
                Space::new().height(20),
                container(text(format!("Error: {}", err)).style(|_theme| text::Style { color: Some(Color::from_rgb(1.0, 0.2, 0.2)) }))
                    .padding(10)
                    .style(|_theme| container::Style {
                        background: Some(Color::from_rgb(0.2, 0.1, 0.1).into()),
                        border: iced::Border { radius: 8.0.into(), ..Default::default() },
                        ..Default::default()
                    })
            ]
        } else {
            column![]
        }
    ]
    .into()
}
