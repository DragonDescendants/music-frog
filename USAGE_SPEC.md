# Usage Specification / 使用指南

This document provides instructions for users on how to interact with the **MusicFrog Despicable Infiltrator** interface and its features.
本文档为用户提供如何使用 **MusicFrog Despicable Infiltrator** 界面及其功能的说明。

---

## 1. Tray Menu / 托盘菜单

The tray menu provides quick access to connection info, proxy/config pages, and core controls.
托盘菜单提供连接信息、代理/配置页面与核心控制的快速访问。

- **Open Proxy Page / 打开代理页**: Opens the proxy Web UI in your default browser.
  **打开代理页**: 在默认浏览器中打开代理 Web UI。
- **Open Config Page / 打开配置页**: Opens the configuration page (Admin Web UI) in your default browser.
  **打开配置页**: 在默认浏览器中打开配置页面（管理 Web UI）。
- **System Proxy / 系统代理**: Toggle the system-wide proxy on or off.
  **系统代理**: 切换全局系统代理的开启或关闭。
- **Proxy Mode / 代理模式**: Switch between `Rule`, `Global`, `Direct`, and `Script` modes.
  **代理模式**: 在 `规则`、`全局`、`直连` 和 `脚本` 模式之间切换。
- **Profile Switch / 配置切换**: Quickly switch between different proxy profiles.
  **配置切换**: 在不同的代理配置之间快速切换。
- **Advanced Settings / 高级设置**: Direct links to specific configuration panels (DNS, Tun, Rules, etc.).
  **高级设置**: 直达特定配置面板（DNS、Tun、规则等）的链接。

---

## 2. Admin UI Navigation / 管理界面导航

The Admin Web UI uses a left-side **Sections** menu and a fixed top status header.
管理 Web UI 使用左侧 **导航** 菜单与顶部固定状态栏。

- **Status Header / 顶部状态栏**:
  - **Language / 语言**: Select Follow System / 简体中文 / English.
      **Language / 语言**: 选择 跟随系统 / 简体中文 / English。
  - **Theme / 深色模式**: Select Follow System / Light / Dark.
      **Theme / 深色模式**: 选择 跟随系统 / 浅色 / 深色。
  - **Refresh Status / 刷新状态**: Reload status and panel data.
      **Refresh Status / 刷新状态**: 刷新状态与各面板数据。
  - **Refresh Updates / 刷新更新**: Appears when background changes are detected; click to reload the latest data.
      **Refresh Updates / 刷新更新**: 检测到后台变更时出现，点击后刷新最新数据。
- **Sections / 导航分组**:
  - **Profiles & Imports / 配置管理**: Profile list, subscription import, local import, editor, and external editor.
      **Profiles & Imports / 配置管理**: 配置列表、订阅导入、本地导入、编辑器与外部编辑器。
  - **WebDAV Sync / WebDAV 同步**: WebDAV configuration, test, and manual sync.
      **WebDAV Sync / WebDAV 同步**: WebDAV 配置、连接测试与手动同步。
  - **DNS & Fake-IP / DNS / Fake-IP**: DNS settings and Fake-IP cache control.
      **DNS & Fake-IP / DNS / Fake-IP**: DNS 设置与 Fake-IP 缓存控制。
  - **Core & TUN / 内核与 TUN**: Core version switching and TUN advanced settings.
      **Core & TUN / 内核与 TUN**: 内核版本切换与 TUN 高级设置。
  - **Runtime / 运行态**: Live connections, logs, traffic, memory, and egress IP diagnostics.
      **Runtime / 运行态**: 实时连接、日志、流量、内存与出口 IP 诊断。
  - **Rules & Providers / 规则与 Providers**: Rule providers and rule list management.
      **Rules & Providers / 规则与 Providers**: Rule Providers 与规则列表管理。

---

## 3. Profiles & Imports / 配置管理

Manage profiles, imports, editing, and synchronization in the **Profiles & Imports** section.
在 **Profiles & Imports**（配置管理）分组中完成配置管理、导入、编辑与同步。

