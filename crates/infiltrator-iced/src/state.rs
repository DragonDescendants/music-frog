use crate::tray::TrayManager;
use crate::types::{Route, ToastStatus};
use iced::Theme;
use iced::widget::text_editor;
use infiltrator_desktop::MihomoRuntime;
use mihomo_api::{ConnectionSnapshot, Rule, TrafficData};
use mihomo_config::Profile;
use mihomo_version::manager::VersionInfo;
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::Arc;

pub struct AppState {
    pub current_route: Route,
    pub runtime: Option<Arc<MihomoRuntime>>,
    pub is_starting: bool,
    pub error_msg: Option<String>,
    pub profiles: Vec<Profile>,
    pub is_loading_profiles: bool,
    pub proxies: HashMap<String, mihomo_api::Proxy>,
    pub is_loading_proxies: bool,
    pub proxy_filter: String,
    pub proxy_sort_by_delay: bool,
    pub traffic: Option<TrafficData>,
    pub traffic_history: VecDeque<(u64, u64)>,
    pub memory: Option<mihomo_api::MemoryData>,
    pub public_ip: Option<String>,
    pub connections: Option<ConnectionSnapshot>,
    pub logs: VecDeque<String>,
    pub log_level: String,
    pub lang: String,
    pub proxy_mode: Option<String>,
    pub tun_enabled: Option<bool>,
    pub tun_stack: String,
    pub tun_auto_route: bool,
    pub tun_strict_route: bool,
    pub sniffer_enabled: bool,
    pub rules: Vec<Rule>,
    pub rules_filter: String,
    pub is_loading_rules: bool,
    pub new_rule_type: String,
    pub new_rule_payload: String,
    pub new_rule_target: String,
    pub is_adding_rule: bool,
    pub proxy_providers: Vec<mihomo_api::ProxyProvider>,
    pub rule_providers: Vec<mihomo_api::RuleProvider>,
    pub is_loading_providers: bool,
    pub tray_manager: Option<TrayManager>,
    pub dns_nameservers: Vec<String>,
    pub dns_fallback_servers: Vec<String>,
    pub dns_enhanced_mode: String,
    pub is_saving_dns: bool,
    pub import_url: String,
    pub import_name: String,
    pub is_importing: bool,
    pub webdav_url: String,
    pub webdav_user: String,
    pub webdav_pass: String,
    pub is_syncing: bool,
    pub is_admin: bool,
    pub system_proxy_enabled: bool,
    pub autostart_enabled: bool,
    pub installed_kernels: Vec<VersionInfo>,
    pub latest_core_version: Option<String>,
    pub download_progress: f32,
    pub is_checking_update: bool,
    pub toasts: Vec<(String, ToastStatus)>,
    pub theme: Theme,
    pub editor_content: text_editor::Content,
    pub editor_path: Option<PathBuf>,
}
