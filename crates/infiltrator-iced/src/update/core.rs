use crate::autostart;
use crate::state::AppState;
use crate::types::{InfiltratorError, Message, RuntimeConfig, RuntimeStatus, ToastStatus};
use iced::{Task, stream};
use infiltrator_desktop::MihomoRuntime;
use mihomo_version::{VersionManager, channel::Channel};
use std::sync::Arc;

impl AppState {
    pub fn cancel_all_tasks(&mut self) {
        self.last_task_id += 1;
    }

    pub fn recompute_filtered_groups(&mut self) {
        let mut groups: Vec<_> = self.proxies.iter().filter(|(_, p)| p.is_group()).collect();

        // Sort groups: GLOBAL first, then by type
        groups.sort_by(|(na, pa), (nb, pb)| {
            if *na == "GLOBAL" {
                return std::cmp::Ordering::Less;
            }
            if *nb == "GLOBAL" {
                return std::cmp::Ordering::Greater;
            }
            pa.proxy_type().cmp(pb.proxy_type())
        });

        let mut result = Vec::new();
        for (group_name, group_info) in groups {
            let mut members: Vec<String> =
                group_info.all().map(|all| all.to_vec()).unwrap_or_default();

            // 1. Filter
            if !self.proxy_filter.is_empty() {
                let filter = self.proxy_filter.to_lowercase();
                members.retain(|m| m.to_lowercase().contains(&filter));
            }

            if members.is_empty() && !self.proxy_filter.is_empty() {
                continue;
            }

            // 2. Sort by delay
            if self.proxy_sort_by_delay {
                members.sort_by_key(|m| {
                    self.proxies
                        .get(m)
                        .and_then(|p| p.history().last().map(|h| h.delay))
                        .unwrap_or(u32::MAX)
                });
            }

            result.push((group_name.clone(), members));
        }
        self.filtered_groups = result;
    }

