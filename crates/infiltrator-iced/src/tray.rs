use muda::{Menu, MenuItem, PredefinedMenuItem, CheckMenuItem, Submenu};
use std::path::Path;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

pub struct TrayManager {
    pub tray_icon: TrayIcon,
    pub menu: Menu,
    pub system_proxy_item: CheckMenuItem,
    pub tun_mode_item: CheckMenuItem,
}

impl TrayManager {
    pub fn new() -> Self {
        let icon = load_icon();

        let menu = Menu::new();
        let show_item = MenuItem::with_id("show", "显示主界面", true, None);
        
        let mode_menu = Submenu::new("代理模式", true);
        let mode_rule = MenuItem::with_id("mode_rule", "规则模式", true, None);
        let mode_global = MenuItem::with_id("mode_global", "全局模式", true, None);
        let mode_direct = MenuItem::with_id("mode_direct", "直连模式", true, None);
        let _ = mode_menu.append_items(&[&mode_rule, &mode_global, &mode_direct]);

        let system_proxy_item = CheckMenuItem::with_id("toggle_system_proxy", "系统代理", true, false, None);
        let tun_mode_item = CheckMenuItem::with_id("toggle_tun", "TUN 模式", true, false, None);
        let theme_item = MenuItem::with_id("toggle_theme", "切换深/浅色模式", true, None);

        let quit_item = MenuItem::with_id("quit", "退出应用", true, None);

        let _ = menu.append_items(&[
            &show_item,
            &PredefinedMenuItem::separator(),
            &mode_menu,
            &system_proxy_item,
            &tun_mode_item,
            &theme_item,
            &PredefinedMenuItem::separator(),
            &quit_item
        ]);

        let mut builder = TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip("MusicFrog Infiltrator");

        if let Some(i) = icon {
            builder = builder.with_icon(i);
        }

        let tray_icon = builder.build().unwrap();

        Self { 
            tray_icon, 
            menu, 
            system_proxy_item, 
            tun_mode_item 
        }
    }

    pub fn update_status(&self, system_proxy: bool, tun: bool) {
        let _ = self.system_proxy_item.set_checked(system_proxy);
        let _ = self.tun_mode_item.set_checked(tun);
    }
}

fn load_icon() -> Option<Icon> {
    let path = Path::new("src-tauri/icons/icon.ico");
    if !path.exists() {
        let path_png = Path::new("src-tauri/icons/icon.png");
        if !path_png.exists() {
            return None;
        }
        return load_icon_from_path(path_png);
    }
    load_icon_from_path(path)
}

fn load_icon_from_path(path: &Path) -> Option<Icon> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path).ok()?.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).ok()
}
