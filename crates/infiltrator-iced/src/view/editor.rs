use crate::locales::{Lang, Localizer};
use crate::{AppState, Message};
use iced::widget::{Space, button, column, row, text, text_editor};
use iced::{Alignment, Color, Element, Font, Length};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let title = state
        .editor_path
        .as_ref()
        .and_then(|p: &std::path::PathBuf| p.file_name())
        .and_then(|n: &std::ffi::OsStr| n.to_str())
        .unwrap_or("Untitled");

    let header = row![
        text(format!("Editing: {}", title)).size(24).font(bold_font),
        Space::new().width(Length::Fill),
        if let Some(path) = &state.editor_path {
            Element::from(
                text(path.to_string_lossy().to_string())
                    .size(12)
                    .style(|_| text::Style {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                    }),
            )
        } else {
            text("").into()
        }
    ]
    .align_y(Alignment::Center);

    let editor = text_editor(&state.editor_content).on_action(Message::EditorAction);

    let actions = row![
        button(text(lang.tr("dns_save")).size(12))
            .on_press(Message::SaveProfile)
            .padding([8, 16])
            .style(button::primary),
        Space::new().width(10),
        button(text("Cancel").size(12))
            .on_press(Message::Navigate(crate::types::Route::Profiles))
            .padding([8, 16])
            .style(button::secondary),
    ];

    column![
        header,
        Space::new().height(20),
        container(editor)
            .height(Length::Fill)
            .padding(10)
            .style(|_| container::Style {
                background: Some(Color::from_rgb(0.05, 0.05, 0.05).into()),
                ..Default::default()
            }),
        Space::new().height(20),
        actions,
    ]
    .into()
}

use iced::widget::container;
