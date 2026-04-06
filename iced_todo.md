# Iced 跨平台原生 UI 改造计划 (Iced Migration Todo)

本项目旨在基于 `iced` (纯 Rust GUI 框架) 为 MusicFrog Despicable Infiltrator 开发一个高性能、低资源占用的原生跨平台客户端，作为现有基于 Webview 的 Tauri 前端的替代方案。

**核心指导原则**：
1.  **功能对等**：保持现有 Tauri 客户端的所有核心功能不受影响。
2.  **视觉连贯**：界面布局（左侧导航、顶部状态、右侧内容区）和设计风格需与当前 Vue 3 + Tailwind CSS 的版本尽可能保持一致。
3.  **架构复用**：最大程度复用现有的 `infiltrator-*` 和 `mihomo-*` crates，移除对 HTTP (Axum) 和 Tauri Commands 的依赖，改为直接调用 Rust 底层 API。
4.  **由内而外**：优先实现核心的代理启停、配置切换和流量监控，再逐步完善高级设置和系统集成特性。

---

## 阶段一：工程基础设施与骨架搭建 (Foundation)

目标：建立 `infiltrator-iced` crate，跑通 `iced` 的基本应用生命周期，并成功桥接异步运行时（Tokio）与底层核心库。

- [x] **1.1 初始化工程**
  - 在 `crates/` 目录下创建 `infiltrator-iced` crate (`cargo new crates/infiltrator-iced --bin`)。
  - 在根 `Cargo.toml` 中将其加入 `[workspace.members]`。
  - 配置 `infiltrator-iced/Cargo.toml`，引入 `iced` (推荐使用最新稳定版或 git 版以获取更佳支持)、`tokio` 以及现有的 `infiltrator-core`、`mihomo-api` 等核心依赖。
- [x] **1.2 搭建 Iced Application 骨架**
  - 创建 `src/main.rs`，实现 `iced::Application` trait (或更现代的 `iced::Program`，视版本而定)。
  - 定义全局 `State` 结构体，用于存储应用的当前页面、主题、核心运行状态等。
  - 定义全局 `Message` 枚举，用于处理用户交互和后台事件。
- [x] **1.3 异步桥接与核心启停控制**
  - 在 `Application::new` 阶段初始化 Tokio 运行时或将 Iced 与全局 Tokio 运行时绑定。
  - 实现“启动内核”和“停止内核”的基础逻辑，验证能否通过界面按钮直接控制 `infiltrator-core`。

## 阶段二：主界面布局与路由导航 (Layout & Routing)

目标：构建与 WebUI 相似的三栏/两栏经典布局结构。

- [x] **2.1 主体框架开发**
  - 使用 `iced::widget::row` 和 `iced::widget::column` 构建主界面：
    - 左侧侧边栏 (Sidebar)：用于放置导航菜单。
    - 右侧主内容区 (Main Content)：用于渲染具体页面的视图。
    - 顶部状态栏 (Status Bar) (可选，视具体设计而定，也可融入侧边栏顶部或内容区顶部)。
- [x] **2.2 路由与状态管理**
  - 在 `State` 中添加 `current_route` 字段（枚举类型），表示当前选中的页面（如：Profiles, Runtime, Settings 等）。
  - 实现侧边栏的点击事件 (`Message::NavigateTo(Route)`)，更新状态并触发内容区重绘。
- [x] **2.3 样式系统基础 (Theme)**
  - 基于现有的 Tailwind 设计稿，定义 `iced` 的自定义 `Theme` 或 `StyleSheet`。
  - 预先设定好基础的颜色变量（背景色、前景色、主色调等），支持浅色和深色模式切换的基础结构。
  - 引入图标库（例如使用 Phosphor Icons 或 Bootstrap Icons 的字体文件并在 Iced 中加载）。

## 阶段三：核心功能模块集成 (Core Features)

目标：实现代理客户端最基础、最关键的使用场景。

- [x] **3.1 Profiles & Imports (配置管理)**
  - 读取本地配置列表（复用 `mihomo-config`）。
  - 在界面上渲染配置列表（List 视图）。
  - 实现配置的选择（Set Active）与核心的联动重启。
  - （可选）初步实现通过 URL 导入订阅的功能弹窗。
