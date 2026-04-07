use crate::locales::{Lang, Localizer};
use crate::view::components::card;
use crate::{AppState, Message};
use iced::widget::{Space, button, column, container, row, text, text_input};
use iced::{Alignment, Element, Font, Length};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = text(lang.tr("sync_title")).size(24).font(bold_font);

    let form = column![
        column![
            text(lang.tr("sync_url")).size(14).font(bold_font),
            text_input("https://dav.example.com", &state.webdav_url)
                .on_input(Message::UpdateWebDavUrl)
                .padding(10)
                .size(14),
        ]
        .spacing(8),
        column![
            text(lang.tr("sync_user")).size(14).font(bold_font),
            text_input("Username", &state.webdav_user)
                .on_input(Message::UpdateWebDavUser)
                .padding(10)
                .size(14),
        ]
        .spacing(8),
        column![
            text(lang.tr("sync_pass")).size(14).font(bold_font),
            text_input("Password", &state.webdav_pass)
                .on_input(Message::UpdateWebDavPass)
                .secure(true)
                .padding(10)
                .size(14),
        ]
        .spacing(8),
        Space::new().height(10),
        row![
            button(
                container(text(lang.tr("sync_upload")).size(12))
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
            )
            .on_press(Message::SyncUpload)
            .width(Length::Fill)
            .padding(12)
            .style(button::primary),
            button(
                container(text(lang.tr("sync_download")).size(12))
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
            )
            .on_press(Message::SyncDownload)
            .width(Length::Fill)
            .padding(12)
            .style(button::secondary),
        ]
        .spacing(20),
        if state.is_syncing {
            container(text("Syncing...").size(12))
                .width(Length::Fill)
                .align_x(Alignment::Center)
        } else {
            container(text("")).width(Length::Fill)
        }
    ]
    .spacing(20);

    column![header, Space::new().height(24), card(form),].into()
}
