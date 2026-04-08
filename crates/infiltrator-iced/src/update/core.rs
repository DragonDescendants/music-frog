use crate::autostart;
use crate::state::AppState;
use crate::types::{Message, RuntimeConfig, ToastStatus};
use iced::{Task, stream};
use infiltrator_desktop::MihomoRuntime;
use mihomo_version::VersionManager;
use std::sync::Arc;

impl AppState {
    pub fn update_core(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FetchIpInfo => Task::perform(
                async {
                    let client = reqwest::Client::builder()
                        .timeout(std::time::Duration::from_secs(5))
                        .build()
                        .map_err(|e| e.to_string())?;
                    let resp = client
                        .get("https://api.ipify.org")
                        .send()
                        .await
                        .map_err(|e| e.to_string())?
                        .text()
                        .await
                        .map_err(|e| e.to_string())?;
                    Ok(resp)
                },
                Message::IpInfoReceived,
            ),
            Message::StartProxy => {
                self.is_starting = true;
                self.error_msg = None;
                Task::perform(
                    async {
                        let vm = VersionManager::new()
                            .map_err(|e: mihomo_api::MihomoError| e.to_string())?;
                        let data_dir = mihomo_platform::get_home_dir()
                            .map_err(|e: mihomo_api::MihomoError| e.to_string())?;
                        let candidates = vec![];
                        let r = MihomoRuntime::bootstrap(&vm, true, &candidates, &data_dir)
                            .await
                            .map_err(|e: anyhow::Error| e.to_string())?;
                        Ok(Arc::new(r))
                    },
                    Message::ProxyStarted,
                )
            }
            Message::StopProxy => {
                let rt = self.runtime.take();
                Task::perform(
                    async move {
                        if let Some(r) = rt {
                            let _ = r.shutdown().await;
                        }
                    },
                    |_| Message::ProxyStopped,
                )
            }
            Message::ProxyStarted(result) => {
                self.is_starting = false;
                match result {
                    Ok(runtime) => {
                        self.runtime = Some(runtime);
                        Task::batch(vec![
                            Task::done(Message::FetchRuntimeConfig),
                            Task::done(Message::LoadProxies),
                            Task::done(Message::FetchIpInfo),
                        ])
                    }
                    Err(e) => {
                        self.error_msg = Some(e);
                        Task::none()
                    }
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
            Message::LoadProxies => {
                if let Some(rt) = self.runtime.clone() {
                    self.is_loading_proxies = true;
                    Task::perform(
                        async move { rt.client().get_proxies().await.map_err(|e| e.to_string()) },
                        Message::ProxiesLoaded,
                    )
                } else {
                    Task::none()
                }
            }
            Message::ProxiesLoaded(result) => {
                self.is_loading_proxies = false;
                match result {
                    Ok(proxies) => {
                        if let Some(tm) = &self.tray_manager {
                            tm.update_groups(&proxies);
                        }
                        self.proxies = proxies;
                    }
                    Err(e) => self.error_msg = Some(e),
                }
                Task::none()
            }
            Message::SelectProxy(group, name) => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .switch_proxy(&group, &name)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        |_| Message::LoadProxies,
                    )
                } else {
                    Task::none()
                }
            }
            Message::FilterProxies(filter) => {
                self.proxy_filter = filter;
                Task::none()
            }
            Message::ToggleProxySort => {
                self.proxy_sort_by_delay = !self.proxy_sort_by_delay;
                Task::none()
            }
            Message::TrafficReceived(data) => {
                self.traffic = Some(data.clone());
                self.traffic_history.push_back((data.up, data.down));
                if self.traffic_history.len() > 60 {
                    self.traffic_history.pop_front();
                }
                Task::none()
            }
            Message::MemoryReceived(data) => {
                self.memory = Some(data);
                Task::none()
            }
            Message::IpInfoReceived(result) => {
                match result {
                    Ok(ip) => self.public_ip = Some(ip),
                    Err(_) => self.public_ip = Some("Failed to fetch IP".to_string()),
                }
                Task::none()
            }
            Message::ConnectionsReceived(data) => {
                self.connections = Some(data);
                Task::none()
            }
            Message::LogReceived(log) => {
                self.logs.push_back(log);
                if self.logs.len() > 500 {
                    self.logs.pop_front();
                }
                iced::widget::operation::snap_to(
                    iced::widget::Id::new("log_scroller"),
                    iced::widget::scrollable::RelativeOffset::END,
                )
            }
            Message::FetchRuntimeConfig => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            let mode = rt.current_mode().await.unwrap_or_default();
                            let config =
                                rt.client().get_config().await.map_err(|e| e.to_string())?;
                            let (dns, fallback, enhanced) = if let Some(d) = config.dns {
                                (
                                    d.nameserver,
                                    d.fallback.unwrap_or_default(),
                                    d.enhanced_mode,
                                )
                            } else {
                                (vec![], vec![], "fake-ip".to_string())
                            };
                            let (tun_en, tun_st, tun_ar, tun_sr) = if let Some(t) = config.tun {
                                (t.enable, t.stack, t.auto_route, t.strict_route)
                            } else {
                                (false, "gvisor".to_string(), true, false)
                            };
                            let sniff = config.sniffer.map(|s| s.enable).unwrap_or(false);
                            Ok(RuntimeConfig {
                                mode,
                                tun_enabled: tun_en,
                                dns_nameservers: dns,
                                dns_fallback: fallback,
                                dns_enhanced_mode: enhanced,
                                tun_stack: tun_st,
                                tun_auto_route: tun_ar,
                                tun_strict_route: tun_sr,
                                sniffer_enabled: sniff,
                            })
                        },
                        Message::RuntimeConfigFetched,
                    )
                } else {
                    Task::none()
                }
            }
            Message::RuntimeConfigFetched(result) => {
                if let Ok(config) = result {
                    self.proxy_mode = Some(config.mode);
                    self.tun_enabled = Some(config.tun_enabled);
                    self.dns_nameservers = config.dns_nameservers;
                    self.dns_fallback_servers = config.dns_fallback;
                    self.dns_enhanced_mode = config.dns_enhanced_mode;
                    self.tun_stack = config.tun_stack;
                    self.tun_auto_route = config.tun_auto_route;
                    self.tun_strict_route = config.tun_strict_route;
                    self.sniffer_enabled = config.sniffer_enabled;
                    if let Some(tm) = &self.tray_manager {
                        tm.update_status(self.system_proxy_enabled, config.tun_enabled);
                    }
                }
                Task::none()
            }
            Message::SetProxyMode(mode) => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.set_mode(&mode)
                                .await
                                .map_err(|e: anyhow::Error| e.to_string())
                        },
                        Message::ModeSetResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SetTunEnabled(enabled) => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.set_tun_enabled(enabled)
                                .await
                                .map_err(|e: anyhow::Error| e.to_string())
                        },
                        Message::ModeSetResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SetTunStack(stack) => {
                self.tun_stack = stack.clone();
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .patch_config(serde_json::json!({"tun": {"stack": stack}}))
                                .await
                                .map_err(|e| e.to_string())
                        },
                        Message::ModeSetResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SetTunAutoRoute(auto) => {
                self.tun_auto_route = auto;
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .patch_config(serde_json::json!({"tun": {"auto-route": auto}}))
                                .await
                                .map_err(|e| e.to_string())
                        },
                        Message::ModeSetResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SetTunStrictRoute(strict) => {
                self.tun_strict_route = strict;
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .patch_config(serde_json::json!({"tun": {"strict-route": strict}}))
                                .await
                                .map_err(|e| e.to_string())
                        },
                        Message::ModeSetResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SetSnifferEnabled(enabled) => {
                self.sniffer_enabled = enabled;
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .patch_config(serde_json::json!({"sniffer": {"enable": enabled}}))
                                .await
                                .map_err(|e| e.to_string())
                        },
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
            Message::CloseAllConnections => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .close_all_connections()
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
            Message::LoadProviders => {
                if let Some(rt) = self.runtime.clone() {
                    self.is_loading_providers = true;
                    Task::perform(
                        async move {
                            let p = rt
                                .client()
                                .get_proxy_providers()
                                .await
                                .map_err(|e| e.to_string())?;
                            let r = rt
                                .client()
                                .get_rule_providers()
                                .await
                                .map_err(|e| e.to_string())?;
                            Ok((
                                p.into_values().collect::<Vec<_>>(),
                                r.into_values().collect::<Vec<_>>(),
                            ))
                        },
                        Message::ProvidersLoaded,
                    )
                } else {
                    Task::none()
                }
            }
            Message::ProvidersLoaded(result) => {
                self.is_loading_providers = false;
                match result {
                    Ok((p, r)) => {
                        self.proxy_providers = p;
                        self.rule_providers = r;
                    }
                    Err(e) => self.error_msg = Some(e),
                }
                Task::none()
            }
            Message::UpdateProxyProvider(name) => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .update_proxy_provider(&name)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        |_| Message::LoadProviders,
                    )
                } else {
                    Task::none()
                }
            }
            Message::UpdateRuleProvider(name) => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .update_rule_provider(&name)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        |_| Message::LoadProviders,
                    )
                } else {
                    Task::none()
                }
            }
            Message::FilterRules(filter) => {
                self.rules_filter = filter;
                Task::none()
            }
            Message::UpdateDnsServer(index, value) => {
                if let Some(server) = self.dns_nameservers.get_mut(index) {
                    *server = value;
                }
                Task::none()
            }
            Message::UpdateDnsEnhancedMode(mode) => {
                self.dns_enhanced_mode = mode;
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
            Message::AddDnsServerTemplate(server) => {
                self.dns_nameservers.push(server);
                Task::none()
            }
            Message::UpdateFallbackDnsServer(index, value) => {
                if let Some(server) = self.dns_fallback_servers.get_mut(index) {
                    *server = value;
                }
                Task::none()
            }
            Message::AddFallbackDnsServer => {
                self.dns_fallback_servers.push(String::new());
                Task::none()
            }
            Message::RemoveFallbackDnsServer(index) => {
                if self.dns_fallback_servers.len() > index {
                    self.dns_fallback_servers.remove(index);
                }
                Task::none()
            }
            Message::SaveDns => {
                if let Some(rt) = self.runtime.clone() {
                    self.is_saving_dns = true;
                    let servers = self.dns_nameservers.clone();
                    let fallbacks = self.dns_fallback_servers.clone();
                    let enhanced = self.dns_enhanced_mode.clone();
                    Task::perform(
                        async move {
                            rt.client().patch_config(serde_json::json!({
                                "dns": { "enable": true, "enhanced-mode": enhanced, "nameserver": servers, "fallback": fallbacks }
                            })).await.map_err(|e| e.to_string())
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
                    self.error_msg = Some(e.clone());
                    return Task::done(Message::ShowToast(e, ToastStatus::Error));
                }
                Task::done(Message::ShowToast(
                    "DNS settings saved".to_string(),
                    ToastStatus::Success,
                ))
            }
            Message::SetAutostart(enabled) => {
                self.autostart_enabled = enabled;
                Task::perform(
                    async move {
                        autostart::set_autostart_enabled(enabled)
                            .map_err(|e: anyhow::Error| e.to_string())
                    },
                    Message::AutostartSet,
                )
            }
            Message::AutostartSet(result) => {
                if let Err(e) = result {
                    self.error_msg = Some(e);
                    self.autostart_enabled = !self.autostart_enabled;
                }
                Task::none()
            }
            Message::SetSystemProxy(enabled) => {
                self.system_proxy_enabled = enabled;
                let endpoint = if enabled {
                    Some("127.0.0.1:7890")
                } else {
                    None
                };
                Task::perform(
                    async move {
                        infiltrator_desktop::proxy::apply_system_proxy(endpoint)
                            .map_err(|e: anyhow::Error| e.to_string())
                    },
                    Message::SystemProxySet,
                )
            }
            Message::SystemProxySet(result) => {
                if let Err(e) = result {
                    self.error_msg = Some(e);
                    self.system_proxy_enabled = !self.system_proxy_enabled;
                }
                if let Some(tm) = &self.tray_manager {
                    tm.update_status(self.system_proxy_enabled, self.tun_enabled.unwrap_or(false));
                }
                Task::none()
            }
            Message::LoadKernels => Task::perform(
                async {
                    let vm = VersionManager::new()
                        .map_err(|e: mihomo_api::MihomoError| e.to_string())?;
                    vm.list_installed()
                        .await
                        .map_err(|e: mihomo_api::MihomoError| e.to_string())
                },
                Message::KernelsLoaded,
            ),
            Message::KernelsLoaded(result) => {
                match result {
                    Ok(kernels) => {
                        self.installed_kernels = kernels;
                    }
                    Err(e) => {
                        self.error_msg = Some(e);
                    }
                }
                Task::none()
            }
            Message::SetDefaultKernel(version) => Task::perform(
                async move {
                    let vm = VersionManager::new()
                        .map_err(|e: mihomo_api::MihomoError| e.to_string())?;
                    vm.set_default(&version)
                        .await
                        .map_err(|e: mihomo_api::MihomoError| e.to_string())
                },
                |_| Message::LoadKernels,
            ),
            Message::CheckCoreUpdate => {
                self.is_checking_update = true;
                Task::perform(
                    async {
                        let info = mihomo_version::channel::fetch_latest(
                            mihomo_version::channel::Channel::Stable,
                        )
                        .await
                        .map_err(|e| e.to_string())?;
                        Ok(info.version)
                    },
                    Message::CoreUpdateInfo,
                )
            }
            Message::CoreUpdateInfo(result) => {
                self.is_checking_update = false;
                match result {
                    Ok(v) => self.latest_core_version = Some(v),
                    Err(e) => self.error_msg = Some(e),
                }
                Task::none()
            }
            Message::DownloadCore(version) => {
                self.download_progress = 0.0;
                Task::run(
                    stream::channel(
                        10,
                        move |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                            use iced::futures::SinkExt;
                            let vm = match VersionManager::new() {
                                Ok(vm) => vm,
                                Err(e) => {
                                    let _ = output
                                        .send(Message::CoreDownloadFinished(Err(e.to_string())))
                                        .await;
                                    return;
                                }
                            };
                            let res = vm
                                .install_with_progress(&version, |p| {
                                    let total = p.total.unwrap_or(0);
                                    let progress = if total > 0 {
                                        p.downloaded as f32 / total as f32
                                    } else {
                                        0.0
                                    };
                                    let _ =
                                        output.try_send(Message::CoreDownloadProgress(progress));
                                })
                                .await;
                            match res {
                                Ok(_) => {
                                    let _ = output
                                        .send(Message::CoreDownloadFinished(Ok(version)))
                                        .await;
                                }
                                Err(e) => {
                                    let _ = output
                                        .send(Message::CoreDownloadFinished(Err(e.to_string())))
                                        .await;
                                }
                            }
                        },
                    ),
                    |msg| msg,
                )
            }
            Message::CoreDownloadProgress(p) => {
                self.download_progress = p;
                Task::none()
            }
            Message::CoreDownloadFinished(result) => {
                self.download_progress = 0.0;
                match result {
                    Ok(_) => Task::batch(vec![
                        Task::done(Message::LoadKernels),
                        Task::done(Message::ShowToast(
                            "Kernel updated successfully".to_string(),
                            ToastStatus::Success,
                        )),
                    ]),
                    Err(e) => {
                        self.error_msg = Some(e.clone());
                        Task::done(Message::ShowToast(e, ToastStatus::Error))
                    }
                }
            }
            Message::DeleteKernel(version) => Task::perform(
                async move {
                    let vm = VersionManager::new()
                        .map_err(|e: mihomo_api::MihomoError| e.to_string())?;
                    vm.uninstall(&version)
                        .await
                        .map_err(|e: mihomo_api::MihomoError| e.to_string())
                },
                |_| Message::LoadKernels,
            ),
            Message::FactoryReset => Task::perform(
                async {
                    let data_dir = mihomo_platform::get_home_dir().map_err(|e| e.to_string())?;
                    if data_dir.exists() {
                        tokio::fs::remove_dir_all(&data_dir)
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                    Ok(())
                },
                |_: Result<(), String>| Message::Exit,
            ),
            Message::OpenConfigDir => Task::perform(
                async {
                    let data_dir = mihomo_platform::get_home_dir().map_err(|e| e.to_string())?;
                    #[cfg(target_os = "windows")]
                    {
                        std::process::Command::new("explorer")
                            .arg(data_dir)
                            .spawn()
                            .map_err(|e| e.to_string())?;
                    }
                    #[cfg(target_os = "macos")]
                    {
                        std::process::Command::new("open")
                            .arg(data_dir)
                            .spawn()
                            .map_err(|e| e.to_string())?;
                    }
                    #[cfg(target_os = "linux")]
                    {
                        std::process::Command::new("xdg-open")
                            .arg(data_dir)
                            .spawn()
                            .map_err(|e| e.to_string())?;
                    }
                    Ok(())
                },
                |res: Result<(), String>| match res {
                    Ok(_) => Message::ShowToast("Folder opened".to_string(), ToastStatus::Info),
                    Err(e) => Message::ShowToast(e, ToastStatus::Error),
                },
            ),
            Message::FlushFakeIpCache => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .patch_config(serde_json::json!({"dns": {"fake-ip-filter": []}}))
                                .await
                                .map_err(|e| e.to_string())?;
                            Ok(())
                        },
                        |_: Result<(), String>| {
                            Message::ShowToast(
                                "Fake-IP Cache Flushed (Filter Reset)".to_string(),
                                ToastStatus::Success,
                            )
                        },
                    )
                } else {
                    Task::none()
                }
            }
            Message::TestProxyDelay(name) => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .test_delay(&name, "http://www.gstatic.com/generate_204", 5000)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        |_| Message::LoadProxies,
                    )
                } else {
                    Task::none()
                }
            }
            Message::TestGroupDelay(group_name) => {
                if let Some(rt) = self.runtime.clone() {
                    let name = group_name.clone();
                    Task::perform(
                        async move {
                            let proxies =
                                rt.client().get_proxies().await.map_err(|e| e.to_string())?;
                            let members = proxies
                                .get(&name)
                                .and_then(|p| p.all())
                                .map(|all| all.to_vec())
                                .unwrap_or_default();
                            for m in members {
                                let _ = rt
                                    .client()
                                    .test_delay(&m, "http://www.gstatic.com/generate_204", 3000)
                                    .await;
                            }
                            Ok(())
                        },
                        |_: Result<(), String>| Message::LoadProxies,
                    )
                } else {
                    Task::none()
                }
            }
            _ => Task::none(),
        }
    }
}
