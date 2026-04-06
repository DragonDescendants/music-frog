mod locales;
mod tray;
mod utils;
mod view;

use crate::locales::{Lang, Localizer, get_system_language};
use crate::tray::TrayManager;
use iced::stream;
use iced::widget::{container, row};
use iced::{Alignment, Border, Color, Element, Length, Subscription, Task, Theme};
use infiltrator_desktop::MihomoRuntime;
use mihomo_api::{ConnectionSnapshot, Rule, TrafficData};
use mihomo_config::{ConfigManager, Profile};
use mihomo_version::VersionManager;
use muda::MenuEvent;
use single_instance::SingleInstance;
use std::sync::Arc;
use tray_icon::TrayIconEvent;

pub fn main() -> iced::Result {
    let instance = SingleInstance::new("com.musicfrog.infiltrator").unwrap();
    if !instance.is_single() {
        eprintln!("Another instance is already running.");
        return Ok(());
    }

    iced::application(AppState::new, AppState::update, AppState::view)
        .title(AppState::title)
        .theme(AppState::theme)
        .subscription(AppState::subscription)
        .run()
}

pub struct AppState {
    pub current_route: Route,
    pub runtime: Option<Arc<MihomoRuntime>>,
    pub is_starting: bool,
    pub error_msg: Option<String>,
    pub profiles: Vec<Profile>,
    pub is_loading_profiles: bool,
    pub traffic: Option<TrafficData>,
    pub connections: Option<ConnectionSnapshot>,
    pub logs: Vec<String>,
    pub log_level: String,
    pub lang: String,
    pub proxy_mode: Option<String>,
    pub tun_enabled: Option<bool>,
    pub rules: Vec<Rule>,
    pub rules_filter: String,
    pub is_loading_rules: bool,
    pub tray_manager: Option<TrayManager>,
    pub dns_nameservers: Vec<String>,
    pub is_saving_dns: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Route {
    #[default]
    Profiles,
    Runtime,
    Rules,
    Dns,
    Settings,
}

#[derive(Clone)]
pub enum Message {
    Navigate(Route),
    StartProxy,
    StopProxy,
    ProxyStarted(Result<Arc<MihomoRuntime>, String>),
    ProxyStopped,
    LoadProfiles,
    ProfilesLoaded(Result<Vec<Profile>, String>),
    SetActiveProfile(String),
    TrafficReceived(TrafficData),
    ConnectionsReceived(ConnectionSnapshot),
    LogReceived(String),
    SetLogLevel(String),
    CloseConnection(String),
    FetchRuntimeConfig,
    RuntimeConfigFetched(Result<(String, bool), String>),
    SetProxyMode(String),
    SetTunEnabled(bool),
    ModeSetResult(Result<(), String>),
    OperationResult(Result<(), String>),
    LoadRules,
    RulesLoaded(Result<Vec<Rule>, String>),
    FilterRules(String),
    TrayIconEvent(TrayIconEvent),
    MenuEvent(MenuEvent),
    Exit,
    UpdateDnsServer(usize, String),
    AddDnsServer,
    RemoveDnsServer(usize),
    SaveDns,
    DnsSaved(Result<(), String>),
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Navigate(route) => write!(f, "Navigate({:?})", route),
            Message::StartProxy => write!(f, "StartProxy"),
            Message::StopProxy => write!(f, "StopProxy"),
            Message::ProxyStarted(Ok(_)) => write!(f, "ProxyStarted(Ok)"),
            Message::ProxyStarted(Err(e)) => write!(f, "ProxyStarted(Err({}))", e),
            Message::ProxyStopped => write!(f, "ProxyStopped"),
            Message::LoadProfiles => write!(f, "LoadProfiles"),
            Message::ProfilesLoaded(Ok(p)) => write!(f, "ProfilesLoaded(Ok({} profiles))", p.len()),
            Message::ProfilesLoaded(Err(e)) => write!(f, "ProfilesLoaded(Err({}))", e),
            Message::SetActiveProfile(name) => write!(f, "SetActiveProfile({})", name),
            Message::TrafficReceived(t) => {
                write!(f, "TrafficReceived(up: {}, down: {})", t.up, t.down)
            }
            Message::ConnectionsReceived(c) => write!(
                f,
                "ConnectionsReceived({} connections)",
                c.connections.len()
            ),
            Message::LogReceived(l) => write!(f, "LogReceived({})", l),
            Message::SetLogLevel(l) => write!(f, "SetLogLevel({})", l),
            Message::CloseConnection(id) => write!(f, "CloseConnection({})", id),
            Message::FetchRuntimeConfig => write!(f, "FetchRuntimeConfig"),
            Message::RuntimeConfigFetched(Ok((m, t))) => {
                write!(f, "RuntimeConfigFetched({}, {})", m, t)
            }
            Message::RuntimeConfigFetched(Err(e)) => write!(f, "RuntimeConfigFetched(Err({}))", e),
            Message::SetProxyMode(m) => write!(f, "SetProxyMode({})", m),
            Message::SetTunEnabled(t) => write!(f, "SetTunEnabled({})", t),
            Message::ModeSetResult(Ok(_)) => write!(f, "ModeSetResult(Ok)"),
            Message::ModeSetResult(Err(e)) => write!(f, "ModeSetResult(Err({}))", e),
            Message::OperationResult(Ok(_)) => write!(f, "OperationResult(Ok)"),
            Message::OperationResult(Err(e)) => write!(f, "OperationResult(Err({}))", e),
            Message::LoadRules => write!(f, "LoadRules"),
            Message::RulesLoaded(Ok(r)) => write!(f, "RulesLoaded(Ok({} rules))", r.len()),
            Message::RulesLoaded(Err(e)) => write!(f, "RulesLoaded(Err({}))", e),
            Message::FilterRules(s) => write!(f, "FilterRules({})", s),
            Message::TrayIconEvent(_) => write!(f, "TrayIconEvent"),
            Message::MenuEvent(e) => write!(f, "MenuEvent({:?})", e),
            Message::Exit => write!(f, "Exit"),
            Message::UpdateDnsServer(i, s) => write!(f, "UpdateDnsServer({}, {})", i, s),
            Message::AddDnsServer => write!(f, "AddDnsServer"),
            Message::RemoveDnsServer(i) => write!(f, "RemoveDnsServer({})", i),
            Message::SaveDns => write!(f, "SaveDns"),
            Message::DnsSaved(Ok(_)) => write!(f, "DnsSaved(Ok)"),
            Message::DnsSaved(Err(e)) => write!(f, "DnsSaved(Err({}))", e),
        }
    }
}

