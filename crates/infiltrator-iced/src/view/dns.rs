use crate::locales::{Lang, Localizer};
use crate::view::components::{card, modern_scrollable};
use crate::view::icons;
use crate::{AppState, Message};
use iced::widget::{Space, button, column, row, text, text_editor};
use iced::{Alignment, Element, Font, Length};

fn save_button<'a>(
    icon: &'a str,
    label: &'a str,
    saving: bool,
    dirty: bool,
    on_press: Message,
) -> Element<'a, Message> {
    if saving {
        button(text("Saving...").size(12))
            .padding([6, 12])
            .style(button::secondary)
            .into()
    } else if dirty {
        button(row![text(icon).size(12), text(label).size(12)].spacing(8))
            .on_press(on_press)
            .padding([6, 12])
            .style(button::primary)
            .into()
    } else {
        button(text("Saved").size(12))
            .padding([6, 12])
            .style(button::secondary)
            .into()
    }
}

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
            row![
                text(icons::REFRESH).size(12),
                text(lang.tr("refresh")).size(12)
            ]
            .spacing(8)
        )
        .on_press(Message::LoadAdvancedConfigs)
        .padding([6, 12])
        .style(button::secondary),
        Space::new().width(10),
        button(text(lang.tr("dns_flush_fakeip")).size(12))
            .on_press(Message::FlushFakeIpCache)
            .padding([6, 12])
            .style(button::secondary),
    ]
    .align_y(Alignment::Center);

    let dns_editor = card(column![
        row![
            text("DNS Config (Full JSON)").font(bold_font),
            Space::new().width(Length::Fill),
            save_button(
                icons::SAVE,
                "Save DNS",
                state.is_saving_dns,
                state.dns_json_dirty,
                Message::SaveDns
            )
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
        text_editor(&state.dns_json_content)
            .on_action(Message::DnsConfigEditorAction)
            .padding(10)
            .height(Length::Fixed(260.0))
    ]);

    let fake_ip_editor = card(column![
        row![
            text("Fake-IP Config (Full JSON)").font(bold_font),
            Space::new().width(Length::Fill),
            save_button(
                icons::SAVE,
                "Save Fake-IP",
                state.is_saving_fake_ip,
                state.fake_ip_json_dirty,
                Message::SaveFakeIpConfig
            )
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
        text_editor(&state.fake_ip_json_content)
            .on_action(Message::FakeIpConfigEditorAction)
            .padding(10)
            .height(Length::Fixed(220.0))
    ]);

    let tun_editor = card(column![
        row![
            text("TUN Config (Full JSON)").font(bold_font),
            Space::new().width(Length::Fill),
            save_button(
                icons::SAVE,
                "Save TUN",
                state.is_saving_tun,
                state.tun_json_dirty,
                Message::SaveTunConfig
            )
        ]
        .align_y(Alignment::Center),
        Space::new().height(10),
        text_editor(&state.tun_json_content)
            .on_action(Message::TunConfigEditorAction)
            .padding(10)
            .height(Length::Fixed(220.0))
    ]);

    let content = column![
        header,
        Space::new().height(20),
        dns_editor,
        Space::new().height(20),
        fake_ip_editor,
        Space::new().height(20),
        tun_editor,
    ]
    .spacing(10);

    modern_scrollable(content).height(Length::Fill).into()
}
