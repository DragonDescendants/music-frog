use crate::state::AppState;
use crate::types::{Message, ToastStatus};
use iced::widget::text_editor;
use iced::Task;
use mihomo_config::{ConfigManager, Profile};
use dav_client::DavClient;

impl AppState {
    pub fn update_profile(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadProfiles => {
                self.is_loading_profiles = true;
                Task::perform(
                    async {
                        let cm = ConfigManager::new().map_err(|e: mihomo_api::MihomoError| e.to_string())?;
                        cm.list_profiles().await.map_err(|e: mihomo_api::MihomoError| e.to_string())
                    },
                    Message::ProfilesLoaded,
                )
            }
            Message::ProfilesLoaded(result) => {
                self.is_loading_profiles = false;
                match result {
                    Ok(p) => self.profiles = p,
                    Err(e) => self.error_msg = Some(e),
                }
                Task::none()
            }
            Message::SetActiveProfile(name) => Task::perform(
                async move {
                    if let Ok(cm) = ConfigManager::new() {
                        let _ = cm.set_current(&name).await;
                    }
                },
                |_| Message::LoadProfiles,
            ),
            Message::UpdateImportUrl(url) => {
                self.import_url = url;
                Task::none()
            }
            Message::UpdateImportName(name) => {
                self.import_name = name;
                Task::none()
            }
            Message::ImportProfile => {
                let url = self.import_url.trim().to_string();
                let name = self.import_name.trim().to_string();
                if url.is_empty() || name.is_empty() {
                    self.error_msg = Some("URL and Name cannot be empty".to_string());
                    return Task::none();
                }
                self.is_importing = true;
                Task::perform(
                    async move {
                        let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build().map_err(|e| e.to_string())?;
                        let content = client.get(&url).send().await.map_err(|e| e.to_string())?.text().await.map_err(|e| e.to_string())?;
                        let cm = ConfigManager::new().map_err(|e: mihomo_api::MihomoError| e.to_string())?;
                        cm.save(&name, &content).await.map_err(|e: mihomo_api::MihomoError| e.to_string())?;
                        
                        let mut profile = Profile::new(name.clone(), std::path::PathBuf::new(), false);
                        profile.subscription_url = Some(url);
                        profile.auto_update_enabled = true;
                        let _ = cm.update_profile_metadata(&name, &profile).await;
                        Ok(())
                    },
                    Message::ProfileImported,
                )
            }
            Message::ProfileImported(result) => {
                self.is_importing = false;
                match result {
                    Ok(_) => {
                        self.import_url.clear();
                        self.import_name.clear();
                        return Task::batch(vec![
                            Task::done(Message::LoadProfiles),
                            Task::done(Message::ShowToast("Profile imported successfully".to_string(), ToastStatus::Success))
                        ]);
                    }
                    Err(e) => {
                        self.error_msg = Some(e.clone());
                        return Task::done(Message::ShowToast(e, ToastStatus::Error));
                    }
                }
            }
            Message::EditProfile(path) => {
                self.editor_path = Some(path.clone());
                Task::perform(
                    async move {
                        let content = tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())?;
                        Ok((path, content))
                    },
                    Message::ProfileContentLoaded,
                )
            }
            Message::ProfileContentLoaded(result) => match result {
                Ok((_, content)) => {
                    self.editor_content = text_editor::Content::with_text(&content);
                    Task::none()
                }
                Err(e) => {
                    self.error_msg = Some(e);
                    Task::none()
                }
            },
            Message::EditorAction(action) => {
                self.editor_content.perform(action);
                Task::none()
            }
            Message::SaveProfile => {
                if let Some(path) = self.editor_path.clone() {
                    let content = self.editor_content.text();
                    Task::perform(
                        async move {
                            tokio::fs::write(path, content).await.map_err(|e: std::io::Error| e.to_string())
                        },
                        Message::ProfileSaved,
                    )
                } else {
                    Task::none()
                }
            }
            Message::ProfileSaved(result) => {
                if let Err(e) = result { 
                    self.error_msg = Some(e.clone());
                    return Task::done(Message::ShowToast(e, ToastStatus::Error));
                }
                Task::done(Message::ShowToast("Profile saved".to_string(), ToastStatus::Success))
            }
            Message::UpdateWebDavUrl(s) => { self.webdav_url = s; Task::none() }
            Message::UpdateWebDavUser(s) => { self.webdav_user = s; Task::none() }
            Message::UpdateWebDavPass(s) => { self.webdav_pass = s; Task::none() }
            Message::SyncUpload => {
                self.is_syncing = true;
                let url = self.webdav_url.clone();
                let user = self.webdav_user.clone();
                let pass = self.webdav_pass.clone();
                Task::perform(
                    async move {
                        let client = dav_client::client::WebDavClient::new(&url, &user, &pass).map_err(|e: anyhow::Error| e.to_string())?;
                        let data_dir = mihomo_platform::get_home_dir().map_err(|e: mihomo_api::MihomoError| e.to_string())?;
                        let settings_path = data_dir.join("settings.toml");
                        if settings_path.exists() {
                            let content = tokio::fs::read(settings_path).await.map_err(|e: std::io::Error| e.to_string())?;
                            let _ = client.put("settings.toml", &content, None).await.map_err(|e: anyhow::Error| e.to_string())?;
                        }
                        Ok(())
                    },
                    Message::SyncFinished,
                )
            }
            Message::SyncDownload => {
                self.is_syncing = true;
                let url = self.webdav_url.clone();
                let user = self.webdav_user.clone();
                let pass = self.webdav_pass.clone();
                Task::perform(
                    async move {
                        let client = dav_client::client::WebDavClient::new(&url, &user, &pass).map_err(|e: anyhow::Error| e.to_string())?;
                        let data_dir = mihomo_platform::get_home_dir().map_err(|e: mihomo_api::MihomoError| e.to_string())?;
                        if let Ok(content) = client.get("settings.toml").await {
                            tokio::fs::write(data_dir.join("settings.toml"), content).await.map_err(|e: std::io::Error| e.to_string())?;
                        }
                        Ok(())
                    },
                    Message::SyncFinished,
                )
            }
            Message::SyncFinished(result) => {
                self.is_syncing = false;
                match result {
                    Ok(_) => return Task::done(Message::ShowToast("Sync successful".to_string(), ToastStatus::Success)),
                    Err(e) => {
                        self.error_msg = Some(e.clone());
                        return Task::done(Message::ShowToast(e, ToastStatus::Error));
                    }
                }
            }
            Message::TickSubUpdate => {
                let profiles_to_update: Vec<_> = self.profiles.iter()
                    .filter(|p| p.auto_update_enabled && p.subscription_url.is_some())
                    .cloned()
                    .collect();
                if profiles_to_update.is_empty() { return Task::none(); }
                Task::batch(profiles_to_update.into_iter().map(|p| {
                    let url = p.subscription_url.unwrap();
                    let name = p.name;
                    Task::perform(
                        async move {
                            let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build().map_err(|e| e.to_string())?;
                            let content = client.get(&url).send().await.map_err(|e| e.to_string())?.text().await.map_err(|e| e.to_string())?;
                            let cm = ConfigManager::new().map_err(|e: mihomo_api::MihomoError| e.to_string())?;
                            cm.save(&name, &content).await.map_err(|e: mihomo_api::MihomoError| e.to_string())
                        },
                        Message::ProfileImported,
                    )
                }))
            }
            Message::TickWebDavSync => {
                if !self.webdav_url.is_empty() && !self.webdav_user.is_empty() { return Task::done(Message::SyncUpload); }
                Task::none()
            }
            _ => Task::none(),
        }
    }
}
