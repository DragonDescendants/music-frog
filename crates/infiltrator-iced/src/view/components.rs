use crate::{Message, Route};
use iced::widget::{Space, button, canvas, container, text};
use iced::{Border, Color, Element, Length, Point, Rectangle, Renderer, Theme, border, mouse};
use std::collections::VecDeque;

pub fn nav_button<'a>(label: String, route: Route, current_route: &Route) -> Element<'a, Message> {
    let is_active = route == *current_route;

    button(
        container(text(label).size(14))
            .width(Length::Fill)
            .padding([8, 16]),
    )
    .width(Length::Fill)
    .style(move |_theme, status| {
        let mut style = button::primary(_theme, status);
        if !is_active {
            style.background = None;
            style.text_color = Color::from_rgb(0.6, 0.6, 0.6);
        }
        style
    })
    .on_press(Message::Navigate(route))
    .into()
}

pub fn status_dot<'a>(active: bool) -> Element<'a, Message> {
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

pub fn card<'a, T: Into<Element<'a, Message>>>(content: T) -> Element<'a, Message> {
    container(content)
        .width(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(Color::from_rgba(0.1, 0.1, 0.1, 0.5).into()),
            border: Border {
                radius: border::Radius::from(12.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .padding(20)
        .into()
}

pub struct TrafficChart {
    pub history: VecDeque<(u64, u64)>,
}

impl<Message> canvas::Program<Message> for TrafficChart {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        _renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(_renderer, bounds.size());

        if self.history.len() < 2 {
            return vec![frame.into_geometry()];
        }

        let width = bounds.width;
        let height = bounds.height;
        let max_points = 60;
        let x_step = width / (max_points - 1) as f32;

        let mut max_speed = self
            .history
            .iter()
            .map(|(up, down)| std::cmp::max(*up, *down))
            .max()
            .unwrap_or(1024 * 1024);

        if max_speed < 1024 * 100 {
            max_speed = 1024 * 100;
        }

        let scale = |speed: u64| height - (speed as f32 / max_speed as f32) * height;

        let down_path = canvas::Path::new(|p| {
            p.move_to(Point::new(0.0, height));
            for (i, (_, down)) in self.history.iter().enumerate() {
                p.line_to(Point::new(i as f32 * x_step, scale(*down)));
            }
            p.line_to(Point::new((self.history.len() - 1) as f32 * x_step, height));
            p.close();
        });
        frame.fill(&down_path, Color::from_rgba(0.2, 0.4, 0.8, 0.15));

        let down_line = canvas::Path::new(|p| {
            for (i, (_, down)) in self.history.iter().enumerate() {
                let pt = Point::new(i as f32 * x_step, scale(*down));
                if i == 0 {
                    p.move_to(pt);
                } else {
                    p.line_to(pt);
                }
            }
        });
        frame.stroke(
            &down_line,
            canvas::Stroke::default()
                .with_color(Color::from_rgb(0.3, 0.5, 1.0))
                .with_width(2.0),
        );

        let up_line = canvas::Path::new(|p| {
            for (i, (up, _)) in self.history.iter().enumerate() {
                let pt = Point::new(i as f32 * x_step, scale(*up));
                if i == 0 {
                    p.move_to(pt);
                } else {
                    p.line_to(pt);
                }
            }
        });
        frame.stroke(
            &up_line,
            canvas::Stroke::default()
                .with_color(Color::from_rgb(0.3, 0.8, 0.3))
                .with_width(1.5),
        );

        vec![frame.into_geometry()]
    }
}
