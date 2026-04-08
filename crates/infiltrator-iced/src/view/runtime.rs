use crate::locales::{Lang, Localizer};
use crate::utils::format_bytes;
use crate::view::components::{TrafficChart, card};
use crate::{AppState, Message};
use iced::widget::{Canvas, Scrollable, Space, button, column, container, pick_list, row, text};
use iced::{Alignment, Border, Color, Element, Font, Length, Theme};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    if state.runtime.is_none() && !state.is_starting {
        return container(card(text(lang.tr("proxy_not_running"))))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
    }

    let header = text(lang.tr("runtime_title")).size(24).font(bold_font);

    // 1. Real-time Traffic Section
    let traffic_section = card(column![
        row![
            column![
                text(lang.tr("overview_traffic"))
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
                row![text(lang.tr("waiting_traffic")).size(10)]
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
            text("CONNECTIONS")
                .size(10)
                .font(bold_font)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                }),
            Space::new().width(Length::Fill),
            button(text(lang.tr("btn_close_all")).size(10))
                .on_press(Message::CloseAllConnections)
                .padding([4, 8])
                .style(button::danger),
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
    ];

    if let Some(c) = &state.connections {
        let mut conn_list = column![].spacing(8);

        let mut sorted_conns = c.connections.clone();
        sorted_conns.sort_by(|a, b| b.download.cmp(&a.download));

        for conn in sorted_conns {
            let host = if conn.metadata.host.is_empty() {
                conn.metadata.destination_ip.clone()
            } else {
                conn.metadata.host.clone()
            };

            let rule_str = format!("{}({})", conn.rule, conn.rule_payload);
            let payload_str = format!("{}:{}", host, conn.metadata.destination_port);
            let source_ip = format!("{} ->", conn.metadata.source_ip);
            let network = conn.metadata.network.to_uppercase();

            let row_content = column![
                row![
                    text(network)
                        .size(10)
                        .font(bold_font)
                        .style(|_theme| text::Style {
                            color: Some(Color::from_rgb(0.4, 0.4, 0.4))
                        }),
                    Space::new().width(8),
                    text(host).size(13).font(bold_font).width(Length::Fill),
                    text(format!(
                        "↑ {} / ↓ {}",
                        format_bytes(conn.upload),
                        format_bytes(conn.download)
                    ))
                    .size(10)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                    }),
                    Space::new().width(12),
                    button(text("×").size(10))
                        .on_press(Message::CloseConnection(conn.id.clone()))
                        .padding([2, 6])
                        .style(button::danger),
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
                        background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.03).into()),
                        border: Border {
                            radius: 6.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }));
        }

        connections_section = connections_section.push(Scrollable::new(conn_list).height(Length::Fill));
    } else {
        connections_section = connections_section.push(text("No active connections").size(12));
    }

    let logs_section = column![
        row![
            text(lang.tr("runtime_system_logs"))
                .font(bold_font)
                .width(Length::Fill),
            pick_list(
                &["debug", "info", "warning", "error"][..],
                Some(state.log_level.as_str()),
                |l| Message::SetLogLevel(l.to_string())
            )
            .text_size(11),
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
        container(
            Scrollable::new(
                column(state.logs.iter().map(|l| text(l).size(11).into()))
                    .spacing(2)
                    .padding(5)
            )
            .id(iced::widget::Id::new("log_scroller"))
            .height(Length::Fill)
        )
        .style(|_theme| container::Style {
            background: Some(Color::BLACK.into()),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
    ];

    let content = column![
        header,
        Space::new().height(20),
        traffic_section,
        Space::new().height(20),
        container(card(connections_section)).height(Length::FillPortion(2)),
        Space::new().height(20),
        container(card(logs_section)).height(Length::FillPortion(1)),
    ]
    .spacing(10);

    content.into()
}
