use crate::locales::{Lang, Localizer};
use crate::view::components::card;
use crate::view::icons;
use crate::{AppState, Message};
use iced::widget::{Scrollable, Space, button, checkbox, column, container, row, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Theme, border};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = iced::Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let search_bar = row![
        text_input(
            lang.tr("rules_filter_placeholder").as_ref(),
            &state.proxy_filter
        )
        .on_input(Message::FilterProxies)
        .padding(8)
        .size(14)
        .width(Length::Fixed(200.0)),
        if !state.proxy_filter.is_empty() {
            button(text(icons::CLOSE).size(12))
                .on_press(Message::FilterProxies(String::new()))
                .padding([8, 12])
                .style(button::secondary)
        } else {
            button(text(" ").size(12))
                .padding([8, 12])
                .style(button::secondary)
        }
    ]
    .spacing(5)
    .align_y(Alignment::Center);

    let header = row![
        text(lang.tr("proxies_title")).size(24).font(bold_font),
        Space::new().width(20),
        search_bar,
        Space::new().width(10),
        checkbox(state.proxy_sort_by_delay)
            .label(lang.tr("proxies_sort_delay").into_owned())
            .on_toggle(|_| Message::ToggleProxySort)
            .size(16),
        Space::new().width(Length::Fill),
        button(
            row![
                text(icons::REFRESH).size(12),
                text(lang.tr("refresh")).size(12)
            ]
            .spacing(8)
        )
        .on_press(Message::LoadProxies)
        .padding([6, 12])
        .style(button::secondary)
    ]
    .align_y(Alignment::Center);

    if state.runtime.is_none() {
        return column![
            header,
            Space::new().height(40),
            card(text(lang.tr("proxy_not_running")))
        ]
        .into();
    }

    if state.is_loading_proxies && state.proxies.is_empty() {
        return column![
            header,
            Space::new().height(40),
            card(text(lang.tr("refresh")))
        ]
        .into();
    }

    let mut content = column![header, Space::new().height(20)].spacing(20);
    let mut groups_col = column![].spacing(20);

    for (group_name, members) in &state.filtered_groups {
        let Some(group_info) = state.proxies.get(group_name) else {
            continue;
        };

        let group_header = row![
            text(group_name).font(bold_font).size(18),
            Space::new().width(10),
            container(text(group_info.proxy_type().to_string()).size(10))
                .padding([2, 6])
                .style(|_theme: &Theme| container::Style {
                    background: Some(Color::from_rgb(0.2, 0.2, 0.2).into()),
                    border: Border {
                        radius: border::Radius::from(4.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            Space::new().width(Length::Fill),
            text(format!("{} nodes", members.len()))
                .size(12)
                .style(|_| text::Style {
                    color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                }),
            Space::new().width(10),
            button(text(icons::SPEED).size(12))
                .on_press(Message::TestGroupDelay(group_name.clone()))
                .padding([4, 8])
                .style(button::secondary),
        ]
        .align_y(Alignment::Center);

        let mut members_col = column![].spacing(8);
        let mut members_row = row![].spacing(8);

        let mut i = 0;
        for member_name in members {
            let is_active = group_info.now() == Some(member_name);

            let delay = state
                .proxies
                .get(member_name)
                .and_then(|p| p.history().last().map(|h| h.delay));

            let m_name = member_name.clone();
            let mut btn = button(
                row![
                    text(member_name).size(13).width(Length::Fill),
                    if let Some(d) = delay {
                        let color = if d < 200 {
                            Color::from_rgb(0.4, 0.8, 0.4)
                        } else if d < 500 {
                            Color::from_rgb(0.8, 0.8, 0.4)
                        } else {
                            Color::from_rgb(0.8, 0.4, 0.4)
                        };
                        text(format!("{}ms", d))
                            .size(10)
                            .style(move |_: &Theme| text::Style { color: Some(color) })
                    } else {
                        text("").size(10)
                    }
                ]
                .align_y(Alignment::Center),
            )
            .width(Length::FillPortion(1))
            .padding(10);

            if is_active {
                btn = btn.style(button::primary);
            } else {
                btn = btn
                    .style(button::secondary)
                    .on_press(Message::SelectProxy(group_name.clone(), m_name));
            }

            members_row = members_row.push(btn);
            i += 1;

            if i % 3 == 0 {
                members_col = members_col.push(members_row);
                members_row = row![].spacing(8);
            }
        }

        if i % 3 != 0 {
            for _ in 0..(3 - (i % 3)) {
                members_row = members_row.push(Space::new().width(Length::FillPortion(1)));
            }
            members_col = members_col.push(members_row);
        }

        groups_col = groups_col.push(card(column![
            group_header,
            Space::new().height(12),
            members_col
        ]));
    }

    content = content.push(Scrollable::new(groups_col).height(Length::Fill));

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