- **Profiles / 现有配置**:
  - **Refresh / 刷新**: Reload profile list and status.
      **Refresh / 刷新**: 重新加载配置列表与状态。
  - **Clear Configs / 清空配置**: Clear all configs and restore defaults.
      **Clear Configs / 清空配置**: 清空配置并恢复默认。
  - **Set Active / 设为当前**: Activate the selected profile and restart the core.
      **Set Active / 设为当前**: 激活所选配置并重启内核。
  - **Edit / 编辑**: Open the built-in **Config Editor** with the selected profile.
      **Edit / 编辑**: 打开内置 **Config Editor** 进行编辑。
  - **External Edit / 外部编辑**: Launch the external editor for the selected profile.
      **External Edit / 外部编辑**: 用外部编辑器打开所选配置。
  - **Delete / 删除**: Remove the selected profile.
      **Delete / 删除**: 删除所选配置。
  - **Update Now / 立即更新** + **Save Settings / 保存订阅设置**: Update subscription and save its schedule.
      **Update Now / 立即更新** + **Save Settings / 保存订阅设置**: 立即更新订阅并保存更新计划。
- **Import Subscription / 通过订阅导入**:
  - **Name / 配置名称** + **Subscription URL / 订阅链接**: Fill in the subscription metadata.
      **Name / 配置名称** + **Subscription URL / 订阅链接**: 填写订阅名称与 URL。
  - **Set as active after import / 导入后设为当前配置** + **Import Now / 立即导入**:
    Import the subscription and optionally activate it (core restarts in background).
      **Set as active after import / 导入后设为当前配置** + **Import Now / 立即导入**:
        导入订阅并可选自动激活（内核后台重启）。
- **Import Local File / 从本地文件导入**:
  - **Select File / 选择文件**: Choose a `.yaml` or `.toml` file from disk.
      **Select File / 选择文件**: 从磁盘选择 `.yaml` 或 `.toml`。
  - **Save Local Config / 保存本地配置**: Import and save the local file.
      **Save Local Config / 保存本地配置**: 导入并保存本地文件。
- **Config Editor / 配置编辑器**:
  - **Save Config / 保存配置**: Save the content as a profile, optionally activate it.
      **Save Config / 保存配置**: 保存内容为配置，并可选激活。
  - **External Edit / 外部编辑**: Open the current profile in external editor.
      **External Edit / 外部编辑**: 用外部编辑器打开当前配置。
- **External Editor / 外部编辑器设置**:
  - **Select Path / 选择路径**: Pick the external editor executable.
      **Select Path / 选择路径**: 选择外部编辑器路径。
  - **Save / 保存** + **Reset / 恢复默认**: Save or reset editor settings.
      **Save / 保存** + **Reset / 恢复默认**: 保存或重置外部编辑器设置。

---

## 4. WebDAV Sync / WebDAV 同步

Manage sync settings, tests, and manual sync in the **WebDAV Sync** section.
在 **WebDAV Sync** 分组中管理同步设置、连接测试与手动同步。

- **Test Connection / 连接测试**: Verify the WebDAV server.
    **Test Connection / 连接测试**: 验证 WebDAV 服务可用性。
- **Sync Now / 立即同步**: Start a manual synchronization.
    **Sync Now / 立即同步**: 手动触发同步。
- **Save / 保存**: Persist sync settings.
    **Save / 保存**: 保存同步设置。

---

## 5. DNS & Fake-IP / DNS 与 Fake-IP

Tune network behavior in the **DNS & Fake-IP** section.
在 **DNS & Fake-IP** 分组中调整网络行为。

- **DNS Settings / DNS 设置**:
  - **Save / 保存**: Apply DNS settings.
      **Save / 保存**: 应用 DNS 设置。
  - **Refresh / 刷新**: Reload DNS settings.
      **Refresh / 刷新**: 重新加载 DNS 设置。
- **Fake-IP / Fake-IP**:
  - **Save / 保存**: Apply Fake-IP settings.
      **Save / 保存**: 应用 Fake-IP 设置。
  - **Refresh / 刷新**: Reload Fake-IP settings.
      **Refresh / 刷新**: 重新加载 Fake-IP 设置。
  - **Flush Cache / 清理缓存**: Clear the Fake-IP cache.
      **Flush Cache / 清理缓存**: 清理 Fake-IP 缓存。

---

## 6. Core & TUN / 内核与 TUN

