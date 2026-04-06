use muda::{Menu, MenuItem, PredefinedMenuItem};
use std::path::Path;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

pub struct TrayManager {
    pub tray_icon: TrayIcon,
}

impl TrayManager {
    pub fn new() -> Self {
        let icon = load_icon();

        let tray_menu = Menu::new();
        let show_item = MenuItem::with_id("show", "显示窗口", true, None);
        let quit_item = MenuItem::with_id("quit", "退出", true, None);

        let _ = tray_menu.append_items(&[&show_item, &PredefinedMenuItem::separator(), &quit_item]);

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("MusicFrog Infiltrator")
            .with_icon(icon)
            .build()
            .unwrap();

        Self { tray_icon }
    }
}

fn load_icon() -> Icon {
    let path = Path::new("src-tauri/icons/icon.ico");
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open tray icon")
}
