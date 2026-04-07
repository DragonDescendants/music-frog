mod app;
mod autostart;
mod locales;
mod state;
mod subscription;
mod tray;
mod types;
mod update;
mod utils;
mod view;
mod view_root;

pub use state::AppState;
pub use types::*;

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