- [x] **3.2 Runtime Status (运行态监控)**
  - **连接列表**：使用 `iced::Subscription` 监听后台连接数据的变化，并在界面上使用表格或列表渲染。需要考虑过滤 (Filter) 功能的实现。
  - **流量与内存**：同样使用 `Subscription` 获取实时流量速度和内存占用，在界面顶部或专属面板以文本或简易图表展示。
- [x] **3.3 Core & TUN (内核控制)**
  - 核心版本切换的基础 UI。
  - TUN 模式的开启/关闭开关，直接调用底层 `infiltrator-core` 设定并重启。

## 阶段四：高级功能与细节完善 (Advanced & Polish)

目标：补齐剩余的业务功能，处理大数据量的渲染性能。

- [x] **4.1 Rules & Providers (规则管理)**
  - [x] 方案设计：集成 `Rules` 路由与侧边栏导航。
  - [x] 数据逻辑：实现异步获取 `mihomo` 规则列表并支持本地关键字过滤。
  - [x] 视图实现：编写 `view/rules.rs`，使用高性能容器渲染规则卡片。
  - [x] 自动化验证：增加规则过滤算法的单元测试。
  - [x] 功能收口：完成 Clippy 检查与最终验证。
- [ ] **4.2 DNS & Fake-IP 设置面板**
  - [ ] 方案设计：新建 `view/dns.rs`，设计 DNS 服务器列表与 Fake-IP 开关的 UI 布局。
  - [ ] 数据同步：实现从核心获取 DNS 配置并绑定到 UI 表单。
  - [ ] 编辑交互：实现 DNS 服务器地址的动态增加、删除与实时校验。
  - [ ] 配置保存：实现 Patch 逻辑，将修改后的 DNS 设定推送到核心并持久化。
  - [ ] 洁癖收口：全量执行 Clippy 与 Fmt 检查，消除所有警告。
  - [ ] 知识沉淀：记录 Iced 表单组件与 Patch 逻辑的最佳实践到 `GEMINI.md`。
- [ ] **4.3 WebDAV 同步 (Sync)**
  - 实现 WebDAV 设置表单。
  - 集成手动同步和连接测试按钮。
- [x] **4.4 日志查看器 (Logs)**
  - 监听核心日志流，并在一个滚动区域内渲染日志。
  - 实现日志级别的切换和自动滚动到底部的逻辑。

## 阶段五：桌面系统级特性集成 (System Integration)

目标：脱离 Tauri 后，重新实现桌面应用必备的系统级交互。

- [x] **5.1 独立系统托盘 (System Tray)**
  - [x] 方案设计：建立 `src/tray.rs` 模块，设计托盘事件与 Iced Message 的桥接机制。
  - [x] 托盘初始化：实现托盘图标加载与右键菜单（显示/退出/模式切换）构建。
  - [x] 事件监听：编写 `Subscription` 实时捕获托盘点击与菜单选择。
  - [x] 窗口联动：实现点击托盘恢复窗口及彻底退出应用的逻辑。
  - [x] 功能收口：完成 Clippy 检查与最终验证。
- [ ] **5.2 系统代理控制 (System Proxy)**
  - 桥接现有的 Windows/macOS/Linux 系统代理设置逻辑（复用 `src-tauri` 中抽离的代码，如果有的话，或直接调用 `infiltrator-core`/平台 API）。
- [x] **5.3 单例运行与开机自启**
  - [x] 方案设计：采用命名管道（Windows）或 Unix Domain Socket 实现进程单例。
  - [x] 通讯机制：实现次实例启动时通过管道唤醒主实例窗口并自动退出。
  - [ ] 开机自启：实现注册表（Windows）写入逻辑。
  - [x] 功能收口：完成全量测试。
- [ ] **5.4 外部交互**
  - 调用外部编辑器打开配置文件的功能。
  - 打开浏览器（跳转到相关网页）。

## 阶段六：最终优化与发布准备 (Final Polish)

- [ ] **6.1 性能与资源优化**
  - 检查内存泄漏，优化高频更新的 UI 组件（如流量监控和日志），避免全屏无意义的重绘。
- [x] **6.2 国际化 (I18n)**
  - 接入本地化文本，支持中英文界面的实时切换。
- [ ] **6.3 打包与分发**
  - 配置替代 Tauri 的打包工具链（如 `cargo-packager` 或 `cargo-bundle`），生成 MSI、DMG、AppImage 等安装包。
- [ ] **6.4 清理与过渡**
  - 在确保 Iced 版本完全可用后，逐步考虑废弃或精简原有的 Tauri 相关代码（视项目规划而定）。