impl AppState {
    fn title(&self) -> String {
        Lang(&self.lang).tr("app_title").to_string()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_route: Route::default(),
                runtime: None,
                is_starting: false,
                error_msg: None,
                profiles: Vec::new(),
                is_loading_profiles: true,
                traffic: None,
                connections: None,
                logs: Vec::new(),
                log_level: "info".to_string(),
                lang: get_system_language(),
                proxy_mode: None,
                tun_enabled: None,
                rules: Vec::new(),
                rules_filter: String::new(),
                is_loading_rules: false,
                tray_manager: Some(TrayManager::new()),
                dns_nameservers: vec!["114.114.114.114".to_string(), "8.8.8.8".to_string()],
                is_saving_dns: false,
            },
            Task::perform(
                async {
                    let cm = ConfigManager::new().map_err(|e| e.to_string())?;
                    cm.list_profiles().await.map_err(|e| e.to_string())
                },
                Message::ProfilesLoaded,
            ),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        if let Some(rt) = &self.runtime {
            let url1 = rt.controller_url.clone();
            let sub_traffic = Subscription::run_with((url1.clone(), "traffic"), move |(u, _)| {
                let c = mihomo_api::MihomoClient::new(u, None).unwrap();
                stream::channel(
                    100,
                    move |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                        loop {
                            if let Ok(mut rx) = c.stream_traffic().await {
                                while let Some(msg) = rx.recv().await {
                                    use iced::futures::SinkExt;
                                    let _ = output.send(Message::TrafficReceived(msg)).await;
                                }
                            }
                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                        }
                    },
                )
            });

