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
        "dns_add_fallback" => "添加备用".into(),
        "dns_flush_fakeip" => "清理 Fake-IP 缓存".into(),

        "profiles_title" => "配置列表".into(),
        "profiles_open_folder" => "打开配置文件夹".into(),
        "profiles_import_sub" => "导入订阅地址".into(),
        "profiles_importing" => "正在导入...".into(),
        "profiles_import_name_placeholder" => "配置文件名称 (例如 MyProxy)".into(),
        "profiles_sub_url" => "订阅 URL".into(),
        "profiles_import_btn" => "立即导入".into(),

        "proxies_title" => "节点选择".into(),
        "proxies_sort_delay" => "按延迟排序".into(),
        "rules_title" => "规则管理".into(),
        "rules_filter_placeholder" => "搜索规则内容...".into(),
        "rules_proxy_providers" => "代理提供者".into(),
        "rules_rule_providers" => "规则提供者".into(),
        "rules_no_providers" => "暂无提供者".into(),
        "rules_add_custom" => "添加自定义规则".into(),
        "rules_type" => "类型".into(),
        "rules_payload" => "匹配内容 (Payload)".into(),
        "rules_target" => "目标代理/组".into(),
        "rules_add_btn" => "确认添加".into(),

        "refresh" => "刷新".into(),
        "loading_profiles" => "加载配置中...".into(),
        "no_profiles" => "暂无配置".into(),
        "active_tag" => " (已启用)".into(),
        "use" => "使用".into(),
        "edit" => "编辑".into(),

        "runtime_title" => "运行状态".into(),
        "runtime_system_logs" => "系统日志".into(),
        "proxy_not_running" => "代理未运行。".into(),
        "upload_speed" => "上传速率: {0} B/s".into(),
        "download_speed" => "下载速率: {0} B/s".into(),
        "waiting_traffic" => "等待流量数据...".into(),

        "settings_title" => "设置 & 内核".into(),
        "settings_system_integration" => "系统集成".into(),
        "settings_sniffer" => "流量嗅探 (Sniffer)".into(),
        "settings_sniffer_desc" => "嗅探流量以还原域名，提升路由精确度。".into(),
        "settings_kernel_mgmt" => "内核管理".into(),
        "settings_check_update" => "检查更新".into(),
        "settings_checking" => "检查中...".into(),
        "settings_latest_version" => "最新版本: {0}".into(),
        "settings_no_kernels" => "未发现内核。".into(),
        "settings_factory_reset" => "恢复出厂设置 (不可逆)".into(),
        "settings_uac_request" => "以管理员身份重启".into(),
        "settings_uac_desc" => "开启 TUN 模式需要管理员权限。点击下方按钮尝试提升权限。".into(),
        "settings_download" => "下载".into(),
        "settings_delete" => "删除".into(),
        "settings_set_default" => "设为默认".into(),
        "settings_installed" => "已安装".into(),
        "settings_available" => "有新版本可用".into(),

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

        "proxy_mode" => "代理模式".into(),
        "mode_rule" => "规则模式".into(),
        "mode_global" => "全局模式".into(),
        "mode_direct" => "直连模式".into(),
        "mode_script" => "脚本模式".into(),
        "tun_mode" => "TUN 模式".into(),
        "tun_stack" => "协议栈 (Stack)".into(),
        "tun_auto_route" => "自动路由 (Auto Route)".into(),
        "tun_strict_route" => "严格路由 (Strict Route)".into(),

        "overview_traffic" => "实时流量".into(),
        "overview_core" => "核心状态".into(),
        "overview_selected_global" => "已选择 GLOBAL 节点".into(),

        "btn_save" => "保存".into(),
        "btn_cancel" => "取消".into(),
        "btn_close_all" => "关闭全部".into(),
        "btn_switch" => "切换".into(),
        "btn_update" => "更新".into(),

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
        "dns_add_fallback" => "Add Fallback".into(),
        "dns_flush_fakeip" => "Flush Fake-IP Cache".into(),

        "profiles_title" => "Profiles & Imports".into(),
        "profiles_open_folder" => "Open Config Folder".into(),
        "profiles_import_sub" => "Import Subscription".into(),
        "profiles_importing" => "Importing...".into(),
        "profiles_import_name_placeholder" => "Profile Name (e.g. MyProxy)".into(),
        "profiles_sub_url" => "Subscription URL".into(),
        "profiles_import_btn" => "Import Now".into(),

        "proxies_title" => "Proxy Selection".into(),
        "proxies_sort_delay" => "Sort by Delay".into(),
        "rules_title" => "Rules Management".into(),
        "rules_filter_placeholder" => "Search rules...".into(),
        "rules_proxy_providers" => "Proxy Providers".into(),
        "rules_rule_providers" => "Rule Providers".into(),
        "rules_no_providers" => "No providers".into(),
        "rules_add_custom" => "Add Custom Rule".into(),
        "rules_type" => "Type".into(),
        "rules_payload" => "Payload".into(),
        "rules_target" => "Target Proxy/Group".into(),
        "rules_add_btn" => "Add Rule".into(),

        "refresh" => "Refresh".into(),
        "loading_profiles" => "Loading profiles...".into(),
        "no_profiles" => "No profiles found.".into(),
        "active_tag" => " (Active)".into(),
        "use" => "Use".into(),
        "edit" => "Edit".into(),

        "runtime_title" => "Runtime Status".into(),
        "runtime_system_logs" => "System Logs".into(),
        "proxy_not_running" => "Proxy is not running.".into(),
        "upload_speed" => "Upload: {0} B/s".into(),
        "download_speed" => "Download: {0} B/s".into(),
        "waiting_traffic" => "Waiting for traffic data...".into(),

        "settings_title" => "Settings & Core".into(),
        "settings_system_integration" => "System Integration".into(),
        "settings_sniffer" => "Traffic Sniffer".into(),
        "settings_sniffer_desc" => "Sniff traffic to restore domain names for better routing.".into(),
        "settings_kernel_mgmt" => "Kernel Management".into(),
        "settings_check_update" => "Check Update".into(),
        "settings_checking" => "Checking...".into(),
        "settings_latest_version" => "Latest Version: {0}".into(),
        "settings_no_kernels" => "No kernels found.".into(),
        "settings_factory_reset" => "Factory Reset (Irreversible)".into(),
        "settings_uac_request" => "Restart as Administrator".into(),
        "settings_uac_desc" => "TUN mode requires administrative privileges. Click the button below to elevate.".into(),
        "settings_download" => "Download".into(),
        "settings_delete" => "Delete".into(),
        "settings_set_default" => "Set Default".into(),
        "settings_installed" => "Already installed".into(),
        "settings_available" => "New version available".into(),

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
        "dark_mode" => "Dark Mode".into(),

        "proxy_mode" => "Proxy Mode".into(),
        "mode_rule" => "Rule".into(),
        "mode_global" => "Global".into(),
        "mode_direct" => "Direct".into(),
        "mode_script" => "Script".into(),
        "tun_mode" => "TUN Mode".into(),
        "tun_stack" => "Stack".into(),
        "tun_auto_route" => "Auto Route".into(),
        "tun_strict_route" => "Strict Route".into(),

        "overview_traffic" => "REAL-TIME TRAFFIC".into(),
        "overview_core" => "CORE STATUS".into(),
        "overview_selected_global" => "Selected from GLOBAL group".into(),

        "btn_save" => "Save".into(),
        "btn_cancel" => "Cancel".into(),
        "btn_close_all" => "Close All".into(),
        "btn_switch" => "Switch".into(),
        "btn_update" => "Update".into(),

        _ => key.to_string().into(),
    }
}
