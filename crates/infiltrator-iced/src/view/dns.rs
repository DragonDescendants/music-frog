use crate::locales::{Lang, Localizer};
use crate::view::components::card;
use crate::{AppState, Message};
use iced::widget::{Scrollable, Space, button, column, row, text, text_input};
use iced::{Alignment, Color, Element, Font, Length};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = row![
        text(lang.tr("dns_title")).size(24).font(bold_font),
        Space::new().width(Length::Fill),
        button(
            text(if state.is_saving_dns {
                "..."
            } else {
                &lang.tr("dns_save")
            })
            .size(12)
        )
        .on_press(Message::SaveDns)
        .padding([6, 12])
        .style(button::primary)
    ]
    .align_y(Alignment::Center);

    let mut dns_list = column![
        text(lang.tr("dns_nameservers")).font(bold_font),
        Space::new().height(10),
    ]
    .spacing(8);

    for (i, server) in state.dns_nameservers.iter().enumerate() {
        dns_list = dns_list.push(
            row![
                text_input("DNS Server", server)
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

    let content = column![header, Space::new().height(24), card(dns_list),].spacing(20);

    Scrollable::new(content).height(Length::Fill).into()
}
