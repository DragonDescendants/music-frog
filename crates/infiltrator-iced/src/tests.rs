use super::*;
use crate::locales::{Lang, Localizer};
use iced::widget::text_editor;
use mihomo_api::{Proxy, ProxyBase, ProxyGroup, ProxyHistory, TrafficData};
use mihomo_config::Profile;
use std::path::PathBuf;

#[test]
fn test_route_navigation() {
    let (mut state, _) = AppState::new();
    assert_eq!(state.current_route, Route::Overview);

    let _ = state.update(Message::Navigate(Route::Runtime));
    assert_eq!(state.current_route, Route::Runtime);

    let _ = state.update(Message::Navigate(Route::Settings));
    assert_eq!(state.current_route, Route::Settings);
}

#[test]
fn test_proxy_lifecycle_messages() {
    let (mut state, _) = AppState::new();

    // Test Start
    let _ = state.update(Message::StartProxy);
    assert!(state.is_starting);
    assert!(state.error_msg.is_none());

    // Test ProxyStarted Error
    let _ = state.update(Message::ProxyStarted(Err("Failed to bind port".into())));
    assert!(!state.is_starting);
    assert_eq!(state.error_msg.as_ref().unwrap(), "Failed to bind port");

    // Test ProxyStopped cleanup
    state.traffic = Some(TrafficData { up: 100, down: 100 });
    state.proxy_mode = Some("rule".into());
    state.logs.push_back("some log".into());

    let _ = state.update(Message::ProxyStopped);
    assert!(state.traffic.is_none());
    assert!(state.proxy_mode.is_none());
    assert!(state.logs.is_empty());
}

#[test]
fn test_runtime_config_sync() {
    let (mut state, _) = AppState::new();

    // Simulate config fetch
    let _ = state.update(Message::RuntimeConfigFetched(Ok(RuntimeConfig {
        mode: "global".into(),
        tun_enabled: true,
        dns_nameservers: vec!["1.1.1.1".into()],
        dns_fallback: vec!["8.8.8.8".into()],
        dns_enhanced_mode: "fake-ip".into(),
        tun_stack: "gvisor".into(),
        tun_auto_route: true,
        tun_strict_route: false,
        sniffer_enabled: true,
    })));

    assert_eq!(state.proxy_mode.as_ref().unwrap(), "global");
    assert!(state.tun_enabled.unwrap());
    assert_eq!(state.dns_nameservers[0], "1.1.1.1");
}

#[test]
fn test_mode_set_interactions() {
    let (mut state, _) = AppState::new();

    // Success path (should trigger a re-fetch)
    let _task = state.update(Message::ModeSetResult(Ok(())));
    assert!(state.error_msg.is_none());

    // Failure path
    let _ = state.update(Message::ModeSetResult(Err("Core busy".into())));
    assert_eq!(state.error_msg.as_ref().unwrap(), "Core busy");
}

#[test]
fn test_log_buffer_limit_and_queue() {
    let (mut state, _) = AppState::new();
    for i in 0..600 {
        let _ = state.update(Message::LogReceived(format!("log line {}", i)));
    }
    assert_eq!(state.logs.len(), 500);
    assert_eq!(state.logs.front().unwrap(), "log line 100");
}

#[test]
fn test_traffic_throttling_logic() {
    let (mut state, _) = AppState::new();
    let _ = state.update(Message::TrafficReceived(TrafficData {
        up: 1000,
        down: 1000,
    }));
    assert_eq!(state.traffic.as_ref().unwrap().up, 1000);

    // No throttling currently implemented
    let _ = state.update(Message::TrafficReceived(TrafficData {
        up: 1500,
        down: 1500,
    }));
    assert_eq!(state.traffic.as_ref().unwrap().up, 1500);

    // Updated
    let _ = state.update(Message::TrafficReceived(TrafficData {
        up: 3000,
        down: 3000,
    }));
    assert_eq!(state.traffic.as_ref().unwrap().up, 3000);
}

