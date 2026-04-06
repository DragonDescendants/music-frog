use crate::{Message, Route};
use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, border};

pub fn nav_button<'a>(label: String, route: Route, current_route: &Route) -> Element<'a, Message> {
    let active = *current_route == route;

    let content = row![Space::new().width(4), text(label).size(14)].align_y(Alignment::Center);

    let mut btn = button(content).width(Length::Fill).padding(10);

    if active {
        btn = btn.style(|theme, status| {
            let mut style = button::primary(theme, status);
            style.border.radius = 8.0.into();
            style
        });
    } else {
        btn = btn
            .on_press(Message::Navigate(route))
            .style(|theme, status| {
                let mut style = button::secondary(theme, status);
                style.background = if status == iced::widget::button::Status::Hovered {
                    Some(Color::from_rgba(1.0, 1.0, 1.0, 0.05).into())
                } else {
                    None
                };
                style.border.radius = 8.0.into();
                style.text_color = Color::from_rgb(0.7, 0.7, 0.7);
                style
            });
    }

    container(btn).padding([0, 8]).into()
}

pub fn card<'a, M: 'a>(content: impl Into<Element<'a, M>>) -> Element<'a, M> {
    container(content)
        .padding(20)
        .style(|_theme| container::Style {
            background: Some(Color::from_rgb(0.12, 0.12, 0.12).into()),
            border: Border {
                radius: border::Radius::from(12.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

pub fn status_dot(active: bool) -> Element<'static, Message> {
    let color = if active {
        Color::from_rgb(0.2, 0.8, 0.2)
    } else {
        Color::from_rgb(0.8, 0.2, 0.2)
    };

    container(Space::new().width(8).height(8))
        .style(move |_theme| container::Style {
            background: Some(color.into()),
            border: Border {
                radius: border::Radius::from(4.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}
