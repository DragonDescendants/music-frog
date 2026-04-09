pub mod core;
pub mod profile;
pub mod ui;

use crate::state::AppState;
use crate::types::Message;
use iced::Task;

impl AppState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // UI & Navigation
            Message::Navigate(_)
            | Message::ToggleTheme
            | Message::WindowClosed(_)
            | Message::HideWindow
            | Message::ShowWindow
            | Message::Exit
            | Message::ShowToast(_, _)
            | Message::RemoveToast(_)
            | Message::SetSystemProxy(_)
            | Message::SystemProxySet(_) => self.update_ui(message),

            // Profiles & Sync
            Message::LoadProfiles
            | Message::ProfilesLoaded(_)
            | Message::SetActiveProfile(_)
            | Message::UpdateImportUrl(_)
            | Message::UpdateImportName(_)
            | Message::ImportProfile
            | Message::ProfileImported(_)
            | Message::EditProfile(_)
            | Message::ProfileContentLoaded(_)
            | Message::EditorAction(_)
            | Message::SaveProfile
            | Message::ProfileSaved(_)
            | Message::UpdateWebDavUrl(_)
            | Message::UpdateWebDavUser(_)
            | Message::UpdateWebDavPass(_)
            | Message::SyncUpload
            | Message::SyncDownload
            | Message::SyncFinished(_)
            | Message::TickSubUpdate
            | Message::TickWebDavSync => self.update_profile(message),

            // Core & Network
            _ => self.update_core(message),
        }
    }
}
