use crate::state::AppState;
use crate::types::{Message, Route, ToastStatus};
use crate::view;
use iced::widget::{column, container, row, stack, text};
use iced::{Alignment, Border, Color, Element, Length, Theme};

impl AppState {
    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = view::sidebar::sidebar(self);
        let content: Element<Message> = match self.current_route {
            Route::Overview => view::overview::view(self),
            Route::Profiles => view::profiles::view(self),
            Route::Proxies => view::proxies::view(self),
            Route::Runtime => view::runtime::view(self),
            Route::Rules => view::rules::view(self),
            Route::Dns => view::dns::view(self),
            Route::Sync => view::sync::view(self),
            Route::Editor => view::editor::view(self),
            Route::Settings => view::settings::view(self),
        };

        // Apply transition effect
        let animated_content = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |theme: &Theme| {
                let _base = theme.palette().background;
                // Simple fade-in by interpolating between background and "actual" visibility is hard without native opacity,
                // but we can at least use the opacity to control the container's alpha if supported by renderer,
                // or just leave it as-is for now if renderer doesn't support complex alpha.
                // In Iced 0.14 with tiny-skia, we can use alpha in colors.
                container::Style {
                    text_color: Some(Color {
                        a: self.transition.opacity,
                        ..theme.palette().text
                    }),
                    ..Default::default()
                }
            });

        let main_content = container(animated_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40);

        let main_view = row![sidebar, main_content];

        if self.toasts.is_empty() {
            main_view.into()
        } else {
            let mut toast_column = column![].spacing(10);
            for (content, status) in &self.toasts {
                let color = match status {
                    ToastStatus::Info => Color::from_rgb(0.2, 0.5, 0.8),
                    ToastStatus::Success => Color::from_rgb(0.2, 0.7, 0.2),
                    ToastStatus::Warning => Color::from_rgb(0.8, 0.5, 0.2),
                    ToastStatus::Error => Color::from_rgb(0.8, 0.2, 0.2),
                };

                toast_column = toast_column.push(
                    container(text(content.clone()).size(13))
                        .padding([10, 20])
                        .style(move |_| container::Style {
                            background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()),
                            border: Border {
                                radius: 8.0.into(),
                                width: 1.0,
                                color,
                            },
                            ..Default::default()
                        }),
                );
            }

            stack![
                main_view,
                container(toast_column)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(20)
                    .align_x(Alignment::End)
                    .align_y(Alignment::End)
            ]
            .into()
        }
    }
}