Manage core versions and advanced TUN settings in the **Core & TUN** section.
在 **Core & TUN** 分组中管理内核版本与 TUN 高级设置。

- **Core Version / 内核版本**:
  - **Refresh / 刷新**: Reload available core versions.
      **Refresh / 刷新**: 重新加载可用内核版本。
  - **Refresh Stable / 刷新稳定版**: Fetch the latest stable release information.
      **Refresh Stable / 刷新稳定版**: 获取最新稳定版发布信息。
  - **Update To Stable / 更新到稳定版**: Download and switch to the latest stable core, then rebuild runtime.
      **Update To Stable / 更新到稳定版**: 下载并切换到最新稳定版内核，然后重建运行时。
  - **Download Stable / 下载稳定版**: Download the latest stable core without switching immediately.
      **Download Stable / 下载稳定版**: 仅下载最新稳定版内核，不立即切换。
  - **Download Specific Version / 下载指定版本**: Enter a version (for example `v1.20.0`) and download it.
      **Download Specific Version / 下载指定版本**: 输入版本号（例如 `v1.20.0`）并下载。
  - **Use / 启用**: Switch to the selected core version.
      **Use / 启用**: 切换到所选内核版本。
- **TUN Advanced / TUN 高级设置**:
  - **Save / 保存**: Apply TUN settings.
      **Save / 保存**: 应用 TUN 设置。
  - **Refresh / 刷新**: Reload TUN settings.
      **Refresh / 刷新**: 重新加载 TUN 设置。

---

## 7. Rules & Providers / 规则与 Providers

Manage rule providers and rules in the **Rules & Providers** section.
在 **Rules & Providers** 分组中管理 Rule Providers 与规则列表。

- **Rule Providers (JSON) / 规则提供者 (JSON)**:
  - **Save Providers / 保存 Providers**: Save the JSON providers configuration.
      **Save Providers / 保存 Providers**: 保存 Providers JSON 配置。
- **Proxy Providers (JSON) / 代理提供者 (JSON)**:
  - **Save Proxy Providers / 保存代理提供者**: Save the JSON proxy providers configuration.
      **Save Proxy Providers / 保存代理提供者**: 保存代理提供者 JSON 配置。
- **Sniffer (JSON) / 嗅探器 (JSON)**:
  - **Save Sniffer / 保存嗅探器**: Save the sniffer JSON configuration.
      **Save Sniffer / 保存嗅探器**: 保存 sniffer JSON 配置。
- **Rules / 规则列表**:
  - **Add Rule / 新增规则**: Insert a new rule line.
      **Add Rule / 新增规则**: 添加新规则行。
  - **Save Rules / 保存规则**: Apply rule list changes.
      **Save Rules / 保存规则**: 保存规则列表更改。
- **Rule list scroll / 规则列表滚动**: The rules list scrolls inside the panel to handle large sets.
      **Rule list scroll / 规则列表滚动**: 规则列表在面板内滚动，以适配大量规则。

---

## 8. Admin UI: Runtime / 管理界面：运行态

Monitor live status in the **Runtime** section.
在 **Runtime** 分组中查看实时运行状态。

- **Refresh / 刷新**: Reload connections, traffic, memory, and IP diagnostics.
    **Refresh / 刷新**: 刷新连接、流量、内存与 IP 诊断数据。
- **Connections / 连接列表**:
  - **Filter input / 过滤输入框**: Filter by host, process, IP, or rule.
      **Filter input / 过滤输入框**: 按主机、进程、IP 或规则过滤连接。
  - **Close / 断开**: Close a single connection row.
      **Close / 断开**: 断开单条连接。
  - **Close All / 断开全部**: Close all active connections after confirmation.
      **Close All / 断开全部**: 确认后断开全部活动连接。
- **Runtime Logs / 运行日志**:
  - **Log Level / 日志级别**: Switch stream level (`Debug`, `Info`, `Warning`, `Error`, `Silent`).
      **Log Level / 日志级别**: 切换日志流级别（`Debug`、`Info`、`Warning`、`Error`、`Silent`）。
  - **Stream badge / 流状态徽标**: Shows whether the log stream is connected.
      **Stream badge / 流状态徽标**: 显示日志流当前是否已连接。
  - **Clear / 清空**: Clear current on-screen log lines.
      **Clear / 清空**: 清空当前界面日志行。
