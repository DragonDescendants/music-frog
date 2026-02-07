# 缺陷与差距清单 (对标主流 mihomo 客户端)

> 基线时间: 2026-02-06  
> 评估范围: Desktop (Tauri + WebAdmin) 与 Android (Compose + UniFFI)

## 缺陷列表

| ID | 严重度 | 缺陷描述 | 影响 | 证据 | 对应 TODO |
| --- | --- | --- | --- | --- | --- |
| D-001 | High | WebAdmin 缺少运行态管理 API (连接/日志/内存/IP 等) | 桌面端难以完成运行态排障，体验弱于主流客户端 | `crates/infiltrator-admin/src/admin_api.rs` 仅暴露 profiles/settings/dns/fake-ip/rules/tun/core/webdav | A-001 |
| D-002 | High | WebAdmin 页面无运行态面板，仅配置管理 | 需依赖外部页面或托盘，管理路径割裂 | `webui/config-manager-ui/src/App.vue` 导航仅 `profiles/webdav/network/core/rules` | A-002 |
| D-003 | Medium | 代理延迟测速能力未接入可视化流程 | 节点选择缺少量化依据 | `crates/mihomo-api/src/client.rs` 存在 `test_delay`，但 WebAdmin 无对应 API/UI | A-003 |
| D-004 | Medium | Core 更新能力主要在托盘，WebAdmin 只可切换已安装版本 | 桌面 WebAdmin 无法闭环完成更新与版本治理 | `src-tauri/src/core_update.rs` 有完整更新逻辑；`webui/config-manager-ui/src/components/CorePanel.vue` 仅切换已安装版本 | A-004 |
| D-005 | Medium | 高级配置项覆盖不全 (如 proxy-providers/sniffer/fallback-filter 等) | 复杂场景需手改 YAML，易错且回归成本高 | `crates/infiltrator-core/src/dns.rs`/`tun.rs`/`rules.rs` 仅覆盖部分字段模型 | A-005 |
| D-006 | High | Android Profiles 能力不完整 (缺删除/本地导入/编辑/订阅设置) | 常见运维动作需切回桌面或手工处理 | `android/app/src/main/java/com/musicfrog/despicableinfiltrator/ui/profiles/ProfilesViewModel.kt` 仅 `add/select/update` | B-001 |
| D-007 | High | Android 分应用路由存在双数据源 (SharedPreferences 与 Rust FFI 并存) | 状态可能分叉，维护复杂，故障难定位 | `android/.../AppRoutingViewModel.kt` 使用 prefs；`crates/infiltrator-android/src/uniffi_api.rs` 另有 `app_routing_*` | B-002 |
| D-008 | Medium | Android 缺少连接管理页面 | 无法按连接维度诊断与主动断开异常连接 | `android/app/src/main/java/com/musicfrog/despicableinfiltrator/ui/App.kt` 无 Connections 页面；`mihomo-api` 已有连接接口 | B-003 |
| D-009 | Medium | Android 代理模式缺少 `script` | 与桌面模式不一致，策略切换能力不足 | `android/.../OverviewScreen.kt` 下拉仅 `rule/global/direct` | B-004 |
| D-010 | Medium | Android DNS/TUN 字段覆盖低于桌面 | 高级网络配置无法在 Android 完整配置 | `android/.../DnsViewModel.kt`、`TunViewModel.kt` 与 `crates/infiltrator-core/src/dns.rs`、`tun.rs` 字段不一致 | B-005 |
| D-011 | Low | Android 未提供独立 IP 诊断入口 | 网络异常排查效率低 | `crates/infiltrator-android/src/uniffi_api.rs` 有 `ip_check`，UI 未接入 | B-006 |
| D-012 | High | 缺少上述能力的系统化回归矩阵 | 补功能后容易出现回归 | API/UI 回归仅覆盖现有路径，缺少新增差距项的测试基线 | A-006 / B-007 |

## 处理原则

1. 先做 P0: A-001, A-002, A-006, B-001, B-002, B-007。  
2. 每完成一项，同步更新 `TODO.md`、`ANDROID.md`、`CHANGELOG.md`。  
3. 仅在功能可用且回归通过后将状态改为“已完成”。
