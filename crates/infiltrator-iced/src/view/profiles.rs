use crate::locales::{Lang, Localizer};
use crate::{AppState, Message};
use iced::widget::{Scrollable, Space, button, column, row, text};
use iced::{Alignment, Color, Element, Length};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let lang = Lang(&state.lang);
    let bold_font = iced::Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let header = row![
        text(lang.tr("profiles_title")).size(24).font(bold_font),
        Space::new().width(Length::Fill),
        button(text(lang.tr("refresh")).size(12))
            .on_press(Message::LoadProfiles)
            .padding([6, 12])
    ]
    .align_y(Alignment::Center);

    let mut profiles_list = column![].spacing(12);
    if state.is_loading_profiles {
        profiles_list = profiles_list.push(text(lang.tr("loading_profiles")));
    } else if state.profiles.is_empty() {
        profiles_list = profiles_list.push(text(lang.tr("no_profiles")));
    } else {
        for p in &state.profiles {
            let is_active = p.active;

            let profile_card = button(
                row![
                    column![
                        text(&p.name).size(16).font(bold_font),
                        text(p.path.to_string_lossy())
                            .size(12)
                            .style(|_theme| text::Style {
                                color: Some(Color::from_rgb(0.5, 0.5, 0.5))
                            })
                    ]
                    .spacing(4),
                    Space::new().width(Length::Fill),
                    if is_active {
                        text("ACTIVE").size(10).font(bold_font)
                    } else {
                        text("")
                    }
                ]
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding(16)
            .style(move |theme, status| {
                let mut style = button::secondary(theme, status);
                style.background = Some(Color::from_rgb(0.12, 0.12, 0.12).into());
                if status == iced::widget::button::Status::Hovered {
                    style.background = Some(Color::from_rgb(0.15, 0.15, 0.15).into());
                }
                style.border.radius = 10.0.into();
                if is_active {
                    style.border.width = 1.0;
                    style.border.color = theme.palette().primary;
                }
                style
            });

            let profile_item = if !is_active {
                profile_card.on_press(Message::SetActiveProfile(p.name.clone()))
            } else {
                profile_card
            };

            profiles_list = profiles_list.push(profile_item);
        }
    }

    column![
        header,
        Space::new().height(24),
        Scrollable::new(profiles_list).height(Length::Fill)
    ]
    .into()
}
