use crate::locales::{Lang, Localizer};
use crate::view::components::{card, modern_scrollable};
use crate::view::icons;
use crate::{AppState, Message};
use iced::widget::{
    Space, button, checkbox, column, container, pick_list, row, text, text_editor, text_input,
};
use iced::{Alignment, Border, Color, Element, Font, Length, Theme, border};

fn split_rule(rule: &str) -> (String, String, String) {
    let mut parts = rule.splitn(3, ',');
    let rule_type = parts.next().unwrap_or("").trim().to_string();
    let payload = parts.next().unwrap_or("").trim().to_string();
    let target = parts.next().unwrap_or("").trim().to_string();
    (rule_type, payload, target)
}

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
        "MATCH".to_string(),
    ];

    let add_rule_form = card(column![
        text(lang.tr("rules_add_custom")).font(bold_font),
        Space::new().height(15),
        row![
            column![
                text(lang.tr("rules_type")).size(12),
                pick_list(
                    rule_types,
                    Some(&state.new_rule_type),
                    Message::UpdateNewRuleType
                )
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
                button(
                    row![
                        text(icons::ADD).size(12),
                        text(lang.tr("rules_add_btn")).size(12)
                    ]
                    .spacing(8)
                )
                .on_press(Message::AddCustomRule)
                .padding([8, 16])
                .style(if state.is_adding_rule {
                    button::secondary
                } else {
                    button::primary
                }),
            ]
            .spacing(5),
        ]
        .align_y(Alignment::Center)
    ]);

    let filter_text = state.rules_filter.to_lowercase();
    let filtered_rules: Vec<(usize, &infiltrator_core::rules::RuleEntry)> = state
        .rules
        .iter()
        .enumerate()
        .filter(|(_, entry)| {
            if filter_text.is_empty() {
                return true;
            }
            entry.rule.to_lowercase().contains(&filter_text)
        })
        .collect();

    let save_action: Element<'_, Message> = if state.is_saving_rules {
        button(text(lang.tr("rules_saving")).size(12))
            .padding([6, 12])
            .style(button::secondary)
            .into()
    } else if state.rules_dirty {
        button(
            row![
                text(icons::SAVE).size(12),
                text(lang.tr("rules_save_btn")).size(12)
            ]
            .spacing(8),
        )
        .on_press(Message::SaveRules)
        .padding([6, 12])
        .style(button::primary)
        .into()
    } else {
        button(text(lang.tr("rules_saved")).size(12))
            .padding([6, 12])
            .style(button::secondary)
            .into()
    };

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
            button(
                row![
                    text(icons::REFRESH).size(12),
                    text(lang.tr("refresh")).size(12)
                ]
                .spacing(8),
            )
            .on_press(Message::LoadRules)
            .padding([6, 12])
            .style(button::secondary)
            .into()
        },
        Space::new().width(8),
        save_action
    ]
    .align_y(Alignment::Center);

    let mut proxy_providers_list = column![
        text(lang.tr("rules_proxy_providers")).font(bold_font),
        Space::new().height(10),
    ]
    .spacing(8);

    if state.proxy_providers.is_empty() {
        proxy_providers_list =
            proxy_providers_list.push(text(lang.tr("rules_no_providers")).size(12).style(|_| {
                text::Style {
                    color: Some(Color::from_rgb(0.4, 0.4, 0.4)),
                }
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
                        button(
                            row![
                                text(icons::UPDATE).size(10),
                                text(lang.tr("btn_update")).size(10)
                            ]
                            .spacing(6)
                        )
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
            rule_providers_list.push(text(lang.tr("rules_no_providers")).size(12).style(|_| {
                text::Style {
                    color: Some(Color::from_rgb(0.4, 0.4, 0.4)),
                }
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
                        button(
                            row![
                                text(icons::UPDATE).size(10),
                                text(lang.tr("btn_update")).size(10)
                            ]
                            .spacing(6)
                        )
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

    let mut rules_list = column![].spacing(6);
    if state.rules.is_empty() {
        rules_list = rules_list.push(text(lang.tr("rules_empty")));
    } else {
        for (index, entry) in filtered_rules {
            let (rule_type, payload, target) = split_rule(&entry.rule);
            let up_button = if index > 0 {
                button(text("↑").size(12))
                    .on_press(Message::MoveRuleUp(index))
                    .padding([4, 8])
                    .style(button::secondary)
            } else {
                button(text("↑").size(12))
                    .padding([4, 8])
                    .style(button::secondary)
            };
            let down_button = if index + 1 < state.rules.len() {
                button(text("↓").size(12))
                    .on_press(Message::MoveRuleDown(index))
                    .padding([4, 8])
                    .style(button::secondary)
            } else {
                button(text("↓").size(12))
                    .padding([4, 8])
                    .style(button::secondary)
            };
            rules_list = rules_list.push(
                container(
                    row![
                        checkbox(entry.enabled)
                            .on_toggle(move |_| Message::ToggleRuleEnabled(index))
                            .size(16),
                        container(text(rule_type.clone()).size(10).font(bold_font))
                            .padding([2, 6])
                            .style(move |_theme: &Theme| {
                                let color = match rule_type.as_str() {
                                    "DOMAIN" | "DOMAIN-SUFFIX" | "DOMAIN-KEYWORD" => {
                                        Color::from_rgb(0.2, 0.5, 0.8)
                                    }
                                    "IP-CIDR" | "IP-CIDR6" | "GEOIP" => {
                                        Color::from_rgb(0.8, 0.5, 0.2)
                                    }
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
                        Space::new().width(10),
                        column![
                            text(payload).size(13).style(move |_| text::Style {
                                color: Some(if entry.enabled {
                                    Color::from_rgb(0.9, 0.9, 0.9)
                                } else {
                                    Color::from_rgb(0.5, 0.5, 0.5)
                                })
                            }),
                            text(target).size(11).style(move |_| text::Style {
                                color: Some(if entry.enabled {
                                    Color::from_rgb(0.4, 0.7, 0.4)
                                } else {
                                    Color::from_rgb(0.4, 0.4, 0.4)
                                })
                            }),
                        ]
                        .width(Length::Fill),
                        up_button,
                        Space::new().width(6),
                        down_button
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

    let rule_providers_json_save_action: Element<'_, Message> =
        if state.is_saving_rule_providers_json {
            button(text(lang.tr("rules_saving")).size(12))
                .padding([6, 12])
                .style(button::secondary)
                .into()
        } else if state.rule_providers_json_dirty {
            button(
                row![
                    text(icons::SAVE).size(12),
                    text(lang.tr("rules_save_rule_providers_btn")).size(12)
                ]
                .spacing(8),
            )
            .on_press(Message::SaveRuleProvidersJson)
            .padding([6, 12])
            .style(button::primary)
            .into()
        } else {
            button(text(lang.tr("rules_saved")).size(12))
                .padding([6, 12])
                .style(button::secondary)
                .into()
        };

    let sniffer_json_save_action: Element<'_, Message> = if state.is_saving_sniffer_json {
        button(text(lang.tr("rules_saving")).size(12))
            .padding([6, 12])
            .style(button::secondary)
            .into()
    } else if state.sniffer_json_dirty {
        button(
            row![
                text(icons::SAVE).size(12),
                text(lang.tr("rules_save_sniffer_btn")).size(12)
            ]
            .spacing(8),
        )
        .on_press(Message::SaveSnifferJson)
        .padding([6, 12])
        .style(button::primary)
        .into()
    } else {
        button(text(lang.tr("rules_saved")).size(12))
            .padding([6, 12])
            .style(button::secondary)
            .into()
    };

    let proxy_providers_json_save_action: Element<'_, Message> =
        if state.is_saving_proxy_providers_json {
            button(text(lang.tr("rules_saving")).size(12))
                .padding([6, 12])
                .style(button::secondary)
                .into()
        } else if state.proxy_providers_json_dirty {
            button(
                row![
                    text(icons::SAVE).size(12),
                    text("Save Proxy Providers").size(12)
                ]
                .spacing(8),
            )
            .on_press(Message::SaveProxyProvidersJson)
            .padding([6, 12])
            .style(button::primary)
            .into()
        } else {
            button(text(lang.tr("rules_saved")).size(12))
                .padding([6, 12])
                .style(button::secondary)
                .into()
        };

    let rule_providers_json_editor = card(column![
        row![
            text(lang.tr("rules_rule_providers_json")).font(bold_font),
            Space::new().width(Length::Fill),
            rule_providers_json_save_action
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
        text_editor(&state.rule_providers_json_content)
            .on_action(Message::RuleProvidersEditorAction)
            .padding(10)
            .height(Length::Fixed(220.0))
    ]);

    let sniffer_json_editor = card(column![
        row![
            text(lang.tr("rules_sniffer_json")).font(bold_font),
            Space::new().width(Length::Fill),
            sniffer_json_save_action
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
        text_editor(&state.sniffer_json_content)
            .on_action(Message::SnifferEditorAction)
            .padding(10)
            .height(Length::Fixed(220.0))
    ]);

    let proxy_providers_json_editor = card(column![
        row![
            text("Proxy Providers JSON").font(bold_font),
            Space::new().width(Length::Fill),
            proxy_providers_json_save_action
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
        text_editor(&state.proxy_providers_json_content)
            .on_action(Message::ProxyProvidersEditorAction)
            .padding(10)
            .height(Length::Fixed(220.0))
    ]);

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
        row![
            container(rule_providers_json_editor).width(Length::FillPortion(1)),
            Space::new().width(20),
            container(proxy_providers_json_editor).width(Length::FillPortion(1)),
        ],
        Space::new().height(20),
        row![
            container(sniffer_json_editor).width(Length::FillPortion(1)),
            Space::new().width(20),
            Space::new().width(Length::FillPortion(1)),
        ],
        Space::new().height(20),
        search_bar,
        Space::new().height(10),
        modern_scrollable(rules_list).height(Length::Fill),
    ]
    .spacing(10);

    content.into()
}
