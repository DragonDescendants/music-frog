# Iced 跨平台原生 UI 改造计划 (Iced Migration Todo)

## 阶段一：工程基础设施与骨架搭建 (Foundation) [DONE]
- [x] 1.1 初始化工程 (infiltrator-iced)
- [x] 1.2 模块化重构 (拆分 main.rs 为 app.rs, types.rs, view/)
- [x] 1.3 异步桥接 (Tokio + Iced 0.14.2 Task 系统)

## 阶段二：布局、路由与主题 [DONE]
- [x] 2.1 主体框架开发 (Sidebar + Main Content)
- [x] 2.2 路由管理 (Overview, Profiles, Proxies, Runtime, Rules, DNS, Sync, Editor, Settings)
- [x] 2.3 深色/浅色模式动态切换 (集成到设置与托盘)

## 阶段三：核心功能对齐 [DONE]
- [x] 3.1 配置管理 (Profile 列表、激活、切换重启)
- [x] 3.2 运行状态 (实时流量、内存、连接总数)
- [x] 3.3 代理节点管理 (网格化选择、分组显示)
- [x] 3.4 托盘菜单增强 (模式切换、系统代理/TUN 快捷开关、状态同步)

## 阶段四：高级功能冲刺 (Advanced Features) [DONE]
- [x] 4.1 代理测速增强 (⚡ 分组一键测速、节点延迟显示)
- [x] 4.2 实时连接审计 (流量降序排列、Rule/Payload 详情展示、单条/全局断开)
- [x] 4.3 规则管理优化 (实时搜索过滤、匹配数量统计)
- [x] 4.4 日志查看器优化 (日志级别筛选、自动滚动跟随新日志)
- [x] 4.5 流量走势图表 (Visual Traffic)
- [x] 4.6 订阅管理器 (Sub Management - URL 导入)
- [x] 4.7 结构化配置生成器 (Visual Config - DNS/TUN/Sniffer)
- [x] 4.8 **高级内核管理 (Core Management)**
  - [x] 检查最新版本 (Check Update)
  - [x] 下载并安装新版本 (Download with Progress)
  - [x] 删除旧版本内核 (Delete Version)
- [x] 4.9 **定时调度器 (Scheduler)**
  - [x] 自动订阅更新 (Hourly)
  - [x] 自动 WebDAV 同步 (Configurable Interval)
- [x] 4.10 **增强配置管理 (Profile Plus)**
  - [x] 导入本地配置文件 (Local File Import)
  - [x] 调用内置编辑器打开 (Built-in Editor)
  - [x] 配置文件过滤搜索 (Filter)
- [x] 4.11 **网络诊断与 Fake-IP (Network Diagnostics)**
  - [x] Fake-IP 详细配置 (Range, Filter)
  - [x] 清理 Fake-IP 缓存 (Flush Cache)
- [x] 4.12 **高级运行状态 (Runtime Details)**
  - [x] 内存使用情况显示 (Memory Usage)
  - [x] 当前出口 IP 信息显示 (Public IP Info)
- [x] 4.13 **提供者管理 (Providers)**
  - [x] Rule Providers 列表与手动更新
  - [x] Proxy Providers 列表与手动更新

## 阶段五：系统集成与发布 [DONE]
- [x] 5.1 窗口生命周期优化 (拦截 CloseRequested，实现托盘常驻/重开逻辑)
- [x] 5.2 系统代理与开机自启驱动
- [x] 5.3 单例启动 (Single Instance)
- [x] 5.4 路径规范化 (集成 mihomo-platform，符合各平台标准数据目录)
- [x] 5.5 **出厂设置重置 (Factory Reset)**
  - [x] 一键清理所有设置、配置、内核与日志

## 阶段六：后续优化 [PROGRESSING]
- [ ] 6.1 动画效果 (图标动效、转场过渡)
- [x] 6.2 打包工具链 (MSI 生成)
- [x] 6.3 **全局通知系统 (Toast Notifications)**
  - [x] 实现内置 Toast 提示，支持 Success/Error/Warning 状态

## 阶段七：精细化打磨与进阶配置 (Final Polish & Advanced Config) [DONE]
- [x] 7.1 **DNS 服务器列表深度管理 (DNS Server List Editor)**
  - [x] 动态增删 Nameserver / Fallback 节点
  - [x] 批量保存 DNS 配置到内核
- [x] 7.2 **规则手动编辑器 (Custom Rule Editor)**
  - [x] 支持图形化添加自定义 DOMAIN/IP-CIDR 规则
  - [x] 支持规则优先级调整逻辑
- [x] 7.3 **代理组增强交互 (Proxy Group Enhancements)**
  - [x] 实现 Proxies 视图的搜索与过滤
  - [x] 支持按延迟/名称排序
  - [x] 托盘菜单：支持在托盘直接切换 GLOBAL 代理组节点
- [x] 7.4 **UI 体验提升 (UX Polish)**
  - [x] 侧边栏菜单美化 (增加选中光标与背景动感)
  - [x] 管理员权限 (UAC) 状态可视化与一键提权重启
- [x] 7.5 **多语言 (i18n) 补充**
  - [x] 补全所有新功能词条 (Kernel Management, Providers, Sync 等)

## 阶段八：金标准稳定性与架构治理 (Gold Standard Reliability) [TODO]
- [ ] 8.1 **领域驱动错误处理 (Domain-Driven Error Handling)**
  - [ ] 在 `infiltrator-iced` 中引入自定义 `Error` 枚举，取代 `to_string()` 降级。
  - [ ] 实现针对不同错误的差异化 UI 反馈（如超时可重试，认证需检查）。
- [ ] 8.2 **依赖项深度清理与统一 (Dependency Cleanup)**
  - [ ] 移除陈旧的 `serde_yaml`，全工作区统一至 `yaml-rust2` 或 `serde_yml`。
  - [ ] 检查并清理各 Crate 中未使用的依赖项。
- [ ] 8.3 **协议定义细节补强 (Protocol Field Refinement)**
  - [ ] 补完 `Shadowsocks` (Plugin), `Vmess` (Header), `Wireguard` (MTU/UDP) 等协议的细颗粒度字段。
  - [ ] 编写更严苛的各种协议混合解析测试。
- [ ] 8.4 **健壮的异步流管理 (Robust Stream Management)**
  - [ ] 为所有 WebSocket 流 (Traffic, Logs, Conns) 引入 `AbortHandle` 或生命周期保护。
  - [ ] 优化核心重启时的自动重连逻辑，彻底杜绝僵尸连接。
- [ ] 8.5 **跨平台 UX 降级处理 (Cross-platform UX Resilience)**
  - [ ] 完善非 Windows 平台下的 `RequestAdminPrivilege` 交互逻辑（显示平台相关的权限获取指南）。
  - [ ] 针对不同 OS 优化默认路径显示和系统集成提示。
