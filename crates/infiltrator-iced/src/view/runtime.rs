use crate::locales::{Lang, Localizer};
use crate::utils::format_bytes;
use crate::view::components::{TrafficChart, card};
use crate::{AppState, Message};
use iced::widget::{Canvas, Scrollable, Space, button, column, container, pick_list, row, text};
use iced::{Alignment, Border, Color, Element, Font, Length, Theme, border};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = iced::Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = text(lang.tr("runtime_title")).size(24).font(bold_font);

    if state.runtime.is_none() {
        return column![
            header,
            Space::new().height(40),
            card(text(lang.tr("proxy_not_running")))
        ]
        .into();
    }

    // Traffic Stats Card with Chart
    let traffic_section = card(column![
        row![
            column![
                text("REAL-TIME TRAFFIC")
                    .size(10)
                    .font(bold_font)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                    }),
                if let Some(ip) = &state.public_ip {
                    text(format!("Public IP: {}", ip))
                        .size(10)
                        .style(|_theme| text::Style {
                            color: Some(Color::from_rgb(0.4, 0.4, 0.4)),
                        })
                } else {
                    text("").size(0)
                }
            ],
            Space::new().width(Length::Fill),
            if let Some(t) = &state.traffic {
                row![
                    text(format!("↑ {}", format_bytes(t.up)))
                        .size(10)
                        .style(|_theme| text::Style {
                            color: Some(Color::from_rgb(0.4, 0.8, 0.4))
                        }),
                    Space::new().width(15),
                    text(format!("↓ {}", format_bytes(t.down)))
                        .size(10)
                        .style(|_theme| text::Style {
                            color: Some(Color::from_rgb(0.4, 0.4, 0.8))
                        }),
                    Space::new().width(15),
                    if let Some(m) = &state.memory {
                        text(format!("MEM: {}", format_bytes(m.in_use)))
                            .size(10)
                            .style(|_theme| text::Style {
                                color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                            })
                    } else {
                        text("").size(0)
                    }
                ]
            } else {
                row![]
            }
        ],
        Space::new().height(10),
        Canvas::new(TrafficChart {
            history: state.traffic_history.clone()
        })
        .width(Length::Fill)
        .height(Length::Fixed(100.0)),
    ]);

    let mut connections_section = column![
        row![
            text(format!(
                "Active Connections ({})",
                state
                    .connections
                    .as_ref()
                    .map(|c| c.connections.len())
                    .unwrap_or(0)
            ))
            .font(bold_font)
            .width(Length::Fill),
            button(text("Close All").size(11))
                .on_press(Message::CloseAllConnections)
                .style(button::danger)
                .padding([4, 10]),
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
    ]
    .spacing(10);

    if let Some(c) = &state.connections {
        let mut conn_list = column![].spacing(8);

        let mut sorted_conns = c.connections.clone();
        sorted_conns.sort_by(|a, b| b.download.cmp(&a.download));

        for conn in sorted_conns.iter().take(50) {
            let host = if !conn.metadata.host.is_empty() {
                conn.metadata.host.clone()
            } else {
                conn.metadata.destination_ip.clone()
            };

            let rule_str = format!("[{}]", conn.rule);
            let payload_str = conn.rule_payload.clone();
            let source_ip = conn.metadata.source_ip.clone();
            let network = conn.metadata.network.clone();
            let dest_port = conn.metadata.destination_port.clone();
            let conn_id = conn.id.clone();

            let row_content = column![
                row![
                    container(text(network).size(9).font(bold_font))
                        .padding([1, 4])
                        .style(|_theme: &Theme| container::Style {
                            background: Some(Color::from_rgb(0.2, 0.2, 0.2).into()),
                            border: Border {
                                radius: border::Radius::from(3.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    Space::new().width(8),
                    text(format!("{}:{}", host, dest_port))
                        .size(13)
                        .font(bold_font)
                        .width(Length::Fill),
                    text(format!(
                        "↑{} / ↓{}",
                        format_bytes(conn.upload),
                        format_bytes(conn.download)
                    ))
                    .size(11)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                    }),
                    Space::new().width(12),
                    button(text("×").size(10))
                        .on_press(Message::CloseConnection(conn_id))
                        .style(button::danger)
                        .padding([2, 6])
                ]
                .align_y(Alignment::Center),
                row![
                    text(rule_str).size(10).style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.4, 0.6, 0.9))
                    }),
                    Space::new().width(8),
                    text(payload_str).size(10).style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                    }),
                    Space::new().width(Length::Fill),
                    text(source_ip).size(10).style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.4, 0.4, 0.4))
                    }),
                ]
            ]
            .spacing(4);

            conn_list =
                conn_list.push(container(row_content).padding(8).style(|_theme: &Theme| {
                    container::Style {
                        background: Some(Color::from_rgb(0.08, 0.08, 0.08).into()),
                        border: Border {
                            radius: border::Radius::from(6.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }));
        }
        connections_section =
            connections_section.push(Scrollable::new(conn_list).height(Length::Fixed(300.0)));
    }

    let log_section = column![
        row![
            text("System Logs").font(bold_font),
            Space::new().width(Length::Fill),
            pick_list(
                &["debug", "info", "warning", "error"][..],
                Some(state.log_level.as_str()),
                |l| Message::SetLogLevel(l.to_string())
            )
            .text_size(11)
            .padding(4)
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
        Scrollable::new(
            column(
                state
                    .logs
                    .iter()
                    .map(|l: &String| text(l.clone()).size(11).font(Font::MONOSPACE).into())
                    .collect::<Vec<Element<Message>>>()
            )
            .spacing(4)
        )
        .id(iced::widget::Id::new("log_scroller"))
        .height(Length::Fill)
    ];

    column![
        header,
        Space::new().height(20),
        traffic_section,
        Space::new().height(20),
        card(connections_section),
        Space::new().height(20),
        card(log_section),
    ]
    .into()
}
