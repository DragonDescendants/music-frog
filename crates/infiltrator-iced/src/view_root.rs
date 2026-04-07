use crate::state::AppState;
use crate::types::{Message, Route, ToastStatus};
use crate::view;
use iced::widget::{column, container, row, stack, text};
use iced::{Alignment, Border, Color, Element, Length};

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

        let main_content = container(content)
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
