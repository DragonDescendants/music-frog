use std::borrow::Cow;

pub trait Localizer {
    fn tr(&self, key: &str) -> Cow<'static, str>;
}

pub struct Lang<'a>(pub &'a str);

impl<'a> Localizer for Lang<'a> {
    fn tr(&self, key: &str) -> Cow<'static, str> {
        match self.0 {
            "en-US" | "en" => translate_en(key),
            _ => translate_zh_cn(key),
        }
    }
}

pub fn get_system_language() -> String {
    if let Some(locale) = sys_locale::get_locale() {
        let normalized = locale.trim().to_ascii_lowercase();
        if normalized.starts_with("zh") {
            "zh-CN".to_string()
        } else {
            "en-US".to_string()
        }
    } else {
        "en-US".to_string()
    }
}

fn translate_zh_cn(key: &str) -> Cow<'static, str> {
    match key {
        "app_title" => "MusicFrog Infiltrator".into(),
        "nav_overview" => "核心概览".into(),
        "nav_profiles" => "配置管理".into(),
        "nav_proxies" => "代理节点".into(),
        "nav_runtime" => "运行状态".into(),
        "nav_rules" => "规则管理".into(),
        "nav_dns" => "DNS 设置".into(),
        "nav_sync" => "数据同步".into(),
        "nav_settings" => "系统设置".into(),

        "sync_title" => "WebDAV 同步".into(),
        "sync_url" => "服务器地址 (URL)".into(),
        "sync_user" => "用户名".into(),
        "sync_pass" => "密码".into(),
        "sync_upload" => "上传配置".into(),
        "sync_download" => "下载配置".into(),

        "dns_title" => "DNS & Fake-IP".into(),
        "dns_nameservers" => "DNS 服务器 (Nameservers)".into(),
        "dns_fallback" => "备用服务器 (Fallback)".into(),
        "dns_save" => "保存并应用".into(),
        "dns_add" => "添加".into(),

        "profiles_title" => "配置列表".into(),
        "proxies_title" => "节点选择".into(),
        "rules_title" => "规则管理".into(),
        "rules_filter_placeholder" => "搜索规则内容...".into(),
        "refresh" => "刷新".into(),
        "loading_profiles" => "加载配置中...".into(),
        "no_profiles" => "暂无配置".into(),
        "active_tag" => " (已启用)".into(),

        "runtime_title" => "运行状态".into(),
        "proxy_not_running" => "代理未运行。".into(),
        "upload_speed" => "上传速率: {0} B/s".into(),
        "download_speed" => "下载速率: {0} B/s".into(),
        "waiting_traffic" => "等待流量数据...".into(),

        "settings_title" => "设置 & 内核".into(),
        "status_starting" => "启动中...".into(),
        "status_running" => "运行中".into(),
        "status_stopped" => "已停止".into(),
        "status_label" => "状态: {0}".into(),
        "error_label" => "错误: {0}".into(),

        "start_proxy" => "启动代理".into(),
        "stop_proxy" => "停止代理".into(),

        "system_proxy" => "系统代理 (System Proxy)".into(),
        "admin_status" => "权限状态".into(),
        "admin_authorized" => "🛡️ 已获得管理员权限 (WinTun 就绪)".into(),
        "admin_unauthorized" => "⚠️ 未获得管理员权限 (无法开启 TUN)".into(),
        "autostart" => "开机自启动".into(),
        "dark_mode" => "深色模式".into(),

        "proxy_mode" => "代理模式".into(),        "mode_rule" => "规则模式".into(),
        "mode_global" => "全局模式".into(),
        "mode_direct" => "直连模式".into(),
        "mode_script" => "脚本模式".into(),
        "tun_mode" => "TUN 模式".into(),

        _ => key.to_string().into(),
    }
}

fn translate_en(key: &str) -> Cow<'static, str> {
    match key {
        "app_title" => "MusicFrog Infiltrator".into(),
        "nav_overview" => "Overview".into(),
        "nav_profiles" => "Profiles".into(),
        "nav_proxies" => "Proxies".into(),
        "nav_runtime" => "Runtime".into(),
        "nav_rules" => "Rules".into(),
        "nav_dns" => "DNS Settings".into(),
        "nav_sync" => "Sync".into(),
        "nav_settings" => "Settings".into(),

        "sync_title" => "WebDAV Sync".into(),
        "sync_url" => "Server URL".into(),
        "sync_user" => "Username".into(),
        "sync_pass" => "Password".into(),
        "sync_upload" => "Upload Config".into(),
        "sync_download" => "Download Config".into(),

        "dns_title" => "DNS & Fake-IP".into(),
        "dns_nameservers" => "Nameservers".into(),
        "dns_fallback" => "Fallback Servers".into(),
        "dns_save" => "Save & Apply".into(),
        "dns_add" => "Add".into(),

        "profiles_title" => "Profiles & Imports".into(),
        "proxies_title" => "Proxy Selection".into(),
        "rules_title" => "Rules Management".into(),
        "rules_filter_placeholder" => "Search rules...".into(),
        "refresh" => "Refresh".into(),
        "loading_profiles" => "Loading profiles...".into(),
        "no_profiles" => "No profiles found.".into(),
        "active_tag" => " (Active)".into(),

        "runtime_title" => "Runtime Status".into(),
        "proxy_not_running" => "Proxy is not running.".into(),
        "upload_speed" => "Upload: {0} B/s".into(),
        "download_speed" => "Download: {0} B/s".into(),
        "waiting_traffic" => "Waiting for traffic data...".into(),

        "settings_title" => "Settings & Core".into(),
        "status_starting" => "Starting...".into(),
        "status_running" => "Running".into(),
        "status_stopped" => "Stopped".into(),
        "status_label" => "Status: {0}".into(),
        "error_label" => "Error: {0}".into(),

        "start_proxy" => "Start Proxy".into(),
        "stop_proxy" => "Stop Proxy".into(),

        "system_proxy" => "System Proxy".into(),
        "admin_status" => "Admin Privileges".into(),
        "admin_authorized" => "🛡️ Admin Authorized (WinTun Ready)".into(),
        "admin_unauthorized" => "⚠️ Not Admin (TUN unavailable)".into(),
        "autostart" => "Start on Boot".into(),

        "proxy_mode" => "Proxy Mode".into(),        "mode_rule" => "Rule".into(),
        "mode_global" => "Global".into(),
        "mode_direct" => "Direct".into(),
        "mode_script" => "Script".into(),
        "tun_mode" => "TUN Mode".into(),

        _ => key.to_string().into(),
    }
}