- **Proxy Delay Test / 节点延迟测速**:
  - **Test / 测速**: Run delay test for one proxy node.
      **Test / 测速**: 对单个代理节点执行延迟测试。
  - **Test All / 全部测速**: Run batch delay test for all available proxy nodes.
      **Test All / 全部测速**: 对可测速节点执行批量延迟测试。
  - **Sort / 排序**: Sort by delay or name to quickly identify fast/slow nodes.
      **Sort / 排序**: 按延迟或名称排序，快速定位快慢节点。
- **Auto Refresh / 自动刷新**: Poll traffic, memory, and connections while Runtime is open.
    **Auto Refresh / 自动刷新**: Runtime 分组打开时自动轮询流量、内存和连接。
- **Refresh IP / 刷新 IP**: Manually update egress IP and location summary.
    **Refresh IP / 刷新 IP**: 手动刷新出口 IP 与地区摘要。

---

## 9. Android App: VPN/TUN / Android 应用：VPN/TUN

Manage VPN/TUN parameters in the **Settings > TUN** screen.
在 **Settings > TUN** 页面管理 VPN/TUN 参数。

- **MTU / MTU**: Set the MTU value.
      **MTU / MTU**: 设置 MTU 数值。
- **Auto Route / 自动路由**: Toggle default routing through the VPN.
      **Auto Route / 自动路由**: 开关默认经由 VPN 的路由。
- **Stack / 协议栈**: Select `Auto`, `system`, or `gvisor`.
      **Stack / 协议栈**: 选择 `Auto`、`system` 或 `gvisor`。
- **Strict Route / 严格路由**: Toggle strict routing behavior (Android uses route settings as available).
      **Strict Route / 严格路由**: 开关严格路由行为（Android 以现有路由设置为准）。
- **Auto Detect Interface / 自动检测网卡**: Toggle automatic outbound interface detection.
      **Auto Detect Interface / 自动检测网卡**: 开关自动检测出口网卡。
- **IPv6 / IPv6**: Enable or disable IPv6 routing for VPN.
      **IPv6 / IPv6**: 开关 VPN 的 IPv6 路由。
- **DNS Servers (one per line) / DNS Servers（每行一个）**: Enter DNS server IPs, one per line.
      **DNS Servers (one per line) / DNS Servers（每行一个）**: 逐行填写 DNS 服务器 IP。
- **Save / 保存**: Apply VPN/TUN settings.
      **Save / 保存**: 应用 VPN/TUN 设置。
- **Reload / 重新加载**: Reload current settings from the active profile.
      **Reload / 重新加载**: 从当前配置重新加载设置。

---

## 10. Android App: DNS / Android 应用：DNS

Manage DNS settings in **Settings > DNS**.
在 **Settings > DNS** 页面管理 DNS 设置。

- **Enable DNS / 启用 DNS**: Toggle DNS on or off.
      **Enable DNS / 启用 DNS**: 开关 DNS 功能。
- **IPv6 / IPv6**: Enable or disable IPv6 resolution.
      **IPv6 / IPv6**: 开关 IPv6 解析。
- **Enhanced Mode / 增强模式**: Enter `fake-ip` or `redir-host`.
      **Enhanced Mode / 增强模式**: 填写 `fake-ip` 或 `redir-host`。
- **Nameserver / 主 DNS**: Enter DNS servers, one per line.
      **Nameserver / 主 DNS**: 逐行填写 DNS 服务器。
- **Default Nameserver / 默认 DNS**: Enter default DNS servers, one per line.
      **Default Nameserver / 默认 DNS**: 逐行填写默认 DNS 服务器。
- **Fallback / 备用 DNS**: Enter fallback DNS servers, one per line.
      **Fallback / 备用 DNS**: 逐行填写备用 DNS 服务器。
- **Fallback Filter GeoIP / Fallback 过滤 GeoIP**: Set `Auto`, `Enabled`, or `Disabled`.
      **Fallback Filter GeoIP / Fallback 过滤 GeoIP**: 选择 `Auto`、`启用` 或 `禁用`。
