use crate::state::AppState;
use crate::types::{InfiltratorError, Message, ToastStatus};
use dav_client::{DavClient, client::WebDavClient};
use iced::Task;
use iced::widget::text_editor;
use mihomo_config::ConfigManager;

impl AppState {
    pub fn update_profile(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadProfiles => {
                self.is_loading_profiles = true;
                Task::perform(
                    async {
                        let cm = ConfigManager::new().map_err(|e: mihomo_api::MihomoError| {
                            InfiltratorError::Mihomo(e.to_string())
                        })?;
                        cm.list_profiles()
                            .await
                            .map_err(|e: mihomo_api::MihomoError| {
                                InfiltratorError::Mihomo(e.to_string())
                            })
                    },
                    Message::ProfilesLoaded,
                )
            }
            Message::ProfilesLoaded(result) => {
                self.is_loading_profiles = false;
                match result {
                    Ok(profiles) => {
                        self.profiles = profiles;
                        Task::none()
                    }
                    Err(e) => {
                        self.error_msg = Some(e.to_string());
                        Task::none()
                    }
                }
            }
            Message::SetActiveProfile(name) => {
                self.error_msg = None;
                Task::perform(
                    async move {
                        let cm = ConfigManager::new().map_err(|e: mihomo_api::MihomoError| {
                            InfiltratorError::Mihomo(e.to_string())
                        })?;
                        cm.set_current(&name)
                            .await
                            .map_err(|e: mihomo_api::MihomoError| {
                                InfiltratorError::Mihomo(e.to_string())
                            })?;
                        Ok(())
                    },
                    |result: Result<(), InfiltratorError>| match result {
                        Ok(_) => Message::StartProxy,
                        Err(e) => Message::ShowToast(e.to_string(), ToastStatus::Error),
                    },
                )
            }
            Message::UpdateImportUrl(url) => {
                self.import_url = url;
                Task::none()
            }
            Message::UpdateImportName(name) => {
                self.import_name = name;
                Task::none()
            }
            Message::ImportProfile => {
                let url = self.import_url.clone();
                if url.is_empty() {
                    return Task::none();
                }
                self.is_importing = true;
                Task::perform(
                    async move {
                        let _profile_info =
                            infiltrator_core::profiles::create_profile_from_url("Downloaded", &url)
                                .await
                                .map_err(|e| InfiltratorError::Mihomo(e.to_string()))?;
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
                        Task::batch(vec![
                            Task::done(Message::LoadProfiles),
                            Task::done(Message::ShowToast(
                                "Profile imported successfully".to_string(),
                                ToastStatus::Success,
                            )),
                        ])
                    }
                    Err(e) => {
                        self.error_msg = Some(e.to_string());
                        Task::done(Message::ShowToast(e.to_string(), ToastStatus::Error))
                    }
                }
            }
            Message::EditProfile(path) => {
                let p = path.clone();
                Task::perform(
                    async move {
                        let content = tokio::fs::read_to_string(&p)
                            .await
                            .map_err(|e| InfiltratorError::Io(e.to_string()))?;
                        Ok((p, content))
                    },
                    Message::ProfileContentLoaded,
                )
            }
            Message::ProfileContentLoaded(result) => match result {
                Ok((path, content)) => {
                    self.editor_path = Some(path);
                    self.editor_content = text_editor::Content::with_text(&content);
                    Task::done(Message::Navigate(crate::types::Route::Editor))
                }
                Err(e) => {
                    self.error_msg = Some(e.to_string());
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
                            tokio::fs::write(&path, content)
                                .await
                                .map_err(|e| InfiltratorError::Io(e.to_string()))?;
                            Ok(())
                        },
                        Message::ProfileSaved,
                    )
                } else {
                    Task::none()
                }
            }
            Message::ProfileSaved(result) => match result {
                Ok(_) => Task::done(Message::ShowToast(
                    "Profile saved".to_string(),
                    ToastStatus::Success,
                )),
                Err(e) => {
                    self.error_msg = Some(e.to_string());
                    Task::done(Message::ShowToast(e.to_string(), ToastStatus::Error))
                }
            },
            Message::UpdateWebDavUrl(url) => {
                self.webdav_url = url;
                Task::none()
            }
            Message::UpdateWebDavUser(user) => {
                self.webdav_user = user;
                Task::none()
            }
            Message::UpdateWebDavPass(pass) => {
                self.webdav_pass = pass;
                Task::none()
            }
            Message::SyncUpload => {
                let url = self.webdav_url.clone();
                let user = self.webdav_user.clone();
                let pass = self.webdav_pass.clone();
                self.is_syncing = true;
                Task::perform(
                    async move {
                        let client = WebDavClient::new(&url, &user, &pass)
                            .map_err(|e| InfiltratorError::Sync(e.to_string()))?;
                        let cm = ConfigManager::new().map_err(|e: mihomo_api::MihomoError| {
                            InfiltratorError::Mihomo(e.to_string())
                        })?;
                        let profiles =
                            cm.list_profiles()
                                .await
                                .map_err(|e: mihomo_api::MihomoError| {
                                    InfiltratorError::Mihomo(e.to_string())
                                })?;

                        for profile in profiles {
                            let content = tokio::fs::read_to_string(&profile.path)
                                .await
                                .map_err(|e| InfiltratorError::Io(e.to_string()))?;
                            client
                                .put(&format!("{}.yaml", profile.name), content.as_bytes(), None)
                                .await
                                .map_err(|e| InfiltratorError::Sync(e.to_string()))?;
                        }
                        Ok(())
                    },
                    Message::SyncFinished,
                )
            }
            Message::SyncDownload => {
                let url = self.webdav_url.clone();
                let user = self.webdav_user.clone();
                let pass = self.webdav_pass.clone();
                self.is_syncing = true;
                Task::perform(
                    async move {
                        let client = WebDavClient::new(&url, &user, &pass)
                            .map_err(|e| InfiltratorError::Sync(e.to_string()))?;
                        let files = client
                            .list("")
                            .await
                            .map_err(|e| InfiltratorError::Sync(e.to_string()))?;
                        let _cm = ConfigManager::new().map_err(|e: mihomo_api::MihomoError| {
                            InfiltratorError::Mihomo(e.to_string())
                        })?;

                        for file in files {
                            if file.path.ends_with(".yaml") {
                                let content = client
                                    .get(&file.path)
                                    .await
                                    .map_err(|e| InfiltratorError::Sync(e.to_string()))?;
                                let data_dir = mihomo_platform::get_home_dir().map_err(
                                    |e: mihomo_api::MihomoError| {
                                        InfiltratorError::Mihomo(e.to_string())
                                    },
                                )?;
                                let path = data_dir.join("profiles").join(file.path);
                                tokio::fs::write(path, content)
                                    .await
                                    .map_err(|e| InfiltratorError::Io(e.to_string()))?;
                            }
                        }
                        Ok(())
                    },
                    Message::SyncFinished,
                )
            }
            Message::SyncFinished(result) => {
                self.is_syncing = false;
                match result {
                    Ok(_) => Task::done(Message::ShowToast(
                        "Sync completed".to_string(),
                        ToastStatus::Success,
                    )),
                    Err(e) => {
                        self.error_msg = Some(e.to_string());
                        Task::done(Message::ShowToast(e.to_string(), ToastStatus::Error))
                    }
                }
            }
            Message::UpdateNewRuleType(t) => {
                self.new_rule_type = t;
                Task::none()
            }
            Message::UpdateNewRulePayload(p) => {
                self.new_rule_payload = p;
                Task::none()
            }
            Message::UpdateNewRuleTarget(t) => {
                self.new_rule_target = t;
                Task::none()
            }
            Message::AddCustomRule => {
                let rule_type = self.new_rule_type.clone();
                let payload = self.new_rule_payload.trim().to_string();
                let target = self.new_rule_target.clone();

                if payload.is_empty() {
                    return Task::done(Message::ShowToast(
                        "Payload cannot be empty".to_string(),
                        ToastStatus::Error,
                    ));
                }

                self.is_adding_rule = true;
                Task::perform(
                    async move {
                        let cm = ConfigManager::new().map_err(|e: mihomo_api::MihomoError| {
                            InfiltratorError::Mihomo(e.to_string())
                        })?;
                        let current_name =
                            cm.get_current()
                                .await
                                .map_err(|e: mihomo_api::MihomoError| {
                                    InfiltratorError::Mihomo(e.to_string())
                                })?;

                        let profiles =
                            cm.list_profiles()
                                .await
                                .map_err(|e: mihomo_api::MihomoError| {
                                    InfiltratorError::Mihomo(e.to_string())
                                })?;
                        let profile = profiles
                            .iter()
                            .find(|p| p.name == current_name)
                            .ok_or_else(|| {
                                InfiltratorError::Config("Profile not found".to_string())
                            })?;

                        let content = tokio::fs::read_to_string(&profile.path)
                            .await
                            .map_err(|e: std::io::Error| InfiltratorError::Io(e.to_string()))?;
                        let mut yaml: serde_yml::Value =
                            serde_yml::from_str(&content).map_err(|e: serde_yml::Error| {
                                InfiltratorError::Config(e.to_string())
                            })?;

                        let rules = yaml
                            .get_mut("rules")
                            .and_then(|v| v.as_sequence_mut())
                            .ok_or_else(|| {
                                InfiltratorError::Config("No rules found in profile".to_string())
                            })?;

                        let new_rule = serde_yml::Value::String(format!(
                            "{},{},{}",
                            rule_type, payload, target
                        ));
                        rules.insert(0, new_rule);

                        let new_content =
                            serde_yml::to_string(&yaml).map_err(|e: serde_yml::Error| {
                                InfiltratorError::Config(e.to_string())
                            })?;
                        tokio::fs::write(&profile.path, new_content)
                            .await
                            .map_err(|e: std::io::Error| InfiltratorError::Io(e.to_string()))?;

                        Ok(())
                    },
                    Message::RuleAdded,
                )
            }
            Message::RuleAdded(result) => {
                self.is_adding_rule = false;
                match result {
                    Ok(_) => {
                        self.new_rule_payload.clear();
                        Task::batch(vec![
                            Task::done(Message::LoadRules),
                            Task::done(Message::StartProxy),
                            Task::done(Message::ShowToast(
                                "Rule added and core restarted".to_string(),
                                ToastStatus::Success,
                            )),
                        ])
                    }
                    Err(e) => {
                        self.error_msg = Some(e.to_string());
                        Task::done(Message::ShowToast(e.to_string(), ToastStatus::Error))
                    }
                }
            }
            Message::TickWebDavSync => {
                if !self.webdav_url.is_empty() && !self.webdav_user.is_empty() {
                    return Task::done(Message::SyncUpload);
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }
}
