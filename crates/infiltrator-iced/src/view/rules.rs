use crate::locales::{Lang, Localizer};
use crate::view::components::card;
use crate::{AppState, Message};
use iced::widget::{Scrollable, Space, button, column, container, row, text, text_input};
use iced::{Alignment, Border, Color, Element, Font, Length, Theme, border};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

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

    let header = row![
        text(lang.tr("rules_title")).size(24).font(bold_font),
        Space::new().width(10),
        text(format!(
            "({} / {})",
            filtered_rules.len(),
            state.rules.len()
        ))
        .size(14)
        .style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.5, 0.5, 0.5))
        }),
        Space::new().width(Length::Fill),
        if state.is_loading_rules || state.is_loading_providers {
            Element::from(text("..."))
        } else {
            button(text(lang.tr("refresh")).size(12))
                .on_press(Message::LoadRules)
                .padding([6, 12])
                .style(button::secondary)
                .into()
        }
    ]
    .align_y(Alignment::Center);

    // 1. Proxy Providers
    let mut proxy_providers_list = column![
        text("Proxy Providers").font(bold_font),
        Space::new().height(10),
    ]
    .spacing(8);

    if state.proxy_providers.is_empty() {
        proxy_providers_list =
            proxy_providers_list.push(text("No proxy providers").size(12).style(|_| text::Style {
                color: Some(Color::from_rgb(0.4, 0.4, 0.4)),
            }));
    } else {
        for provider in &state.proxy_providers {
            proxy_providers_list = proxy_providers_list.push(
                container(
                    row![
                        column![
                            text(&provider.name).size(14).font(bold_font),
                            text(format!(
                                "{} - Updated: {}",
                                provider.vehicle_type, provider.updated_at
                            ))
                            .size(10)
                            .style(|_| text::Style {
                                color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                            }),
                        ]
                        .width(Length::Fill),
                        button(text("Update").size(10))
                            .on_press(Message::UpdateProxyProvider(provider.name.clone()))
                            .padding([4, 8])
                            .style(button::secondary)
                    ]
                    .align_y(Alignment::Center),
                )
                .padding(8)
                .style(|_| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.03).into()),
                    border: Border {
                        radius: border::Radius::from(6.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
        }
    }

    // 2. Rule Providers
    let mut rule_providers_list = column![
        text("Rule Providers").font(bold_font),
        Space::new().height(10),
    ]
    .spacing(8);

    if state.rule_providers.is_empty() {
        rule_providers_list =
            rule_providers_list.push(text("No rule providers").size(12).style(|_| text::Style {
                color: Some(Color::from_rgb(0.4, 0.4, 0.4)),
            }));
    } else {
        for provider in &state.rule_providers {
            rule_providers_list = rule_providers_list.push(
                container(
                    row![
                        column![
                            text(&provider.name).size(14).font(bold_font),
                            text(format!(
                                "{} rules - Updated: {}",
                                provider.rule_count, provider.updated_at
                            ))
                            .size(10)
                            .style(|_| text::Style {
                                color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                            }),
                        ]
                        .width(Length::Fill),
                        button(text("Update").size(10))
                            .on_press(Message::UpdateRuleProvider(provider.name.clone()))
                            .padding([4, 8])
                            .style(button::secondary)
                    ]
                    .align_y(Alignment::Center),
                )
                .padding(8)
                .style(|_| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.03).into()),
                    border: Border {
                        radius: border::Radius::from(6.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
        }
    }

    let providers_row = row![
        container(proxy_providers_list).width(Length::FillPortion(1)),
        Space::new().width(20),
        container(rule_providers_list).width(Length::FillPortion(1)),
    ];

    let mut rules_list = column![].spacing(8);

    if state.runtime.is_none() {
        rules_list = rules_list.push(text(lang.tr("proxy_not_running")));
    } else if state.rules.is_empty() && !state.is_loading_rules {
        rules_list = rules_list.push(text(lang.tr("refresh")));
    } else {
        for rule in filtered_rules {
            let rule_card = container(
                row![
                    container(text(&rule.rule_type).size(10).font(bold_font))
                        .padding([2, 6])
                        .style(|_theme: &Theme| container::Style {
                            background: Some(Color::from_rgb(0.2, 0.2, 0.2).into()),
                            border: Border {
                                radius: border::Radius::from(4.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    Space::new().width(12),
                    text(&rule.payload).size(13).width(Length::Fill),
                    text(&rule.proxy).size(12).style(|_theme| text::Style {
                        color: Some(Color::from_rgb(0.4, 0.7, 0.4))
                    }),
                ]
                .align_y(Alignment::Center),
            )
            .padding(10)
            .style(|_theme: &Theme| container::Style {
                background: Some(Color::from_rgb(0.08, 0.08, 0.08).into()),
                border: Border {
                    radius: border::Radius::from(8.0),
                    ..Default::default()
                },
                ..Default::default()
            });

            rules_list = rules_list.push(rule_card);
        }
    }

    let content = column![
        header,
        Space::new().height(20),
        card(providers_row),
        Space::new().height(20),
        search_bar,
        Space::new().height(20),
        Scrollable::new(rules_list).height(Length::Fill)
    ]
    .spacing(10);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
