use crate::locales::{Lang, Localizer};
use crate::view::components::card;
use crate::view::icons;
use crate::{AppState, Message};
use iced::widget::{Scrollable, Space, button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Border, Color, Element, Font, Length, Theme, border};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let search_bar = text_input(&lang.tr("rules_filter_placeholder"), &state.rules_filter)
        .on_input(Message::FilterRules)
        .padding(10)
        .size(16);

    // 0. Add Custom Rule Form
    let mut available_targets: Vec<String> = state
        .proxies
        .iter()
        .filter(|(_, p): &(&String, &mihomo_api::Proxy)| p.is_group())
        .map(|(name, _)| name.clone())
        .collect();
    available_targets.sort();
    if !available_targets.contains(&"DIRECT".to_string()) {
        available_targets.push("DIRECT".to_string());
    }
    if !available_targets.contains(&"REJECT".to_string()) {
        available_targets.push("REJECT".to_string());
    }

    let rule_types = vec![
        "DOMAIN".to_string(),
        "DOMAIN-SUFFIX".to_string(),
        "DOMAIN-KEYWORD".to_string(),
        "IP-CIDR".to_string(),
        "IP-CIDR6".to_string(),
        "GEOIP".to_string(),
    ];

    let add_rule_form = card(column![
        text(lang.tr("rules_add_custom")).font(bold_font),
        Space::new().height(15),
        row![
            column![
                text(lang.tr("rules_type")).size(12),
                pick_list(rule_types, Some(&state.new_rule_type), |t| {
                    Message::UpdateNewRuleType(t)
                })
                .width(Length::Fill),
            ]
            .width(Length::FillPortion(1))
            .spacing(5),
            Space::new().width(15),
            column![
                text(lang.tr("rules_payload")).size(12),
                text_input("e.g. google.com", &state.new_rule_payload)
                    .on_input(Message::UpdateNewRulePayload)
                    .padding(8),
            ]
            .width(Length::FillPortion(2))
            .spacing(5),
            Space::new().width(15),
            column![
                text(lang.tr("rules_target")).size(12),
                pick_list(available_targets, Some(&state.new_rule_target), |t| {
                    Message::UpdateNewRuleTarget(t)
                })
                .width(Length::Fill),
            ]
            .width(Length::FillPortion(1))
            .spacing(5),
            Space::new().width(15),
            column![
                text(" ").size(12),
                button(row![text(icons::ADD).size(12), text(lang.tr("rules_add_btn")).size(12)].spacing(8))
                    .on_press(Message::AddCustomRule)
                    .padding([8, 16])
                    .style(if state.is_adding_rule { button::secondary } else { button::primary }),
            ]
            .spacing(5),
        ]
        .align_y(Alignment::Center)
    ]);

    let filtered_rules: Vec<_> = state
        .rules
        .iter()
        .filter(|r| {
            state.rules_filter.is_empty()
                || r.payload
                    .to_lowercase()
                    .contains(&state.rules_filter.to_lowercase())
                || r.proxy
                    .to_lowercase()
                    .contains(&state.rules_filter.to_lowercase())
        })
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
            button(row![text(icons::REFRESH).size(12), text(lang.tr("refresh")).size(12)].spacing(8))
                .on_press(Message::LoadRules)
                .padding([6, 12])
                .style(button::secondary)
                .into()
        }
    ]
    .align_y(Alignment::Center);

    // 1. Providers Section
    let mut proxy_providers_list = column![
        text(lang.tr("rules_proxy_providers")).font(bold_font),
        Space::new().height(10),
    ]
    .spacing(8);

    if state.proxy_providers.is_empty() {
        proxy_providers_list =
            proxy_providers_list.push(text(lang.tr("rules_no_providers")).size(12).style(|_| text::Style {
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
                        button(row![text(icons::UPDATE).size(10), text(lang.tr("btn_update")).size(10)].spacing(6))
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

    let mut rule_providers_list = column![
        text(lang.tr("rules_rule_providers")).font(bold_font),
        Space::new().height(10),
    ]
    .spacing(8);

    if state.rule_providers.is_empty() {
        rule_providers_list =
            rule_providers_list.push(text(lang.tr("rules_no_providers")).size(12).style(|_| text::Style {
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
                        button(row![text(icons::UPDATE).size(10), text(lang.tr("btn_update")).size(10)].spacing(6))
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

    // 2. Rules List
    let mut rules_list = column![].spacing(1);

    if state.rules.is_empty() {
        rules_list = rules_list.push(text(lang.tr("proxy_not_running")));
    } else {
        for rule in filtered_rules {
            rules_list = rules_list.push(
                container(
                    row![
                        container(text(&rule.rule_type).size(10).font(bold_font))
                            .padding([2, 6])
                            .style(move |_theme: &Theme| {
                                let color = match rule.rule_type.as_str() {
                                    "Domain" | "DomainSuffix" | "DomainKeyword" => {
                                        Color::from_rgb(0.2, 0.5, 0.8)
                                    }
                                    "IPCIDR" | "GeoIP" => Color::from_rgb(0.8, 0.5, 0.2),
                                    _ => Color::from_rgb(0.5, 0.5, 0.5),
                                };
                                container::Style {
                                    background: Some(color.into()),
                                    border: Border {
                                        radius: 4.0.into(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }
                            }),
                        Space::new().width(12),
                        text(&rule.payload).size(13).width(Length::Fill),
                        text(&rule.proxy).size(12).style(|_theme| text::Style {
                            color: Some(Color::from_rgb(0.4, 0.7, 0.4))
                        }),
                    ]
                    .align_y(Alignment::Center),
                )
                .padding(8)
                .style(|_theme: &Theme| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.02).into()),
                    ..Default::default()
                }),
            );
        }
    }

    let content = column![
        header,
        Space::new().height(20),
        add_rule_form,
        Space::new().height(20),
        row![
            container(card(proxy_providers_list)).width(Length::FillPortion(1)),
            Space::new().width(20),
            container(card(rule_providers_list)).width(Length::FillPortion(1)),
        ],
        Space::new().height(20),
        search_bar,
        Space::new().height(10),
        Scrollable::new(rules_list).height(Length::Fill),
    ]
    .spacing(10);

    content.into()
}