            let url2 = rt.controller_url.clone();
            let sub_connections = Subscription::run_with(
                (url2.clone(), "connections"),
                move |(u, _)| {
                    let c = mihomo_api::MihomoClient::new(u, None).unwrap();
                    stream::channel(
                        100,
                        move |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                            loop {
                                if let Ok(mut rx) = c.stream_connections().await {
                                    while let Some(msg) = rx.recv().await {
                                        use iced::futures::SinkExt;
                                        let _ =
                                            output.send(Message::ConnectionsReceived(msg)).await;
                                    }
                                }
                                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                            }
                        },
                    )
                },
            );

            let url3 = rt.controller_url.clone();
            let level = self.log_level.clone();
            let sub_logs = Subscription::run_with(
                (url3.clone(), "logs", level),
                move |(u, _, l)| {
                    let c = mihomo_api::MihomoClient::new(u, None).unwrap();
                    let l_str = l.clone();
                    stream::channel(
                        100,
                        move |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                            loop {
                                if let Ok(mut rx) = c.stream_logs(Some(&l_str)).await {
                                    while let Some(msg) = rx.recv().await {
                                        use iced::futures::SinkExt;
                                        let _ = output.send(Message::LogReceived(msg)).await;
                                    }
                                }
                                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                            }
                        },
                    )
                },
            );

            let sub_tray = Subscription::run_with("tray_events", |_| {
                stream::channel(
                    10,
                    |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                        use iced::futures::SinkExt;
                        let tray_channel = TrayIconEvent::receiver();
                        let menu_channel = MenuEvent::receiver();

                        loop {
                            tokio::select! {
                                Ok(event) = async { tray_channel.recv() } => {
                                    let _ = output.send(Message::TrayIconEvent(event)).await;
                                }
                                Ok(event) = async { menu_channel.recv() } => {
                                    let _ = output.send(Message::MenuEvent(event)).await;
                                }
                            }
                        }
                    },
                )
            });

            Subscription::batch(vec![sub_traffic, sub_connections, sub_logs, sub_tray])
        } else {
            Subscription::run_with("tray_events_no_rt", |_| {
                stream::channel(
                    10,
                    |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                        use iced::futures::SinkExt;
                        let tray_channel = TrayIconEvent::receiver();
                        let menu_channel = MenuEvent::receiver();

                        loop {
                            tokio::select! {
                                Ok(event) = async { tray_channel.recv() } => {
                                    let _ = output.send(Message::TrayIconEvent(event)).await;
                                }
                                Ok(event) = async { menu_channel.recv() } => {
                                    let _ = output.send(Message::MenuEvent(event)).await;
                                }
                            }
                        }
                    },
                )
            })
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(route) => {
                self.current_route = route;
                Task::none()
            }
            Message::StartProxy => {
                self.is_starting = true;
                self.error_msg = None;

                Task::perform(
                    async {
                        let vm = VersionManager::new().map_err(|e| e.to_string())?;
                        let data_dir = std::env::current_dir().unwrap_or_default();
                        let candidates = vec![];
                        let r = MihomoRuntime::bootstrap(&vm, true, &candidates, &data_dir)
                            .await
                            .map_err(|e| e.to_string())?;
                        Ok(Arc::new(r))
                    },
                    Message::ProxyStarted,
                )
            }
            Message::ProxyStarted(result) => {
                self.is_starting = false;
                match result {
                    Ok(runtime) => {
                        self.runtime = Some(runtime);
                        Task::done(Message::FetchRuntimeConfig)
                    }
                    Err(e) => {
                        self.error_msg = Some(e);
                        Task::none()
                    }
                }
            }
            Message::StopProxy => {
                if let Some(rt) = self.runtime.take() {
                    Task::perform(
                        async move {
                            let _ = rt.shutdown().await;
                        },
                        |_| Message::ProxyStopped,
                    )
                } else {
                    Task::none()
                }
            }
            Message::ProxyStopped => {
                self.traffic = None;
                self.connections = None;
                self.logs.clear();
                self.proxy_mode = None;
                self.tun_enabled = None;
                Task::none()
            }
            Message::LoadProfiles => {
                self.is_loading_profiles = true;
                Task::perform(
                    async {
                        let cm = ConfigManager::new().map_err(|e| e.to_string())?;
                        cm.list_profiles().await.map_err(|e| e.to_string())
                    },
                    Message::ProfilesLoaded,
                )
            }
            Message::ProfilesLoaded(result) => {
                self.is_loading_profiles = false;
                if let Ok(profiles) = result {
                    self.profiles = profiles;
                }
                Task::none()
            }
            Message::SetActiveProfile(name) => Task::perform(
                async move {
                    if let Ok(cm) = ConfigManager::new() {
                        let _ = cm.set_current(&name).await;
                    }
                },
                |_| Message::LoadProfiles,
            ),
            Message::TrafficReceived(data) => {
                self.traffic = Some(data);
                Task::none()
            }
            Message::ConnectionsReceived(data) => {
                self.connections = Some(data);
                Task::none()
            }
            Message::LogReceived(log) => {
                self.logs.push(log);
                if self.logs.len() > 500 {
                    self.logs.remove(0);
                }
                Task::none()
            }
            Message::FetchRuntimeConfig => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            let mode = rt.current_mode().await.unwrap_or_default();
                            let config = rt.client().get_config().await.ok();
                            let tun = config
                                .and_then(|c| c.tun)
                                .and_then(|t| t.get("enable").and_then(|v| v.as_bool()))
                                .unwrap_or(false);
                            Ok((mode, tun))
                        },
                        Message::RuntimeConfigFetched,
                    )
                } else {
                    Task::none()
                }
            }
            Message::RuntimeConfigFetched(result) => {
                if let Ok((mode, tun)) = result {
                    self.proxy_mode = Some(mode);
                    self.tun_enabled = Some(tun);
                }
                Task::none()
            }
            Message::SetProxyMode(mode) => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move { rt.set_mode(&mode).await.map_err(|e| e.to_string()) },
                        Message::ModeSetResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SetTunEnabled(enabled) => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move { rt.set_tun_enabled(enabled).await.map_err(|e| e.to_string()) },
                        Message::ModeSetResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::ModeSetResult(result) => {
                if let Err(e) = result {
                    self.error_msg = Some(e);
                } else {
                    return Task::done(Message::FetchRuntimeConfig);
                }
                Task::none()
            }
            Message::SetLogLevel(level) => {
                self.log_level = level;
                self.logs.clear();
                Task::none()
            }
            Message::CloseConnection(id) => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .close_connection(&id)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        Message::OperationResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::OperationResult(result) => {
                if let Err(e) = result {
                    self.error_msg = Some(e);
                }
                Task::none()
            }
            Message::LoadRules => {
                if let Some(rt) = self.runtime.clone() {
                    self.is_loading_rules = true;
                    Task::perform(
                        async move { rt.client().get_rules().await.map_err(|e| e.to_string()) },
                        Message::RulesLoaded,
                    )
                } else {
                    Task::none()
                }
            }
            Message::RulesLoaded(result) => {
                self.is_loading_rules = false;
                match result {
                    Ok(rules) => {
                        self.rules = rules;
                    }
                    Err(e) => {
                        self.error_msg = Some(e);
                    }
                }
                Task::none()
            }
            Message::FilterRules(filter) => {
                self.rules_filter = filter;
                Task::none()
            }
            Message::TrayIconEvent(event) => {
                if let tray_icon::TrayIconEvent::Click {
                    button: tray_icon::MouseButton::Left,
                    ..
                } = event
                {
                    return iced::window::latest().and_then(iced::window::gain_focus);
                }
                Task::none()
            }
            Message::MenuEvent(event) => match event.id.as_ref() {
                "quit" => Task::done(Message::Exit),
                "show" => iced::window::latest().and_then(iced::window::gain_focus),
                _ => Task::none(),
            },
            Message::Exit => {
                std::process::exit(0);
            }
            Message::UpdateDnsServer(index, value) => {
                if let Some(server) = self.dns_nameservers.get_mut(index) {
                    *server = value;
                }
                Task::none()
            }
            Message::AddDnsServer => {
                self.dns_nameservers.push(String::new());
                Task::none()
            }
            Message::RemoveDnsServer(index) => {
                if self.dns_nameservers.len() > index {
                    self.dns_nameservers.remove(index);
                }
                Task::none()
            }
            Message::SaveDns => {
                if let Some(rt) = self.runtime.clone() {
                    self.is_saving_dns = true;
                    let servers = self.dns_nameservers.clone();
                    Task::perform(
                        async move {
                            rt.client()
                                .patch_config(serde_json::json!({
                                    "dns": {
                                        "enable": true,
                                        "nameserver": servers
                                    }
                                }))
                                .await
                                .map_err(|e| e.to_string())
                        },
                        Message::DnsSaved,
                    )
                } else {
                    Task::none()
                }
            }
            Message::DnsSaved(result) => {
                self.is_saving_dns = false;
                if let Err(e) = result {
                    self.error_msg = Some(e);
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let sidebar = view::sidebar::sidebar(self);

        let content: Element<Message> = match self.current_route {
            Route::Profiles => view::profiles::view(self),
            Route::Runtime => view::runtime::view(self),
            Route::Rules => view::rules::view(self),
            Route::Dns => view::dns::view(self),
            Route::Settings => view::settings::view(self),
        };

        let main_content = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40);

        row![sidebar, main_content].into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_navigation() {
        let (mut state, _) = AppState::new();
        assert_eq!(state.current_route, Route::Profiles);

        let _ = state.update(Message::Navigate(Route::Runtime));
        assert_eq!(state.current_route, Route::Runtime);

        let _ = state.update(Message::Navigate(Route::Settings));
        assert_eq!(state.current_route, Route::Settings);
    }

    #[test]
    fn test_rules_filtering() {
        let (mut state, _) = AppState::new();
        state.rules = vec![
            mihomo_api::Rule {
                rule_type: "Domain".into(),
                payload: "google.com".into(),
                proxy: "Proxy".into(),
                size: 0,
            },
            mihomo_api::Rule {
                rule_type: "IP".into(),
                payload: "1.1.1.1".into(),
                proxy: "Direct".into(),
                size: 0,
            },
        ];

        state.update(Message::FilterRules("google".into()));
        assert_eq!(state.rules_filter, "google");

        let filtered: Vec<_> = state
            .rules
            .iter()
            .filter(|r| r.payload.contains(&state.rules_filter))
            .collect();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].payload, "google.com");
    }
}
