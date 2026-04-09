use crate::state::AppState;
use crate::types::Message;
use iced::{Subscription, stream};
use muda::MenuEvent;
use std::time::Duration;
use tray_icon::TrayIconEvent;

impl AppState {
    pub fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![];

        // 1. Tray Events
        subs.push(Subscription::run(|| {
            stream::channel(
                100,
                |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                    loop {
                        if let Ok(event) = TrayIconEvent::receiver().try_recv() {
                            let _ = output.try_send(Message::TrayIconEvent(event));
                        }
                        if let Ok(event) = MenuEvent::receiver().try_recv() {
                            let _ = output.try_send(Message::MenuEvent(event));
                        }
                        tokio::time::sleep(Duration::from_millis(10)).await;
                    }
                },
            )
        }));

        // 2. Real-time Status Refresh (When running)
        if self.runtime.is_some() {
            subs.push(Subscription::run(|| {
                stream::channel(
                    100,
                    |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                        let mut traffic_interval =
                            tokio::time::interval(Duration::from_millis(1000));
                        let mut sync_interval = tokio::time::interval(Duration::from_secs(3600)); // WebDAV sync hourly

                        loop {
                            tokio::select! {
                                _ = traffic_interval.tick() => {
                                    // Actual data comes from stream, this is just a heartbeat if needed
                                }
                                _ = sync_interval.tick() => {
                                    let _ = output.try_send(Message::TickWebDavSync);
                                }
                            }
                        }
                    },
                )
            }));
        }

        // 3. Transition Animation Tick
        if self.transition.is_animating {
            subs.push(
                iced::time::every(Duration::from_millis(16)).map(|_| Message::TickTransition),
            );
        }

        Subscription::batch(subs)
    }
}
