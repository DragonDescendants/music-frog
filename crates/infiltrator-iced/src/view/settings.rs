use crate::locales::{Lang, Localizer};
use crate::view::components::card;
use crate::{AppState, Message};
use iced::widget::{Space, button, checkbox, column, pick_list, row, text};
use iced::{Color, Element, Length};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = iced::Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = text(lang.tr("settings_title")).size(24).font(bold_font);

    let status_text = if state.is_starting {
        lang.tr("status_starting")
    } else if state.runtime.is_some() {
        lang.tr("status_running")
    } else {
        lang.tr("status_stopped")
    };

    let mut col = column![
        header,
        Space::new().height(24),
        card(
            column![
                row![
                    text(lang.tr("status_label").replace("{0}", "")).size(16),
                    text(status_text)
                        .size(16)
                        .font(bold_font)
                        .style(move |_theme| text::Style {
                            color: Some(if state.runtime.is_some() {
                                Color::from_rgb(0.3, 0.8, 0.3)
                            } else {
                                Color::from_rgb(0.8, 0.3, 0.3)
                            })
                        })
                ]
                .spacing(8),
                if let Some(err) = &state.error_msg {
                    text(lang.tr("error_label").replace("{0}", err))
                        .size(14)
                        .style(|_theme| text::Style {
                            color: Some(Color::from_rgb(1.0, 0.2, 0.2)),
                        })
                } else {
                    text("")
                }
            ]
            .spacing(12)
        ),
        Space::new().height(20),
    ]
    .spacing(10);

    if let Some(_rt) = &state.runtime {
        const MODES: [&str; 4] = ["rule", "global", "direct", "script"];
        let current_mode = state.proxy_mode.as_deref().unwrap_or("rule");

        col = col.push(card(
            column![
                text(lang.tr("proxy_mode")).font(bold_font),
                pick_list(&MODES[..], Some(current_mode), |m| Message::SetProxyMode(
                    m.to_string()
                ))
                .width(Length::Fill),
                Space::new().height(10),
                checkbox(state.tun_enabled.unwrap_or(false))
                    .label(lang.tr("tun_mode").into_owned())
                    .on_toggle(Message::SetTunEnabled)
            ]
            .spacing(12),
        ));

        col = col.push(Space::new().height(20));
        col = col.push(
            button(text(lang.tr("stop_proxy")).font(bold_font))
                .on_press(Message::StopProxy)
                .style(button::danger)
                .width(Length::Fill)
                .padding(12),
        );
    } else if !state.is_starting {
        col = col.push(
            button(text(lang.tr("start_proxy")).font(bold_font))
                .on_press(Message::StartProxy)
                .style(button::primary)
                .width(Length::Fill)
                .padding(12),
        );
    }

    col.into()
}
