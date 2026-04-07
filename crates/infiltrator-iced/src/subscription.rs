use crate::state::AppState;
use crate::types::Message;
use iced::{stream, Subscription};
use tray_icon::TrayIconEvent;
use muda::MenuEvent;

impl AppState {
    pub fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![];

        subs.push(iced::event::listen_with(|event, _status, id| {
            if let iced::Event::Window(iced::window::Event::CloseRequested) = event {
                Some(Message::WindowClosed(id))
            } else {
                None
            }
        }));

        subs.push(Subscription::run_with("tray_events", |_| {
            stream::channel(
                10,
                |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                    let tray_channel = TrayIconEvent::receiver();
                    let menu_channel = MenuEvent::receiver();
                    loop {
                        tokio::select! {
                            Ok(event) = async { tray_channel.recv() } => {
                                let _ = output.try_send(Message::TrayIconEvent(event));
                            }
                            Ok(event) = async { menu_channel.recv() } => {
                                let _ = output.try_send(Message::MenuEvent(event));
                            }
                        }
                    }
                },
            )
        }));

        if let Some(rt) = &self.runtime {
            let url1 = rt.controller_url.clone();
            subs.push(Subscription::run_with((url1.clone(), "traffic"), move |(u, _)| {
                let c = mihomo_api::MihomoClient::new(u, None).unwrap();
                stream::channel(
                    100,
                    move |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                        loop {
                            if let Ok(mut rx) = c.stream_traffic().await {
                                while let Some(msg) = rx.recv().await {
                                    let _ = output.try_send(Message::TrafficReceived(msg));
                                }
                            }
                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                        }
                    },
                )
            }));

            let url2 = rt.controller_url.clone();
            subs.push(Subscription::run_with((url2.clone(), "connections"), move |(u, _)| {
                let c = mihomo_api::MihomoClient::new(u, None).unwrap();
                stream::channel(
                    100,
                    move |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                        loop {
                            if let Ok(mut rx) = c.stream_connections().await {
                                while let Some(msg) = rx.recv().await {
                                    let _ = output.try_send(Message::ConnectionsReceived(msg));
                                }
                            }
                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                        }
                    },
                )
            }));

            let url3 = rt.controller_url.clone();
            let level = self.log_level.clone();
            subs.push(Subscription::run_with((url3.clone(), "logs", level), move |(u, _, l)| {
                let c = mihomo_api::MihomoClient::new(u, None).unwrap();
                let l_str = l.clone();
                stream::channel(
                    100,
                    move |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                        loop {
                            if let Ok(mut rx) = c.stream_logs(Some(&l_str)).await {
                                while let Some(msg) = rx.recv().await {
                                    let _ = output.try_send(Message::LogReceived(msg));
                                }
                            }
                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                        }
                    },
                )
            }));

            let url4 = rt.controller_url.clone();
            subs.push(Subscription::run_with((url4.clone(), "memory"), move |(u, _)| {
                let c = mihomo_api::MihomoClient::new(u, None).unwrap();
                stream::channel(
                    10,
                    move |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                        loop {
                            if let Ok(memory) = c.get_memory().await {
                                let _ = output.try_send(Message::MemoryReceived(memory));
                            }
                            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        }
                    },
                )
            }));
        }

        subs.push(Subscription::run_with("scheduler", |_| {
            stream::channel(
                10,
                |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                    let mut sub_interval = tokio::time::interval(tokio::time::Duration::from_secs(3600));
                    let mut sync_interval = tokio::time::interval(tokio::time::Duration::from_secs(1800));
                    loop {
                        tokio::select! {
                            _ = sub_interval.tick() => { let _ = output.try_send(Message::TickSubUpdate); }
                            _ = sync_interval.tick() => { let _ = output.try_send(Message::TickWebDavSync); }
                        }
                    }
                },
            )
        }));

        Subscription::batch(subs)
    }
}