    pub fn update_core(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FetchIpInfo => {
                self.cancel_all_tasks();
                let task_id = self.last_task_id;
                Task::perform(
                    async move {
                        let client = reqwest::Client::builder()
                            .timeout(std::time::Duration::from_secs(5))
                            .build()
                            .map_err(|e| InfiltratorError::Internal(e.to_string()))?;
                        let resp = client
                            .get("https://api.ipify.org")
                            .send()
                            .await
                            .map_err(|e| InfiltratorError::Internal(e.to_string()))?
                            .text()
                            .await
                            .map_err(|e| InfiltratorError::Internal(e.to_string()))?;
                        Ok(resp)
                    },
                    move |res| Message::IpInfoReceived(res, task_id),
                )
            }
            Message::StartProxy => {
                self.cancel_all_tasks();
                self.status = RuntimeStatus::Starting;
                self.error_msg = None;
                Task::perform(
                    async {
                        let vm = VersionManager::new().map_err(|e: mihomo_api::MihomoError| {
                            InfiltratorError::Mihomo(e.to_string())
                        })?;
                        let data_dir = mihomo_platform::get_home_dir().map_err(
                            |e: mihomo_api::MihomoError| InfiltratorError::Mihomo(e.to_string()),
                        )?;
                        let candidates = vec![];
                        let r = MihomoRuntime::bootstrap(&vm, true, &candidates, &data_dir)
                            .await
                            .map_err(|e: anyhow::Error| InfiltratorError::Mihomo(e.to_string()))?;
                        Ok(Arc::new(r))
                    },
                    Message::ProxyStarted,
                )
            }
            Message::StopProxy => {
                self.cancel_all_tasks();
                let rt = self.runtime.take();
                self.status = RuntimeStatus::Stopped;
                Task::perform(
                    async move {
                        if let Some(r) = rt {
                            let _ = r.shutdown().await;
                        }
                    },
                    |_| Message::ProxyStopped,
                )
            }
            Message::ProxyStarted(result) => match result {
                Ok(runtime) => {
                    self.status = RuntimeStatus::Running;
                    self.runtime = Some(runtime);
                    Task::batch(vec![
                        Task::done(Message::FetchRuntimeConfig),
                        Task::done(Message::LoadProxies),
                        Task::done(Message::FetchIpInfo),
                    ])
                }
                Err(e) => {
                    self.status = RuntimeStatus::Error(e.clone());
                    self.error_msg = Some(e.to_string());
                    Task::none()
                }
            },
            Message::ProxyStopped => {
                self.traffic = None;
                self.connections = None;
                self.logs.clear();
                self.proxy_mode = None;
                self.tun_enabled = None;
                self.status = RuntimeStatus::Stopped;
                Task::none()
            }
            Message::LoadProxies => {
                if let Some(rt) = self.runtime.clone() {
                    self.is_loading_proxies = true;
                    Task::perform(
                        async move {
                            rt.client()
                                .get_proxies()
                                .await
                                .map_err(InfiltratorError::from)
                        },
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
                        self.recompute_filtered_groups();
                    }
                    Err(e) => self.error_msg = Some(e.to_string()),
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
                                .map_err(InfiltratorError::from)
                        },
                        |_| Message::LoadProxies,
                    )
                } else {
                    Task::none()
                }
            }
            Message::FilterProxies(filter) => {
                self.proxy_filter = filter;
                Task::done(Message::UpdateFilteredGroups)
            }
            Message::ToggleProxySort => {
                self.proxy_sort_by_delay = !self.proxy_sort_by_delay;
                Task::done(Message::UpdateFilteredGroups)
            }
            Message::UpdateFilteredGroups => {
                self.recompute_filtered_groups();
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
            Message::IpInfoReceived(result, task_id) => {
                if task_id == self.last_task_id {
                    match result {
                        Ok(ip) => self.public_ip = Some(ip),
                        Err(e) => self.error_msg = Some(e.to_string()),
                    }
                }
                Task::none()
            }
            Message::ConnectionsReceived(data) => {
                self.connections = Some(data);
                Task::none()
            }
            Message::LogReceived(log) => {
                self.logs.push_back(log);
                if self.logs.len() > 100 {
                    self.logs.pop_front();
                }
                iced::widget::operation::snap_to(
                    iced::widget::Id::new("log_scroller"),
                    iced::widget::scrollable::RelativeOffset::END,
                )
            }
            Message::SetLogLevel(level) => {
                self.log_level = level.clone();
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .patch_config(serde_json::json!({ "log-level": level }))
                                .await
                                .map_err(InfiltratorError::from)
                        },
                        Message::OperationResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::CloseConnection(id) => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .close_connection(&id)
                                .await
                                .map_err(InfiltratorError::from)
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
                                .map_err(InfiltratorError::from)
                        },
                        Message::OperationResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::FetchRuntimeConfig => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            let config = rt
                                .client()
                                .get_config()
                                .await
                                .map_err(InfiltratorError::from)?;
                            let mode = config.mode;
                            let (tun_en, tun_st, tun_ar, tun_sr) = config
                                .tun
                                .map(|t| (t.enable, t.stack, t.auto_route, t.strict_route))
                                .unwrap_or((false, String::new(), false, false));
                            let (dns, fallback, enhanced) = config
                                .dns
                                .map(|d| {
                                    (
                                        d.nameserver,
                                        d.fallback.unwrap_or_default(),
                                        d.enhanced_mode,
                                    )
                                })
                                .unwrap_or((vec![], vec![], String::new()));
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
                self.proxy_mode = Some(mode.clone());
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .patch_config(serde_json::json!({ "mode": mode }))
                                .await
                                .map_err(InfiltratorError::from)
                        },
                        Message::ModeSetResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::ModeSetResult(result) => match result {
                Ok(_) => Task::done(Message::FetchRuntimeConfig),
                Err(e) => {
                    self.error_msg = Some(e.to_string());
                    Task::none()
                }
            },
            Message::SetTunEnabled(enabled) => {
                self.tun_enabled = Some(enabled);
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .patch_config(serde_json::json!({ "tun": { "enable": enabled } }))
                                .await
                                .map_err(InfiltratorError::from)
                        },
                        Message::OperationResult,
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
                                .patch_config(serde_json::json!({ "tun": { "stack": stack } }))
                                .await
                                .map_err(InfiltratorError::from)
                        },
                        Message::OperationResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SetTunAutoRoute(enabled) => {
                self.tun_auto_route = enabled;
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .patch_config(
                                    serde_json::json!({ "tun": { "auto-route": enabled } }),
                                )
                                .await
                                .map_err(InfiltratorError::from)
                        },
                        Message::OperationResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SetTunStrictRoute(enabled) => {
                self.tun_strict_route = enabled;
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .patch_config(
                                    serde_json::json!({ "tun": { "strict-route": enabled } }),
                                )
                                .await
                                .map_err(InfiltratorError::from)
                        },
                        Message::OperationResult,
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
                                .patch_config(
                                    serde_json::json!({ "sniffer": { "enable": enabled } }),
                                )
                                .await
                                .map_err(InfiltratorError::from)
                        },
                        Message::OperationResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::OperationResult(result) => match result {
                Ok(_) => Task::done(Message::FetchRuntimeConfig),
                Err(e) => {
                    self.error_msg = Some(e.to_string());
                    Task::none()
                }
            },
            Message::LoadRules => {
                if let Some(rt) = self.runtime.clone() {
                    self.is_loading_rules = true;
                    self.is_loading_providers = true;
                    let rt2 = rt.clone();
                    Task::batch(vec![
                        Task::perform(
                            async move {
                                rt.client()
                                    .get_rules()
                                    .await
                                    .map_err(InfiltratorError::from)
                            },
                            Message::RulesLoaded,
                        ),
                        Task::perform(
                            async move {
                                let proxies = rt2
                                    .client()
                                    .get_proxy_providers()
                                    .await
                                    .map_err(InfiltratorError::from)?;
                                let rules = rt2
                                    .client()
                                    .get_rule_providers()
                                    .await
                                    .map_err(InfiltratorError::from)?;
                                Ok((
                                    proxies.into_values().collect(),
                                    rules.into_values().collect(),
                                ))
                            },
                            Message::ProvidersLoaded,
                        ),
                    ])
                } else {
                    Task::none()
                }
            }
            Message::RulesLoaded(result) => {
                self.is_loading_rules = false;
                match result {
                    Ok(rules) => self.rules = rules,
                    Err(e) => self.error_msg = Some(e.to_string()),
                }
                Task::none()
            }
            Message::ProvidersLoaded(result) => {
                self.is_loading_providers = false;
                match result {
                    Ok((proxies, rules)) => {
                        self.proxy_providers = proxies;
                        self.rule_providers = rules;
                    }
                    Err(e) => self.error_msg = Some(e.to_string()),
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
                                .map_err(InfiltratorError::from)
                        },
                        Message::OperationResult,
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
                                .map_err(InfiltratorError::from)
                        },
                        Message::OperationResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::UpdateDnsServer(index, server) => {
                if let Some(target) = self.dns_nameservers.get_mut(index) {
                    *target = server;
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
            Message::AddDnsServerTemplate(server) => {
                self.dns_nameservers.push(server);
                Task::none()
            }
            Message::RemoveDnsServer(index) => {
                if self.dns_nameservers.len() > index {
                    self.dns_nameservers.remove(index);
                }
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
                            rt.client()
                                .patch_config(serde_json::json!({
                                    "dns": { "enable": true, "enhanced-mode": enhanced, "nameserver": servers, "fallback": fallbacks }
                                }))
                                .await
                                .map_err(InfiltratorError::from)
                        },
                        Message::DnsSaved,
                    )
                } else {
                    Task::none()
                }
            }
            Message::DnsSaved(result) => {
                self.is_saving_dns = false;
                match result {
                    Ok(_) => Task::done(Message::ShowToast(
                        "DNS settings saved".to_string(),
                        ToastStatus::Success,
                    )),
                    Err(e) => {
                        self.error_msg = Some(e.to_string());
                        Task::done(Message::ShowToast(e.to_string(), ToastStatus::Error))
                    }
                }
            }
            Message::FlushFakeIpCache => {
                if let Some(rt) = self.runtime.clone() {
                    Task::perform(
                        async move {
                            rt.client()
                                .flush_fakeip_cache()
                                .await
                                .map_err(InfiltratorError::from)
                        },
                        Message::OperationResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::TestProxyDelay(name) => {
                if let Some(rt) = self.runtime.clone() {
                    let n = name.clone();
                    Task::perform(
                        async move {
                            rt.client()
                                .test_delay(&n, "http://www.gstatic.com/generate_204", 3000)
                                .await
                                .map(|d| d as u64)
                                .map_err(InfiltratorError::from)
                        },
                        move |res| Message::ProxyTested(name, res),
                    )
                } else {
                    Task::none()
                }
            }
            Message::ProxyTested(name, result) => match result {
                Ok(delay) => Task::done(Message::ShowToast(
                    format!("{}: {}ms", name, delay),
                    ToastStatus::Success,
                )),
                Err(e) => Task::done(Message::ShowToast(
                    format!("{}: {}", name, e),
                    ToastStatus::Error,
                )),
            },
            Message::TestGroupDelay(name) => {
                if let Some(rt) = self.runtime.clone() {
                    let proxies = self.proxies.clone();
                    Task::perform(
                        async move {
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
                        |_: Result<(), InfiltratorError>| Message::LoadProxies,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SetAutostart(enabled) => {
                self.autostart_enabled = enabled;
                Task::perform(
                    async move {
                        autostart::set_autostart_enabled(enabled)
                            .map_err(|e: anyhow::Error| InfiltratorError::Internal(e.to_string()))
                    },
                    Message::AutostartSet,
                )
            }
            Message::AutostartSet(result) => {
                if let Err(e) = result {
                    self.autostart_enabled = !self.autostart_enabled;
                    self.error_msg = Some(e.to_string());
                }
                Task::none()
            }
            Message::CheckCoreUpdate => {
                self.is_checking_update = true;
                Task::perform(
                    async {
                        let vm = VersionManager::new().map_err(InfiltratorError::from)?;
                        vm.install_channel(Channel::Stable)
                            .await
                            .map_err(InfiltratorError::from)
                    },
                    Message::CoreUpdateInfo,
                )
            }
            Message::CoreUpdateInfo(result) => {
                self.is_checking_update = false;
                match result {
                    Ok(version) => {
                        self.latest_core_version = Some(version);
                        Task::none()
                    }
                    Err(e) => {
                        self.error_msg = Some(e.to_string());
                        Task::none()
                    }
                }
            }
            Message::DownloadCore(version) => {
                let stream = stream::channel(
                    100,
                    move |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                        let vm = match VersionManager::new() {
                            Ok(v) => v,
                            Err(e) => {
                                let _ = output.try_send(Message::CoreDownloadFinished(Err(
                                    InfiltratorError::from(e),
                                )));
                                return;
                            }
                        };

                        match vm.install(&version).await {
                            Ok(_) => {
                                let _ = output.try_send(Message::CoreDownloadFinished(Ok(version)));
                            }
                            Err(e) => {
                                let _ = output.try_send(Message::CoreDownloadFinished(Err(
                                    InfiltratorError::from(e),
                                )));
                            }
                        }
                    },
                );
                Task::run(stream, |m| m)
            }
            Message::CoreDownloadProgress(progress) => {
                self.download_progress = progress;
                Task::none()
            }
            Message::CoreDownloadFinished(result) => {
                self.download_progress = 0.0;
                match result {
                    Ok(_) => Task::done(Message::LoadKernels),
                    Err(e) => {
                        self.error_msg = Some(e.to_string());
                        Task::done(Message::ShowToast(e.to_string(), ToastStatus::Error))
                    }
                }
            }
            Message::LoadKernels => Task::perform(
                async {
                    let vm = VersionManager::new().map_err(InfiltratorError::from)?;
                    vm.list_installed().await.map_err(InfiltratorError::from)
                },
                Message::KernelsLoaded,
            ),
            Message::KernelsLoaded(result) => {
                match result {
                    Ok(versions) => self.installed_kernels = versions,
                    Err(e) => self.error_msg = Some(e.to_string()),
                }
                Task::none()
            }
            Message::SetDefaultKernel(version) => Task::perform(
                async move {
                    let vm = VersionManager::new().map_err(InfiltratorError::from)?;
                    vm.set_default(&version)
                        .await
                        .map_err(InfiltratorError::from)
                },
                |_| Message::LoadKernels,
            ),
            Message::DeleteKernel(version) => Task::perform(
                async move {
                    let vm = VersionManager::new().map_err(InfiltratorError::from)?;
                    vm.uninstall(&version).await.map_err(InfiltratorError::from)
                },
                |_| Message::LoadKernels,
            ),
            Message::FactoryReset => Task::none(),
            _ => Task::none(),
        }
    }
}
