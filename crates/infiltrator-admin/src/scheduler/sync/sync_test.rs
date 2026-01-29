#[cfg(test)]
mod tests {
    use super::super::run_sync_tick;
    use infiltrator_core::settings::WebDavConfig;
    use crate::admin_api::AdminApiContext;
    use infiltrator_core::AppSettings;
    use crate::TEST_LOCK;

    #[derive(Clone)]
    struct MockContext;

    #[async_trait::async_trait]
    impl AdminApiContext for MockContext {
        async fn notify_subscription_update(&self, _profile: String, _success: bool, _message: Option<String>) {}
        async fn rebuild_runtime(&self) -> anyhow::Result<()> { Ok(()) }
        async fn set_use_bundled_core(&self, _enabled: bool) {}
        async fn refresh_core_version_info(&self) {}
        async fn editor_path(&self) -> Option<String> { None }
        async fn set_editor_path(&self, _path: Option<String>) {}
        async fn pick_editor_path(&self) -> Option<String> { None }
        async fn open_profile_in_editor(&self, _profile_name: &str) -> anyhow::Result<()> { Ok(()) }
        async fn get_app_settings(&self) -> AppSettings { AppSettings::default() }
        async fn save_app_settings(&self, _settings: AppSettings) -> anyhow::Result<()> { Ok(()) }
    }

    #[tokio::test]
    async fn test_run_sync_tick_disabled() {
        let ctx = MockContext;
        let config = WebDavConfig::default(); // default enabled is false
        
        let result = run_sync_tick(&ctx, &config).await;
        assert!(result.is_ok());
        let summary = result.unwrap();
        assert_eq!(summary.total_actions, 0);
    }

    #[tokio::test]
    async fn test_run_sync_tick_empty_url() {
        let ctx = MockContext;
        let mut config = WebDavConfig::default();
        config.enabled = true;
        config.url = "".to_string();
        
        let result = run_sync_tick(&ctx, &config).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "WebDAV URL is empty");
    }

    #[tokio::test]
    async fn test_run_sync_tick_invalid_url() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        mihomo_platform::set_home_dir_override(temp_dir.path().to_path_buf());

        let ctx = MockContext;
        let mut config = WebDavConfig::default();
        config.enabled = true;
        config.url = "not-a-url".to_string();
        
        let result = run_sync_tick(&ctx, &config).await;
        // Should fail during client creation or plan building
        assert!(result.is_err());
        
        mihomo_platform::clear_home_dir_override();
    }
}