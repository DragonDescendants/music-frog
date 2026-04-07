mod autostart;
mod locales;
mod tray;
mod utils;
mod view;
mod types;
mod state;
mod subscription;
mod update;
mod view_root;
mod app;

pub use types::*;
pub use state::AppState;

use single_instance::SingleInstance;

pub fn main() -> iced::Result {
    let instance = SingleInstance::new("com.musicfrog.infiltrator").unwrap();
    if !instance.is_single() {
        return Ok(());
    }

    iced::application(AppState::new, AppState::update, AppState::view)
        .title(AppState::title)
        .theme(AppState::theme)
        .subscription(AppState::subscription)
        .exit_on_close_request(false)
        .run()
}

#[cfg(test)]
mod tests;