- **Fallback Filter GeoIP Code / Fallback 过滤 GeoIP 代码**: Set optional country/region code.
      **Fallback Filter GeoIP Code / Fallback 过滤 GeoIP 代码**: 设置可选国家/地区代码。
- **Fallback Filter IPCIDR / Fallback 过滤 IPCIDR**: Enter CIDR list, one per line.
      **Fallback Filter IPCIDR / Fallback 过滤 IPCIDR**: 逐行填写 CIDR 列表。
- **Fallback Filter Domain / Fallback 过滤域名**: Enter full domains, one per line.
      **Fallback Filter Domain / Fallback 过滤域名**: 逐行填写完整域名。
- **Fallback Filter Domain Suffix / Fallback 过滤域名后缀**: Enter suffix list, one per line.
      **Fallback Filter Domain Suffix / Fallback 过滤域名后缀**: 逐行填写域名后缀列表。
- **Save / 保存**: Apply DNS settings.
      **Save / 保存**: 应用 DNS 设置。
- **Reload / 重新加载**: Reload current DNS settings.
      **Reload / 重新加载**: 重新加载 DNS 设置。

---

## 11. Android App: Fake-IP / Android 应用：Fake-IP

Manage Fake-IP settings in **Settings > Fake-IP**.
在 **Settings > Fake-IP** 页面管理 Fake-IP 设置。

- **Fake-IP Range / Fake-IP 范围**: Set the fake IP CIDR range.
      **Fake-IP Range / Fake-IP 范围**: 设置 Fake-IP 的 CIDR 范围。
- **Fake-IP Filter / Fake-IP 过滤**: Enter filter rules, one per line.
      **Fake-IP Filter / Fake-IP 过滤**: 逐行填写过滤规则。
- **Store Fake-IP / 持久化 Fake-IP**: Toggle persistence for fake IP cache.
      **Store Fake-IP / 持久化 Fake-IP**: 开关 Fake-IP 缓存持久化。
- **Save / 保存**: Apply Fake-IP settings.
      **Save / 保存**: 应用 Fake-IP 设置。
- **Reload / 重新加载**: Reload current Fake-IP settings.
      **Reload / 重新加载**: 重新加载 Fake-IP 设置。
- **Clear Cache / 清理缓存**: Clear the Fake-IP cache file.
      **Clear Cache / 清理缓存**: 清理 Fake-IP 缓存文件。

---

## 12. Android App: Rules / Android 应用：规则

Manage rules and providers in **Settings > Rules**.
在 **Settings > Rules** 页面管理规则与 Providers。

- **New Rule / 新增规则** + **Add Rule / 添加规则**: Add a new rule line.
      **New Rule / 新增规则** + **Add Rule / 添加规则**: 添加新规则行。
- **Rule Toggle / 规则开关**: Enable or disable an entry.
      **Rule Toggle / 规则开关**: 启用或禁用规则条目。
- **Remove / 删除**: Remove a rule entry.
      **Remove / 删除**: 删除规则条目。
- **Save Rules / 保存规则**: Apply rule list changes.
      **Save Rules / 保存规则**: 保存规则列表变更。
- **Rule Providers (JSON) / Providers (JSON)**: Edit providers JSON config.
      **Rule Providers (JSON) / Providers (JSON)**: 编辑 Providers 的 JSON 配置。
- **Save Providers / 保存 Providers**: Save providers configuration.
      **Save Providers / 保存 Providers**: 保存 Providers 配置。

---

## 13. Android App: WebDAV Sync / Android 应用：WebDAV 同步

Manage WebDAV sync in the **Sync** tab.
在 **Sync** 标签页管理 WebDAV 同步。

- **Enable WebDAV Sync / 启用 WebDAV 同步**: Toggle WebDAV sync on or off.
      **Enable WebDAV Sync / 启用 WebDAV 同步**: 开关 WebDAV 同步。
- **WebDAV URL / WebDAV 地址**: Set the WebDAV endpoint.
      **WebDAV URL / WebDAV 地址**: 设置 WebDAV 地址。
- **Username / 用户名** + **Password / 密码**: Configure credentials.
      **Username / 用户名** + **Password / 密码**: 配置用户名与密码。
- **Sync interval (minutes) / 同步间隔（分钟）**: Set the sync interval.
      **Sync interval (minutes) / 同步间隔（分钟）**: 设置同步间隔。
