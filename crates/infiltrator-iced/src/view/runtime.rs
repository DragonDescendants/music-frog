use crate::locales::{Lang, Localizer};
use crate::utils::format_bytes;
use crate::view::components::card;
use crate::{AppState, Message};
use iced::widget::{Scrollable, Space, button, column, container, pick_list, row, text};
use iced::{Alignment, Border, Color, Element, Font, Length, Theme, border};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = iced::Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = text(lang.tr("runtime_title")).size(24).font(bold_font);

    if state.runtime.is_none() {
        column![
            header,
            Space::new().height(40),
            card(text(lang.tr("proxy_not_running")))
        ]
        .into()
    } else {
        let mut stats_row = row![].spacing(20);

        if let Some(t) = &state.traffic {
            stats_row = stats_row.push(
                container(
                    column![
                        text("UP")
                            .size(10)
                            .font(bold_font)
                            .style(|_theme| text::Style {
                                color: Some(Color::from_rgb(0.4, 0.8, 0.4))
                            }),
                        text(format_bytes(t.up)).size(20).font(bold_font)
                    ]
                    .spacing(4),
                )
                .width(Length::FillPortion(1)),
            );
            stats_row = stats_row.push(
                container(
                    column![
                        text("DOWN")
                            .size(10)
                            .font(bold_font)
                            .style(|_theme| text::Style {
                                color: Some(Color::from_rgb(0.4, 0.4, 0.8))
                            }),
                        text(format_bytes(t.down)).size(20).font(bold_font)
                    ]
                    .spacing(4),
                )
                .width(Length::FillPortion(1)),
            );
        }

        let mut connections_section = column![
            text(format!(
                "Connections ({})",
                state
                    .connections
                    .as_ref()
                    .map(|c| c.connections.len())
                    .unwrap_or(0)
            ))
            .font(bold_font),
            Space::new().height(10),
        ]
        .spacing(10);

        if let Some(c) = &state.connections {
            let mut conn_list = column![].spacing(8);
            for conn in c.connections.iter().take(30) {
                let host = if !conn.metadata.host.is_empty() {
                    &conn.metadata.host
                } else {
                    &conn.metadata.destination_ip
                };

                let row_content = row![
                    container(text(&conn.metadata.network).size(10).font(bold_font))
                        .padding([2, 6])
                        .style(|_theme: &Theme| container::Style {
                            background: Some(Color::from_rgb(0.2, 0.2, 0.2).into()),
                            border: Border {
                                radius: border::Radius::from(4.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    text(format!(
                        "{} -> {}:{}",
                        conn.metadata.source_ip, host, conn.metadata.destination_port
                    ))
                    .size(13)
                    .width(Length::Fill),
                    text(format!(
                        "{} / {}",
                        format_bytes(conn.upload),
                        format_bytes(conn.download)
                    ))
                    .size(11)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                    }),
                    button(text("×").size(12))
                        .on_press(Message::CloseConnection(conn.id.clone()))
                        .style(button::danger)
                        .padding([2, 8])
                ]
                .spacing(12)
                .align_y(Alignment::Center);

                conn_list =
                    conn_list.push(container(row_content).padding(10).style(|_theme: &Theme| {
                        container::Style {
                            background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()),
                            border: Border {
                                radius: border::Radius::from(6.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    }));
            }
            connections_section =
                connections_section.push(Scrollable::new(conn_list).height(Length::Fixed(250.0)));
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
                        .rev()
                        .map(|l| text(l).size(11).font(Font::MONOSPACE).into())
                        .collect::<Vec<Element<Message>>>()
                )
                .spacing(4)
            )
            .height(Length::Fill)
        ];

        column![
            header,
            Space::new().height(20),
            card(stats_row),
            Space::new().height(20),
            card(connections_section),
            Space::new().height(20),
            card(log_section),
        ]
        .into()
    }
}
