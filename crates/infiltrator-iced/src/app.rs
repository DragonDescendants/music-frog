use crate::autostart;
use crate::locales::{get_system_language, Lang, Localizer};
use crate::tray::TrayManager;
use crate::types::{Message, Route};
use crate::state::AppState;
use iced::Task;
use mihomo_config::ConfigManager;

impl AppState {
    pub fn title(&self) -> String {
        Lang(&self.lang).tr("app_title").to_string()
    }

    pub fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }

    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_route: Route::default(),
                runtime: None,
                is_starting: false,
                error_msg: None,
                profiles: Vec::new(),
                is_loading_profiles: true,
                proxies: std::collections::HashMap::new(),
                is_loading_proxies: false,
                traffic: None,
                traffic_history: std::collections::VecDeque::from(vec![(0, 0); 60]),
                memory: None,
                public_ip: None,
                connections: None,
                logs: std::collections::VecDeque::with_capacity(505),
                log_level: "info".to_string(),
                lang: get_system_language(),
                proxy_mode: None,
                tun_enabled: None,
                tun_stack: "gvisor".to_string(),
                tun_auto_route: true,
                tun_strict_route: false,
                sniffer_enabled: false,
                rules: Vec::new(),
                rules_filter: String::new(),
                is_loading_rules: false,
                proxy_providers: Vec::new(),
                rule_providers: Vec::new(),
                is_loading_providers: false,
                tray_manager: Some(TrayManager::new()),
                dns_nameservers: Vec::new(),
                dns_fallback_servers: Vec::new(),
                dns_enhanced_mode: "fake-ip".to_string(),
                is_saving_dns: false,
                import_url: String::new(),
                import_name: String::new(),
                is_importing: false,
                webdav_url: String::new(),
                webdav_user: String::new(),
                webdav_pass: String::new(),
                is_syncing: false,
                is_admin: is_elevated::is_elevated(),
                system_proxy_enabled: infiltrator_desktop::proxy::read_system_proxy_state()
                    .map(|s| s.enabled)
                    .unwrap_or(false),
                autostart_enabled: autostart::is_autostart_enabled(),
                installed_kernels: Vec::new(),
                latest_core_version: None,
                download_progress: 0.0,
                is_checking_update: false,
                toasts: Vec::new(),
                theme: iced::Theme::Dark,
                editor_content: iced::widget::text_editor::Content::new(),
                editor_path: None,
            },
            Task::batch(vec![
                Task::perform(
                    async {
                        let data_dir = mihomo_platform::get_home_dir().unwrap_or_default();
                        let path = data_dir.join("settings.toml");
                        infiltrator_core::settings::load_settings(&path).await.unwrap_or_default()
                    },
                    |_| {
                        Message::Navigate(Route::Overview)
                    }
                ),
                Task::perform(
                    async {
                        if let Ok(cm) = ConfigManager::new() {
                            cm.list_profiles().await.unwrap_or_default()
                        } else {
                            vec![]
                        }
                    },
                    |p| Message::ProfilesLoaded(Ok(p)),
                ),
                Task::done(Message::LoadKernels),
            ]),
        )
    }
}
