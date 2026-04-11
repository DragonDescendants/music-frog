use crate::state::AppState;
use crate::types::Message;
use iced::{Subscription, stream, window};
use muda::MenuEvent;
use tray_icon::TrayIconEvent;
use std::time::Duration;

impl AppState {
    pub fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![];

        // 1. Tray Events
        subs.push(Subscription::run(|| {
            stream::channel(100, |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                loop {
                    if let Ok(event) = TrayIconEvent::receiver().try_recv() {
                        let _ = output.try_send(Message::TrayIconEvent(event));
                    }
                    if let Ok(event) = MenuEvent::receiver().try_recv() {
                        let _ = output.try_send(Message::MenuEvent(event));
                    }
                    tokio::time::sleep(Duration::from_millis(20)).await;
                }
            })
        }));

        // 2. Real-time Status Refresh
        if self.runtime.is_some() {
            subs.push(Subscription::run(|| {
                stream::channel(100, |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                    let mut traffic_interval = tokio::time::interval(Duration::from_millis(1000));
                    let mut sync_interval = tokio::time::interval(Duration::from_secs(3600));

                    loop {
                        tokio::select! {
                            _ = traffic_interval.tick() => { }
                            _ = sync_interval.tick() => { 
                                let _ = output.try_send(Message::TickWebDavSync); 
                            }
                        }
                    }
                })
            }));
        }

        // 3. 高性能动画订阅：只有正在转场时才开启帧回调
        if self.transition.start_time.is_some() {
            subs.push(window::frames().map(Message::TickFrame));
        }

        Subscription::batch(subs)
    }
}