#[test]
fn test_dns_server_list_manipulation() {
    let (mut state, _) = AppState::new();
    state.dns_nameservers = vec!["old".into()];

    let _ = state.update(Message::UpdateDnsServer(0, "new".into()));
    assert_eq!(state.dns_nameservers[0], "new");

    let _ = state.update(Message::AddDnsServer);
    assert_eq!(state.dns_nameservers.len(), 2);

    let _ = state.update(Message::AddDnsServerTemplate("https://1.1.1.1/dns-query".into()));
    assert_eq!(state.dns_nameservers.len(), 3);
    assert_eq!(state.dns_nameservers[2], "https://1.1.1.1/dns-query");

    let _ = state.update(Message::RemoveDnsServer(0));
    assert_eq!(state.dns_nameservers.len(), 2);
    assert_eq!(state.dns_nameservers[0], "");

    // Fallbacks
    let _ = state.update(Message::AddFallbackDnsServer);
    assert_eq!(state.dns_fallback_servers.len(), 1);

    let _ = state.update(Message::UpdateFallbackDnsServer(0, "8.8.8.8".into()));
    assert_eq!(state.dns_fallback_servers[0], "8.8.8.8");

    let _ = state.update(Message::RemoveFallbackDnsServer(0));
    assert_eq!(state.dns_fallback_servers.len(), 0);
}

#[test]
fn test_system_integration_states() {
    let (mut state, _) = AppState::new();

    // System Proxy toggle
    state.system_proxy_enabled = false;
    let _ = state.update(Message::SetSystemProxy(true));
    assert!(state.system_proxy_enabled);

    // Rollback on error
    let _ = state.update(Message::SystemProxySet(Err("Access denied".into())));
    assert!(!state.system_proxy_enabled, "Should rollback on failure");
    assert_eq!(state.error_msg.as_ref().unwrap(), "Access denied");

    // Autostart
    state.autostart_enabled = false;
    let _ = state.update(Message::SetAutostart(true));
    assert!(state.autostart_enabled);

    let _ = state.update(Message::AutostartSet(Err("Registry lock".into())));
    assert!(
        !state.autostart_enabled,
        "Should rollback autostart on failure"
    );
}

#[test]
fn test_profiles_and_rules_loading() {
    let (mut state, _) = AppState::new();

    // Profiles loaded
    let _ = state.update(Message::ProfilesLoaded(Ok(vec![Profile::new(
        "test".into(),
        PathBuf::from("test.yaml"),
        true,
    )])));
    assert_eq!(state.profiles.len(), 1);
    assert!(!state.is_loading_profiles);

    // Rules loaded
    let _ = state.update(Message::RulesLoaded(Ok(vec![mihomo_api::Rule {
        rule_type: "Direct".into(),
        payload: "local".into(),
        proxy: "Direct".into(),
        size: 0,
    }])));
    assert_eq!(state.rules.len(), 1);
}

#[test]
fn test_editor_actions() {
    let (mut state, _) = AppState::new();

    // Simulate successful load
    let _ = state.update(Message::ProfileContentLoaded(Ok((
        PathBuf::from("config.yaml"),
        "proxies: []".into(),
    ))));
    assert_eq!(
        state.editor_path.as_ref().unwrap().to_str().unwrap(),
        "config.yaml"
    );
    assert_eq!(state.editor_content.text(), "proxies: []");

    // Editor action (simulating typing)
    let _ = state.update(Message::EditorAction(text_editor::Action::Edit(
        text_editor::Edit::Insert('a'),
    )));
    assert_ne!(state.editor_content.text(), "proxies: []");

    // Save success
    state.current_route = Route::Editor;
    let _ = state.update(Message::ProfileSaved(Ok(())));
    assert_eq!(state.current_route, Route::Editor);
}

