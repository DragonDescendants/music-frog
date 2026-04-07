use crate::locales::{Lang, Localizer};
use crate::view::components::card;
use crate::{AppState, Message};
use iced::widget::{Scrollable, Space, button, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, Theme, border};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = iced::Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = text(lang.tr("proxies_title")).size(24).font(bold_font);

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

    // Group proxies by type
    let mut groups: Vec<_> = state
        .proxies
        .iter()
        .filter(|(_, p)| p.all.is_some()) // Only groups have the 'all' field
        .collect();

    // Sort groups: GLOBAL and Selector first
    groups.sort_by(|(na, pa), (nb, pb)| {
        if *na == "GLOBAL" {
            return std::cmp::Ordering::Less;
        }
        if *nb == "GLOBAL" {
            return std::cmp::Ordering::Greater;
        }
        pa.proxy_type.cmp(&pb.proxy_type)
    });

    let mut groups_col = column![].spacing(20);

    for (group_name, group_info) in groups {
        let mut group_content = column![
            row![
                text(group_name).font(bold_font).size(18),
                Space::new().width(10),
                container(text(&group_info.proxy_type).size(10))
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
                button(text("⚡").size(12))
                    .on_press(Message::TestGroupDelay(group_name.clone()))
                    .padding([4, 8])
                    .style(button::secondary),
            ]
            .align_y(Alignment::Center),
            Space::new().height(12),
        ]
        .spacing(8);

        if let Some(members) = &group_info.all {
            let mut members_row = row![].spacing(8);
            let mut members_col = column![].spacing(8);

            // Simple grid implementation
            let mut i = 0;
            for member_name in members {
                let is_active = group_info.now.as_ref() == Some(member_name);

                // Try to find the node info for delay display
                let delay = state
                    .proxies
                    .get(member_name)
                    .and_then(|p| p.history.last().map(|h| h.delay));

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
                    btn = btn.style(button::secondary).on_press(Message::SelectProxy(
                        group_name.clone(),
                        member_name.clone(),
                    ));
                }

                members_row = members_row.push(btn);
                i += 1;

                if i % 3 == 0 {
                    members_col = members_col.push(members_row);
                    members_row = row![].spacing(8);
                }
            }

            if i % 3 != 0 {
                // Add spacers to fill the last row
                for _ in 0..(3 - (i % 3)) {
                    members_row = members_row.push(Space::new().width(Length::FillPortion(1)));
                }
                members_col = members_col.push(members_row);
            }

            group_content = group_content.push(members_col);
        }

        groups_col = groups_col.push(card(group_content));
    }

    content = content.push(Scrollable::new(groups_col).height(Length::Fill));

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
