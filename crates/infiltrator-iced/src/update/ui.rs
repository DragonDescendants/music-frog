use crate::state::AppState;
use crate::types::{Message, Route};
use iced::{window, Task, Theme};

impl AppState {
    pub fn update_ui(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(route) => {
                self.current_route = route;
                if route == Route::Proxies || route == Route::Overview {
                    return Task::done(Message::LoadProxies);
                }
                if route == Route::Overview || route == Route::Runtime {
                    return Task::done(Message::FetchIpInfo);
                }
                if route == Route::Rules {
                    return Task::batch(vec![
                        Task::done(Message::LoadRules),
                        Task::done(Message::LoadProviders),
                    ]);
                }
                Task::none()
            }
            Message::ToggleTheme => {
                self.theme = if self.theme == Theme::Dark { Theme::Light } else { Theme::Dark };
                Task::none()
            }
            Message::WindowClosed(id) => {
                let current_route = self.current_route;
                window::close(id).map(move |_: ()| Message::Navigate(current_route))
            }
            Message::HideWindow => {
                let current_route = self.current_route;
                window::latest().then(move |id| {
                    if let Some(id) = id { window::close(id).map(move |_: ()| Message::Navigate(current_route)) }
                    else { Task::none() }
                })
            }
            Message::ShowWindow => {
                window::latest().then(move |id| {
                    if let Some(id) = id { window::gain_focus(id) }
                    else {
                        let (_, task) = window::open(window::Settings { size: (1000.0, 700.0).into(), ..Default::default() });
                        task.map(|_: window::Id| Message::Navigate(Route::Overview))
                    }
                })
            }
            Message::Exit => {
                let rt = self.runtime.take();
                Task::perform(
                    async move { if let Some(r) = rt { let _ = r.shutdown().await; } },
                    |_| Message::ProxyStopped,
                ).then(|_| { std::process::exit(0); })
            }
            Message::ShowToast(content, status) => {
                self.toasts.push((content, status));
                let index = self.toasts.len() - 1;
                Task::perform(
                    async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        index
                    },
                    Message::RemoveToast
                )
            }
            Message::RemoveToast(index) => {
                if index < self.toasts.len() {
                    self.toasts.remove(index);
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }
}