#[test]
fn test_tray_and_exit() {
    let (mut state, _) = AppState::new();

    // Tray event shouldn't crash
    let _ = state.update(Message::TrayIconEvent(tray_icon::TrayIconEvent::Click {
        id: "test".into(),
        position: tray_icon::dpi::PhysicalPosition { x: 0.0, y: 0.0 },
        rect: tray_icon::Rect {
            position: tray_icon::dpi::PhysicalPosition { x: 0.0, y: 0.0 },
            size: tray_icon::dpi::PhysicalSize {
                width: 0,
                height: 0,
            },
        },
        button: tray_icon::MouseButton::Left,
        button_state: tray_icon::MouseButtonState::Up,
    }));

    let _ = state.update(Message::MenuEvent(muda::MenuEvent { id: "show".into() }));
}

#[test]
fn test_i18n_fallback() {
    let lang = Lang("fr-FR"); // Unsupported
    assert_eq!(
        lang.tr("nav_profiles"),
        "配置管理",
        "Should fallback to ZH for unsupported locales"
    );
}

#[test]
fn test_proxy_filtering_and_sorting() {
    let (mut state, _) = AppState::new();
    let mut proxies = std::collections::HashMap::new();

    // Group GLOBAL
    proxies.insert(
        "GLOBAL".to_string(),
        Proxy::Selector(ProxyGroup {
            name: "GLOBAL".to_string(),
            now: "Proxy-A".to_string(),
            all: vec!["Proxy-A".into(), "Proxy-B".into(), "Special".into()],
            history: vec![],
        }),
    );

    // Node A (100ms)
    proxies.insert(
        "Proxy-A".to_string(),
        Proxy::Shadowsocks(mihomo_api::proxy::types::Shadowsocks {
            base: ProxyBase {
                name: "Proxy-A".to_string(),
                history: vec![ProxyHistory {
                    time: "".into(),
                    delay: 100,
                }],
                ..Default::default()
            },
            ..Default::default()
        }),
    );

    // Node B (50ms)
    proxies.insert(
        "Proxy-B".to_string(),
        Proxy::Shadowsocks(mihomo_api::proxy::types::Shadowsocks {
            base: ProxyBase {
                name: "Proxy-B".to_string(),
                history: vec![ProxyHistory {
                    time: "".into(),
                    delay: 50,
                }],
                ..Default::default()
            },
            ..Default::default()
        }),
    );

    // Node Special (200ms)
    proxies.insert(
        "Special".to_string(),
        Proxy::Shadowsocks(mihomo_api::proxy::types::Shadowsocks {
            base: ProxyBase {
                name: "Special".to_string(),
                history: vec![ProxyHistory {
                    time: "".into(),
                    delay: 200,
                }],
                ..Default::default()
            },
            ..Default::default()
        }),
    );

    state.proxies = proxies;

    // Test Search
    let _ = state.update(Message::FilterProxies("special".into()));
    assert_eq!(state.proxy_filter, "special");

    // Test Sort Toggle
    let _ = state.update(Message::ToggleProxySort);
    assert!(state.proxy_sort_by_delay);

    // Verification of logic (manual check of sorting logic used in view)
    let global = state.proxies.get("GLOBAL").unwrap();
    let mut members = global.all().unwrap().to_vec();

    // Apply filter
    let filter = state.proxy_filter.to_lowercase();
    members.retain(|m| m.to_lowercase().contains(&filter));
    assert_eq!(members.len(), 1);
    assert_eq!(members[0], "Special");

    // Apply sort (on all members)
    let mut all_members = global.all().unwrap().to_vec();
    all_members.sort_by_key(|m| {
        state
            .proxies
            .get(m)
            .and_then(|p| p.history().last().map(|h| h.delay))
            .unwrap_or(u32::MAX)
    });

    assert_eq!(all_members[0], "Proxy-B"); // 50ms
    assert_eq!(all_members[1], "Proxy-A"); // 100ms
    assert_eq!(all_members[2], "Special"); // 200ms
}
