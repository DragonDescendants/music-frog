use crate::locales::{Lang, Localizer};
use crate::view::components::{card, modern_scrollable};
use crate::view::icons;
use crate::{AppState, Message};
use iced::widget::{Space, button, checkbox, column, container, row, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Theme, border};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = iced::Font { weight: iced::font::Weight::Bold, ..Default::default() };

    let search_bar = row![
        text_input(lang.tr("rules_filter_placeholder").as_ref(), &state.proxy_filter)
            .on_input(Message::FilterProxies)
            .padding(10)
            .size(14)
            .width(Length::Fixed(240.0)),
        if !state.proxy_filter.is_empty() {
            button(text(icons::CLOSE).size(12))
                .on_press(Message::FilterProxies(String::new()))
                .padding([10, 14])
                .style(button::secondary)
        } else {
            button(text(" ").size(12)).padding([10, 14]).style(button::secondary)
        }
    ].spacing(5).align_y(Alignment::Center);

    let header = row![
        text(lang.tr("proxies_title")).size(28).font(bold_font),
        Space::new().width(30),
        search_bar,
        Space::new().width(20),
        checkbox(state.proxy_sort_by_delay)
            .label(lang.tr("proxies_sort_delay").into_owned())
            .on_toggle(|_| Message::ToggleProxySort)
            .size(18),
        Space::new().width(Length::Fill),
        button(row![text(icons::REFRESH).size(14), text(lang.tr("refresh")).size(14)].spacing(10))
            .on_press(Message::LoadProxies)
            .padding([10, 20])
            .style(button::secondary)
    ]
    .align_y(Alignment::Center);

    if state.runtime.is_none() {
        return column![header, Space::new().height(40), card(text(lang.tr("proxy_not_running"))) ].into();
    }

    // 给右侧留出滚动条空间
    let mut groups_col = column![].spacing(30).padding(iced::Padding {
        top: 0.0,
        right: 20.0,
        bottom: 0.0,
        left: 0.0,
    });

    for (group_name, members) in &state.filtered_groups {
        let Some(group_info) = state.proxies.get(group_name) else { continue };

        let group_header = row![
            text(group_name).font(bold_font).size(20),
            Space::new().width(12),
            container(text(group_info.proxy_type().to_string()).size(10))
                .padding([2, 8])
                .style(|_| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.05).into()),
                    border: Border { radius: border::Radius::from(4.0), ..Default::default() },
                    ..Default::default()
                }),
            Space::new().width(Length::Fill),
            text(format!("{} nodes", members.len())).size(12).style(|_| text::Style { color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.3)) }),
            Space::new().width(15),
            button(text(icons::SPEED).size(14))
                .on_press(Message::TestGroupDelay(group_name.clone()))
                .padding([6, 12])
                .style(button::secondary),
        ]
        .align_y(Alignment::Center);

        let mut members_col = column![].spacing(10);
        let mut members_row = row![].spacing(10);

        let mut i = 0;
        for member_name in members {
            let is_active = group_info.now() == Some(member_name);
            let delay = state.proxies.get(member_name).and_then(|p: &mihomo_api::Proxy| p.history().last().map(|h| h.delay));
            let m_name = member_name.clone();
            
            let mut btn = button(
                row![
                    text(member_name).size(14).width(Length::Fill),
                    if let Some(d) = delay {
                        let color = if d < 200 { Color::from_rgb(0.4, 0.8, 0.4) } else if d < 500 { Color::from_rgb(0.8, 0.8, 0.4) } else { Color::from_rgb(0.8, 0.4, 0.4) };
                        text(format!("{}ms", d)).size(11).style(move |_: &Theme| text::Style { color: Some(color) })
                    } else {
                        text("").size(11)
                    }
                ].align_y(Alignment::Center),
            )
            .width(Length::FillPortion(1))
            .padding(12);

            if is_active {
                btn = btn.style(button::primary);
            } else {
                btn = btn.style(button::secondary).on_press(Message::SelectProxy(group_name.clone(), m_name));
            }

            members_row = members_row.push(btn);
            i += 1;
            if i % 3 == 0 {
                members_col = members_col.push(members_row);
                members_row = row![].spacing(10);
            }
        }

        if i % 3 != 0 {
            for _ in 0..(3 - (i % 3)) { members_row = members_row.push(Space::new().width(Length::FillPortion(1))); }
            members_col = members_col.push(members_row);
        }

        groups_col = groups_col.push(card(column![group_header, Space::new().height(15), members_col]));
    }

    let content = column![
        header,
        Space::new().height(30),
        modern_scrollable(groups_col).height(Length::Fill)
    ];

    content.into()
}