- **Sync on startup / 启动时同步**: Run sync when the app starts.
      **Sync on startup / 启动时同步**: 应用启动时触发同步。
- **Save / 保存**: Persist WebDAV settings.
      **Save / 保存**: 保存 WebDAV 设置。
- **Test / 连接测试**: Verify the WebDAV connection.
      **Test / 连接测试**: 验证 WebDAV 连接。
- **Sync Now / 立即同步**: Trigger a manual sync.
      **Sync Now / 立即同步**: 手动触发同步。
- **Reload / 重新加载**: Reload WebDAV settings.
      **Reload / 重新加载**: 重新加载 WebDAV 设置。

---

## 14. Android App: Profiles / Android 应用：配置管理

Manage profiles in the **Profiles** tab.
在 **Profiles** 标签页管理配置。

- **Add Profile / 添加配置**: Tap the `+` floating button, enter **Name** and **URL**, then tap **Add**.
      **Add Profile / 添加配置**: 点击右下角 `+` 浮动按钮，填写 **名称** 与 **URL** 后点击 **添加**。
- **Import Local / 本地导入**: Tap the upload floating button, choose a local file, review content, then tap **Save**.
      **Import Local / 本地导入**: 点击上传浮动按钮，选择本地文件，确认内容后点击 **保存**。
- **Edit / 编辑**: Tap row **Edit** to open profile content editor, then tap **Save**.
      **Edit / 编辑**: 点击行内 **编辑** 打开配置内容编辑器，完成后点击 **保存**。
- **Update Now / 立即更新**: Tap row **Update Now** to refresh subscription content.
      **Update Now / 立即更新**: 点击行内 **立即更新** 拉取订阅最新内容。
- **Subscription Settings / 订阅设置**: Tap row **Subscription Settings** to update URL, auto-update switch, and interval.
      **Subscription Settings / 订阅设置**: 点击行内 **订阅设置**，更新 URL、自动更新开关与间隔。
- **Delete / 删除**: Tap row **Delete**, then confirm in the dialog.
      **Delete / 删除**: 点击行内 **删除**，并在确认框中确认。

---

## 15. Android App: App Routing / Android 应用：应用路由

Manage per-app routing in **Settings > App Routing**.
在 **设置 > App Routing** 页面管理分应用路由。

- **Routing Mode / 路由模式**: Select **Proxy All Apps**, **Proxy Selected (Allowlist)**, or **Bypass Selected (Blocklist)**.
      **Routing Mode / 路由模式**: 选择 **代理所有应用**、**仅代理选中（白名单）** 或 **仅绕过选中（黑名单）**。
- **Search Apps / 搜索应用**: Use the search box to filter app list.
      **Search Apps / 搜索应用**: 使用搜索框过滤应用列表。
- **User Apps / System Apps / 用户应用 / 系统应用**: Switch tabs and toggle app switches to include/exclude apps based on mode.
      **User Apps / System Apps / 用户应用 / 系统应用**: 切换标签页并使用开关按当前模式选择应用。

---

## 16. Android App: Connections / Android 应用：连接管理

Manage runtime connections in **Settings > Connections**.
在 **设置 > 连接管理** 页面管理运行时连接。

- **Host Filter / 主机过滤** + **Process Filter / 进程过滤**: Filter active connections by host/process path.
      **Host Filter / 主机过滤** + **Process Filter / 进程过滤**: 按主机或进程路径过滤活动连接。
- **Refresh / 刷新**: Reload current active connection list.
      **Refresh / 刷新**: 重新加载当前活动连接列表。
- **Disconnect / 断开**: Disconnect a single connection from the row action.
      **Disconnect / 断开**: 通过行内操作断开单条连接。
- **Close All / 全部断开**: Disconnect all active connections at once.
      **Close All / 全部断开**: 一次性断开所有活动连接。

---

## 17. Android App: Overview Mode / Android 应用：概览模式

Switch proxy mode in the **Overview** page with **Change Mode**.
在 **概览** 页面通过 **切换模式** 修改代理模式。

- **Rule / Global / Direct / Script**: Select any mode from the dropdown and apply immediately.
      **Rule / Global / Direct / Script**: 在下拉菜单中选择模式并立即生效。
