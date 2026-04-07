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

    let header = row![
        text(lang.tr("profiles_title")).size(24).font(bold_font),
        Space::new().width(Length::Fill),
        button(text("Open Config Folder").size(12))
            .on_press(Message::OpenConfigDir)
            .padding([6, 12])
            .style(button::secondary)
    ]
    .align_y(Alignment::Center);

    // Import Section
    let import_actions: Element<'_, Message> = if state.is_importing {
        button(text("Importing...").size(12))
            .padding([10, 20])
            .into()
    } else {
        button(text("Import").size(12))
            .on_press(Message::ImportProfile)
            .padding([10, 20])
            .style(button::primary)
            .into()
    };

    let import_panel = card(column![
        text("Import Subscription").font(bold_font),
        Space::new().height(12),
        row![
            text_input("Profile Name (e.g. MyProxy)", &state.import_name)
                .on_input(Message::UpdateImportName)
                .padding(10)
                .width(Length::FillPortion(1)),
            Space::new().width(10),
            text_input("Subscription URL", &state.import_url)
                .on_input(Message::UpdateImportUrl)
                .padding(10)
                .width(Length::FillPortion(2)),
            Space::new().width(10),
            import_actions
        ]
        .align_y(Alignment::Center)
    ]);

    let mut profiles_list = column![].spacing(12);

    if state.is_loading_profiles {
        profiles_list = profiles_list.push(text(lang.tr("loading_profiles")));
    } else if state.profiles.is_empty() {
        profiles_list = profiles_list.push(text(lang.tr("no_profiles")));
    } else {
        for profile in &state.profiles {
            let is_active = profile.active;

            let mut actions = row![].spacing(8);
            if !is_active {
                actions = actions.push(
                    button(text(lang.tr("use")).size(12))
                        .on_press(Message::SetActiveProfile(profile.name.clone()))
                        .padding([6, 12])
                        .style(button::secondary),
                );
                actions = actions.push(
                    button(text(lang.tr("edit")).size(12))
                        .on_press(Message::EditProfile(profile.path.clone()))
                        .padding([6, 12])
                        .style(button::secondary),
                );
            } else {
                actions = actions.push(
                    container(text("ACTIVE").size(10).font(bold_font))
                        .padding([4, 8])
                        .style(|_theme| container::Style {
                            background: Some(Color::from_rgb(0.2, 0.6, 0.2).into()),
                            border: iced::Border {
                                radius: 4.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                );
            }

            let profile_card = container(
                row![
                    column![
                        text(&profile.name).size(18).font(bold_font),
                        text(profile.path.to_string_lossy().to_string())
                            .size(12)
                            .style(|_theme| text::Style {
                                color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                            }),
                    ]
                    .width(Length::Fill),
                    actions
                ]
                .align_y(Alignment::Center),
            )
            .padding(15)
            .style(move |_theme| container::Style {
                background: Some(Color::from_rgba(0.1, 0.1, 0.1, 0.3).into()),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: if is_active { 1.0 } else { 0.0 },
                    color: Color::from_rgb(0.3, 0.6, 0.3),
                },
                ..Default::default()
            });

            profiles_list = profiles_list.push(profile_card);
        }
    }

    column![
        header,
        Space::new().height(24),
        import_panel,
        Space::new().height(24),
        Scrollable::new(profiles_list).height(Length::Fill)
    ]
    .into()
}
