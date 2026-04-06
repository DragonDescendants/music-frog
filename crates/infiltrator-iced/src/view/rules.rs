use crate::locales::{Lang, Localizer};
use crate::{AppState, Message};
use iced::widget::{Scrollable, Space, button, column, container, row, text, text_input};
use iced::{Alignment, Color, Element, Font, Length};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = row![
        text(lang.tr("rules_title")).size(24).font(bold_font),
        Space::new().width(Length::Fill),
        if state.is_loading_rules {
            Element::from(text("..."))
        } else {
            button(text(lang.tr("refresh")).size(12))
                .on_press(Message::LoadRules)
                .padding([6, 12])
                .into()
        }
    ]
    .align_y(Alignment::Center);

    let search_bar = text_input(&lang.tr("rules_filter_placeholder"), &state.rules_filter)
        .on_input(Message::FilterRules)
        .padding(12)
        .size(14);

    let filtered_rules: Vec<_> = state
        .rules
        .iter()
        .filter(|r| {
            if state.rules_filter.is_empty() {
                true
            } else {
                r.payload
                    .to_lowercase()
                    .contains(&state.rules_filter.to_lowercase())
                    || r.proxy
                        .to_lowercase()
                        .contains(&state.rules_filter.to_lowercase())
            }
        })
        .take(200) // Performance guard
        .collect();

    let mut rules_list = column![].spacing(8);

    if state.runtime.is_none() {
        rules_list = rules_list.push(text(lang.tr("proxy_not_running")));
    } else if state.rules.is_empty() && !state.is_loading_rules {
        rules_list = rules_list.push(button(text("Fetch Rules")).on_press(Message::LoadRules));
    } else {
        for rule in filtered_rules {
            let row_content = row![
                container(text(&rule.rule_type).size(10).font(bold_font))
                    .padding([2, 6])
                    .style(|_theme: &iced::Theme| container::Style {
                        background: Some(Color::from_rgb(0.2, 0.2, 0.2).into()),
                        border: iced::Border {
                            radius: iced::border::Radius::from(4.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                text(&rule.payload).size(13).width(Length::Fill),
                text(&rule.proxy).size(12).style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.4, 0.7, 1.0))
                }),
            ]
            .spacing(12)
            .align_y(Alignment::Center);

            rules_list = rules_list.push(container(row_content).padding(10).style(
                |_theme: &iced::Theme| container::Style {
                    background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()),
                    border: iced::Border {
                        radius: iced::border::Radius::from(6.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
        }
    }

    column![
        header,
        Space::new().height(20),
        search_bar,
        Space::new().height(20),
        Scrollable::new(rules_list).height(Length::Fill)
    ]
    .into()
}
