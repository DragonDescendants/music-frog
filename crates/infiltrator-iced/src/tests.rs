use super::*;
use crate::locales::{Lang, Localizer};
use iced::widget::text_editor;
use mihomo_api::TrafficData;
use mihomo_config::Profile;
use std::path::PathBuf;

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
    let _ = state.update(Message::RuntimeConfigFetched(Ok((
        "global".into(),
        true,
        vec!["1.1.1.1".into()],
        vec!["8.8.8.8".into()],
        "fake-ip".into(),
        "gvisor".into(),
        true,
        false,
        true,
    ))));

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

    // Throttled
    let _ = state.update(Message::TrafficReceived(TrafficData {
        up: 1500,
        down: 1500,
    }));
    assert_eq!(state.traffic.as_ref().unwrap().up, 1000);

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

    let _ = state.update(Message::RemoveDnsServer(0));
    assert_eq!(state.dns_nameservers.len(), 1);
    assert_eq!(state.dns_nameservers[0], "");
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

    // Save success (should return to profiles)
    state.current_route = Route::Editor;
    let _ = state.update(Message::ProfileSaved(Ok(())));
    assert_eq!(state.current_route, Route::Profiles);
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
