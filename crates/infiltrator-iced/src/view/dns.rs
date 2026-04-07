use crate::locales::{Lang, Localizer};
use crate::view::components::card;
use crate::{AppState, Message};
use iced::widget::{Scrollable, Space, button, column, container, row, text, text_input};
use iced::{Alignment, Color, Element, Font, Length};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let save_text = if state.is_saving_dns {
        String::from("...")
    } else {
        lang.tr("dns_save").into_owned()
    };

    let header = row![
        text(lang.tr("dns_title")).size(24).font(bold_font),
        Space::new().width(Length::Fill),
        button(text("Flush Fake-IP Cache").size(12))
            .on_press(Message::FlushFakeIpCache)
            .padding([6, 12])
            .style(button::secondary),
        Space::new().width(10),
        button(text(save_text).size(12))
            .on_press(Message::SaveDns)
            .padding([6, 12])
            .style(button::primary)
    ]
    .align_y(Alignment::Center);

    // 1. Enhanced Mode Selection
    let enhanced_mode_section = card(column![
        text("Enhanced Mode").font(bold_font),
        Space::new().height(10),
        row![
            mode_button(
                "fake-ip".to_string(),
                "Fake-IP (Recommended)".to_string(),
                &state.dns_enhanced_mode
            ),
            Space::new().width(10),
            mode_button(
                "redir-host".to_string(),
                "Redir-Host".to_string(),
                &state.dns_enhanced_mode
            ),
        ]
    ]);

    // 2. Nameservers
    let mut dns_list = column![
        text(lang.tr("dns_nameservers")).font(bold_font),
        Space::new().height(10),
    ]
    .spacing(8);

    for (i, server) in state.dns_nameservers.iter().enumerate() {
        dns_list = dns_list.push(
            row![
                text_input(
                    "e.g. 1.1.1.1 or https://dns.cloudflare.com/dns-query",
                    server
                )
                .on_input(move |v| Message::UpdateDnsServer(i, v))
                .padding(10)
                .size(14),
                button(text("−").size(14))
                    .on_press(Message::RemoveDnsServer(i))
                    .style(button::danger)
                    .padding([8, 12])
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        );
    }

    dns_list = dns_list.push(
        button(text(format!("+ {}", lang.tr("dns_add"))).size(12))
            .on_press(Message::AddDnsServer)
            .padding([6, 12])
            .style(button::secondary),
    );

    // 3. DNS Templates (Quick Add)
    let templates = card(column![
        text("Quick Templates").font(bold_font),
        Space::new().height(10),
        row![
            template_button(
                "Cloudflare".to_string(),
                "https://1.1.1.1/dns-query".to_string()
            ),
            Space::new().width(10),
            template_button(
                "Google".to_string(),
                "https://8.8.8.8/dns-query".to_string()
            ),
            Space::new().width(10),
            template_button(
                "AliDNS".to_string(),
                "https://223.5.5.5/dns-query".to_string()
            ),
        ]
    ]);

    let content = column![
        header,
        Space::new().height(24),
        enhanced_mode_section,
        Space::new().height(20),
        card(dns_list),
        Space::new().height(20),
        templates,
    ]
    .spacing(10);

    Scrollable::new(content).height(Length::Fill).into()
}

fn mode_button<'a>(mode: String, label: String, current_mode: &str) -> Element<'a, Message> {
    let is_active = mode == current_mode;

    button(
        container(text(label).size(12))
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding(8),
    )
    .width(Length::FillPortion(1))
    .style(move |_theme, status| {
        if is_active {
            button::primary(_theme, status)
        } else {
            let mut s = button::secondary(_theme, status);
            s.background = Some(Color::from_rgb(0.15, 0.15, 0.15).into());
            s
        }
    })
    .on_press(Message::UpdateDnsEnhancedMode(mode))
    .into()
}

fn template_button<'a>(label: String, url: String) -> Element<'a, Message> {
    button(text(label).size(11))
        .on_press(Message::UpdateDnsServer(0, url))
        .padding([6, 12])
        .style(button::secondary)
        .into()
}
