use iced::{widget::text_editor, window};
pub use infiltrator_core::error::InfiltratorError;
use infiltrator_desktop::MihomoRuntime;
use mihomo_api::{ConnectionSnapshot, Rule, TrafficData};
use mihomo_config::Profile;
use mihomo_version::manager::VersionInfo;
use muda::MenuEvent;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tray_icon::TrayIconEvent;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Route {
    #[default]
    Overview,
    Profiles,
    Proxies,
    Runtime,
    Rules,
    Dns,
    Sync,
    Editor,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastStatus {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    pub previous_route: Option<Route>,
    pub start_time: Option<Instant>,
    pub duration: std::time::Duration,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            previous_route: None,
            start_time: None,
            duration: std::time::Duration::from_millis(300),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum RuntimeStatus {
    #[default]
    Stopped,
    Starting,
    Running,
    Error(InfiltratorError),
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeConfig {
    pub mode: String,
    pub tun_enabled: bool,
    pub dns_nameservers: Vec<String>,
    pub dns_fallback: Vec<String>,
    pub dns_enhanced_mode: String,
    pub tun_stack: String,
    pub tun_auto_route: bool,
    pub tun_strict_route: bool,
    pub sniffer_enabled: bool,
}

#[derive(Clone)]
pub enum Message {
    Navigate(Route),
    StartProxy,
    StopProxy,
    ProxyStarted(Result<Arc<MihomoRuntime>, InfiltratorError>),
    ProxyStopped,
    LoadProfiles,
    ProfilesLoaded(Result<Vec<Profile>, InfiltratorError>),
    SetActiveProfile(String),
    UpdateImportUrl(String),
    UpdateImportName(String),
    ImportProfile,
    ProfileImported(Result<(), InfiltratorError>),
    LoadProxies,
    ProxiesLoaded(Result<HashMap<String, mihomo_api::Proxy>, InfiltratorError>),
    SelectProxy(String, String),
    FilterProxies(String),
    ToggleProxySort,
    TrafficReceived(TrafficData),
    MemoryReceived(mihomo_api::MemoryData),
    IpInfoReceived(Result<String, InfiltratorError>, usize),
    ConnectionsReceived(ConnectionSnapshot),
    LogReceived(String),
    SetLogLevel(String),
    CloseConnection(String),
    CloseAllConnections,
    FetchRuntimeConfig,
    FetchIpInfo,
    RuntimeConfigFetched(Result<RuntimeConfig, InfiltratorError>),
    SetProxyMode(String),
    SetTunEnabled(bool),
    SetTunStack(String),
    SetTunAutoRoute(bool),
    SetTunStrictRoute(bool),
    SetSnifferEnabled(bool),
    ModeSetResult(Result<(), InfiltratorError>),
    OperationResult(Result<(), InfiltratorError>),
    LoadRules,
    RulesLoaded(Result<Vec<Rule>, InfiltratorError>),
    LoadProviders,
    ProvidersLoaded(
        Result<
            (
                Vec<mihomo_api::ProxyProvider>,
                Vec<mihomo_api::RuleProvider>,
            ),
            InfiltratorError,
        >,
    ),
    UpdateProxyProvider(String),
    UpdateRuleProvider(String),
    FilterRules(String),
    UpdateFilteredGroups,
    UpdateNewRuleType(String),
    UpdateNewRulePayload(String),
    UpdateNewRuleTarget(String),
    AddCustomRule,
    RuleAdded(Result<(), InfiltratorError>),
    TickSubUpdate,
    TickWebDavSync,
    TickFrame(Instant),
    TrayIconEvent(TrayIconEvent),
    MenuEvent(MenuEvent),
    Exit,
    UpdateDnsServer(usize, String),
    UpdateDnsEnhancedMode(String),
    AddDnsServer,
    AddDnsServerTemplate(String),
    RemoveDnsServer(usize),
    UpdateFallbackDnsServer(usize, String),
    AddFallbackDnsServer,
    RemoveFallbackDnsServer(usize),
    SaveDns,
    DnsSaved(Result<(), InfiltratorError>),
    SetAutostart(bool),
    AutostartSet(Result<(), InfiltratorError>),
    UpdateWebDavUrl(String),
    UpdateWebDavUser(String),
    UpdateWebDavPass(String),
    SyncUpload,
    SyncDownload,
    SyncFinished(Result<(), InfiltratorError>),
    SetSystemProxy(bool),
    SystemProxySet(Result<(), InfiltratorError>),
    RequestAdminPrivilege,
    EditProfile(PathBuf),
    ProfileContentLoaded(Result<(PathBuf, String), InfiltratorError>),
    EditorAction(text_editor::Action),
    SaveProfile,
    ProfileSaved(Result<(), InfiltratorError>),
    LoadKernels,
    KernelsLoaded(Result<Vec<VersionInfo>, InfiltratorError>),
    CheckCoreUpdate,
    CoreUpdateInfo(Result<String, InfiltratorError>), // Latest version string
    DownloadCore(String),
    CoreDownloadProgress(f32),
    CoreDownloadFinished(Result<String, InfiltratorError>),
    DeleteKernel(String),
    SetDefaultKernel(String),
    FactoryReset,
    OpenConfigDir,
    FlushFakeIpCache,
    TestProxyDelay(String),
    TestGroupDelay(String),
    ProxyTested(String, Result<u64, InfiltratorError>),
    WindowClosed(window::Id),
    HideWindow,
    ShowWindow,
    ToggleTheme,
    ShowToast(String, ToastStatus),
    RemoveToast(usize),
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Navigate(route) => write!(f, "Navigate({:?})", route),
            Message::StartProxy => write!(f, "StartProxy"),
            Message::StopProxy => write!(f, "StopProxy"),
            Message::ProxyStarted(Ok(_)) => write!(f, "ProxyStarted(Ok)"),
            Message::ProxyStarted(Err(e)) => write!(f, "ProxyStarted(Err({:?}))", e),
            Message::ProxyStopped => write!(f, "ProxyStopped"),
            Message::LoadProfiles => write!(f, "LoadProfiles"),
            Message::ProfilesLoaded(Ok(p)) => write!(f, "ProfilesLoaded(Ok({} profiles))", p.len()),
            Message::ProfilesLoaded(Err(e)) => write!(f, "ProfilesLoaded(Err({:?}))", e),
            Message::SetActiveProfile(name) => write!(f, "SetActiveProfile({})", name),
            Message::UpdateImportUrl(url) => write!(f, "UpdateImportUrl({})", url),
            Message::UpdateImportName(name) => write!(f, "UpdateImportName({})", name),
            Message::ImportProfile => write!(f, "ImportProfile"),
            Message::ProfileImported(Ok(_)) => write!(f, "ProfileImported(Ok)"),
            Message::ProfileImported(Err(e)) => write!(f, "ProfileImported(Err({:?}))", e),
            Message::LoadProxies => write!(f, "LoadProxies"),
            Message::ProxiesLoaded(Ok(p)) => write!(f, "ProxiesLoaded(Ok({} proxies))", p.len()),
            Message::ProxiesLoaded(Err(e)) => write!(f, "ProxiesLoaded(Err({:?}))", e),
            Message::SelectProxy(g, n) => write!(f, "SelectProxy({}, {})", g, n),
            Message::FilterProxies(s) => write!(f, "FilterProxies({})", s),
            Message::ToggleProxySort => write!(f, "ToggleProxySort"),
            Message::TrafficReceived(t) => {
                write!(f, "TrafficReceived(up: {}, down: {})", t.up, t.down)
            }
            Message::MemoryReceived(m) => write!(
                f,
                "MemoryReceived(in_use: {}, os_limit: {})",
                m.in_use, m.os_limit
            ),
            Message::IpInfoReceived(Ok(ip), id) => {
                write!(f, "IpInfoReceived(Ok({}), taskId: {})", ip, id)
            }
            Message::IpInfoReceived(Err(e), id) => {
                write!(f, "IpInfoReceived(Err({:?}), taskId: {})", e, id)
            }
            Message::ConnectionsReceived(c) => write!(
                f,
                "ConnectionsReceived({} connections)",
                c.connections.len()
            ),
            Message::LogReceived(l) => write!(f, "LogReceived({})", l),
            Message::SetLogLevel(l) => write!(f, "SetLogLevel({})", l),
            Message::CloseConnection(id) => write!(f, "CloseConnection({})", id),
            Message::CloseAllConnections => write!(f, "CloseAllConnections"),
            Message::FetchRuntimeConfig => write!(f, "FetchRuntimeConfig"),
            Message::FetchIpInfo => write!(f, "FetchIpInfo"),
            Message::RuntimeConfigFetched(Ok(config)) => {
                write!(
                    f,
                    "RuntimeConfigFetched({}, {}, {} DNS, {} FB, {}, {}, {}, {}, {})",
                    config.mode,
                    config.tun_enabled,
                    config.dns_nameservers.len(),
                    config.dns_fallback.len(),
                    config.dns_enhanced_mode,
                    config.tun_stack,
                    config.tun_auto_route,
                    config.tun_strict_route,
                    config.sniffer_enabled
                )
            }
            Message::RuntimeConfigFetched(Err(e)) => {
                write!(f, "RuntimeConfigFetched(Err({:?}))", e)
            }
            Message::SetProxyMode(m) => write!(f, "SetProxyMode({})", m),
            Message::SetTunEnabled(t) => write!(f, "SetTunEnabled({})", t),
            Message::SetTunStack(s) => write!(f, "SetTunStack({})", s),
            Message::SetTunAutoRoute(a) => write!(f, "SetTunAutoRoute({})", a),
            Message::SetTunStrictRoute(s) => write!(f, "SetTunStrictRoute({})", s),
            Message::SetSnifferEnabled(s) => write!(f, "SetSnifferEnabled({})", s),
            Message::ModeSetResult(Ok(_)) => write!(f, "ModeSetResult(Ok)"),
            Message::ModeSetResult(Err(e)) => write!(f, "ModeSetResult(Err({:?}))", e),
            Message::OperationResult(Ok(_)) => write!(f, "OperationResult(Ok)"),
            Message::OperationResult(Err(e)) => write!(f, "OperationResult(Err({:?}))", e),
            Message::LoadRules => write!(f, "LoadRules"),
            Message::RulesLoaded(Ok(r)) => write!(f, "RulesLoaded(Ok({} rules))", r.len()),
            Message::RulesLoaded(Err(e)) => write!(f, "RulesLoaded(Err({:?}))", e),
            Message::LoadProviders => write!(f, "LoadProviders"),
            Message::ProvidersLoaded(Ok((p, r))) => write!(
                f,
                "ProvidersLoaded(Ok({} proxies, {} rules))",
                p.len(),
                r.len()
            ),
            Message::ProvidersLoaded(Err(e)) => write!(f, "ProvidersLoaded(Err({:?}))", e),
            Message::UpdateProxyProvider(p) => write!(f, "UpdateProxyProvider({})", p),
            Message::UpdateRuleProvider(p) => write!(f, "UpdateRuleProvider({})", p),
            Message::FilterRules(s) => write!(f, "FilterRules({})", s),
            Message::UpdateFilteredGroups => write!(f, "UpdateFilteredGroups"),
            Message::UpdateNewRuleType(s) => write!(f, "UpdateNewRuleType({})", s),
            Message::UpdateNewRulePayload(s) => write!(f, "UpdateNewRulePayload({})", s),
            Message::UpdateNewRuleTarget(s) => write!(f, "UpdateNewRuleTarget({})", s),
            Message::AddCustomRule => write!(f, "AddCustomRule"),
            Message::RuleAdded(Ok(_)) => write!(f, "RuleAdded(Ok)"),
            Message::RuleAdded(Err(e)) => write!(f, "RuleAdded(Err({:?}))", e),
            Message::TickSubUpdate => write!(f, "TickSubUpdate"),
            Message::TickWebDavSync => write!(f, "TickWebDavSync"),
            Message::TickFrame(now) => write!(f, "TickFrame({:?})", now),
            Message::TrayIconEvent(_) => write!(f, "TrayIconEvent"),
            Message::MenuEvent(e) => write!(f, "MenuEvent({:?})", e),
            Message::Exit => write!(f, "Exit"),
            Message::UpdateDnsServer(i, s) => write!(f, "UpdateDnsServer({}, {})", i, s),
            Message::UpdateDnsEnhancedMode(m) => write!(f, "UpdateDnsEnhancedMode({})", m),
            Message::AddDnsServer => write!(f, "AddDnsServer"),
            Message::AddDnsServerTemplate(s) => write!(f, "AddDnsServerTemplate({})", s),
            Message::RemoveDnsServer(i) => write!(f, "RemoveDnsServer({})", i),
            Message::UpdateFallbackDnsServer(i, s) => {
                write!(f, "UpdateFallbackDnsServer({}, {})", i, s)
            }
            Message::AddFallbackDnsServer => write!(f, "AddFallbackDnsServer"),
            Message::RemoveFallbackDnsServer(i) => write!(f, "RemoveFallbackDnsServer({})", i),
            Message::SaveDns => write!(f, "SaveDns"),
            Message::DnsSaved(Ok(_)) => write!(f, "DnsSaved(Ok)"),
            Message::DnsSaved(Err(e)) => write!(f, "DnsSaved(Err({:?}))", e),
            Message::SetAutostart(b) => write!(f, "SetAutostart({})", b),
            Message::AutostartSet(Ok(_)) => write!(f, "AutostartSet(Ok)"),
            Message::AutostartSet(Err(e)) => write!(f, "AutostartSet(Err({:?}))", e),
            Message::UpdateWebDavUrl(s) => write!(f, "UpdateWebDavUrl({})", s),
            Message::UpdateWebDavUser(s) => write!(f, "UpdateWebDavUser({})", s),
            Message::UpdateWebDavPass(_) => write!(f, "UpdateWebDavPass(***)"),
            Message::SyncUpload => write!(f, "SyncUpload"),
            Message::SyncDownload => write!(f, "SyncDownload"),
            Message::SyncFinished(Ok(_)) => write!(f, "SyncFinished(Ok)"),
            Message::SyncFinished(Err(e)) => write!(f, "SyncFinished(Err({:?}))", e),
            Message::SetSystemProxy(b) => write!(f, "SetSystemProxy({})", b),
            Message::SystemProxySet(Ok(_)) => write!(f, "SystemProxySet(Ok)"),
            Message::SystemProxySet(Err(e)) => write!(f, "SystemProxySet(Err({:?}))", e),
            Message::RequestAdminPrivilege => write!(f, "RequestAdminPrivilege"),
            Message::EditProfile(p) => write!(f, "EditProfile({:?})", p),
            Message::ProfileContentLoaded(Ok((p, _))) => {
                write!(f, "ProfileContentLoaded(Ok({:?}))", p)
            }
            Message::ProfileContentLoaded(Err(e)) => {
                write!(f, "ProfileContentLoaded(Err({:?}))", e)
            }
            Message::EditorAction(_) => write!(f, "EditorAction"),
            Message::SaveProfile => write!(f, "SaveProfile"),
            Message::ProfileSaved(Ok(_)) => write!(f, "ProfileSaved(Ok)"),
            Message::ProfileSaved(Err(e)) => write!(f, "ProfileSaved(Err({:?}))", e),
            Message::LoadKernels => write!(f, "LoadKernels"),
            Message::KernelsLoaded(Ok(k)) => write!(f, "KernelsLoaded(Ok({} kernels))", k.len()),
            Message::KernelsLoaded(Err(e)) => write!(f, "KernelsLoaded(Err({:?}))", e),
            Message::CheckCoreUpdate => write!(f, "CheckCoreUpdate"),
            Message::CoreUpdateInfo(Ok(v)) => write!(f, "CoreUpdateInfo(Ok({}))", v),
            Message::CoreUpdateInfo(Err(e)) => write!(f, "CoreUpdateInfo(Err({:?}))", e),
            Message::DownloadCore(v) => write!(f, "DownloadCore({})", v),
            Message::CoreDownloadProgress(p) => {
                write!(f, "CoreDownloadProgress({:.2}%)", p * 100.0)
            }
            Message::CoreDownloadFinished(Ok(v)) => write!(f, "CoreDownloadFinished(Ok({}))", v),
            Message::CoreDownloadFinished(Err(e)) => {
                write!(f, "CoreDownloadFinished(Err({:?}))", e)
            }
            Message::DeleteKernel(v) => write!(f, "DeleteKernel({})", v),
            Message::SetDefaultKernel(v) => write!(f, "SetDefaultKernel({})", v),
            Message::FactoryReset => write!(f, "FactoryReset"),
            Message::OpenConfigDir => write!(f, "OpenConfigDir"),
            Message::FlushFakeIpCache => write!(f, "FlushFakeIpCache"),
            Message::TestProxyDelay(p) => write!(f, "TestProxyDelay({})", p),
            Message::TestGroupDelay(g) => write!(f, "TestGroupDelay({})", g),
            Message::ProxyTested(p, Ok(d)) => write!(f, "ProxyTested({}, Ok({}ms))", p, d),
            Message::ProxyTested(p, Err(e)) => write!(f, "ProxyTested({}, Err({:?}))", p, e),
            Message::WindowClosed(id) => write!(f, "WindowClosed({:?})", id),
            Message::HideWindow => write!(f, "HideWindow"),
            Message::ShowWindow => write!(f, "ShowWindow"),
            Message::ToggleTheme => write!(f, "ToggleTheme"),
            Message::ShowToast(s, st) => write!(f, "ShowToast({}, {:?})", s, st),
            Message::RemoveToast(i) => write!(f, "RemoveToast({})", i),
        }
    }
}
