#[cfg(test)]
mod tests {
    use axum::
    {
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`, `ready`, and `call`
    use std::sync::{Arc, Mutex};
    use crate::admin_api::*;
    use infiltrator_core::AppSettings;
    use crate::TEST_LOCK;

    #[derive(Clone)]
    struct MockContext {
        rebuild_count: Arc<Mutex<usize>>,
    }

    #[async_trait::async_trait]
    impl AdminApiContext for MockContext {
        async fn rebuild_runtime(&self) -> anyhow::Result<()> {
            let mut count = self.rebuild_count.lock().unwrap_or_else(|e| e.into_inner());
            *count += 1;
            Ok(())
        }
        async fn set_use_bundled_core(&self, _enabled: bool) {}
        async fn refresh_core_version_info(&self) {}
        async fn notify_subscription_update(&self, _p: String, _s: bool, _m: Option<String>) {}
        async fn editor_path(&self) -> Option<String> { None }
        async fn set_editor_path(&self, _path: Option<String>) {}
        async fn pick_editor_path(&self) -> Option<String> { None }
        async fn open_profile_in_editor(&self, _name: &str) -> anyhow::Result<()> { Ok(()) }
        async fn get_app_settings(&self) -> AppSettings { AppSettings::default() }
        async fn save_app_settings(&self, _s: AppSettings) -> anyhow::Result<()> { Ok(()) }
    }

    fn setup_app() -> axum::Router {
        let ctx = MockContext {
            rebuild_count: Arc::new(Mutex::new(0)),
        };
        let bus = events::AdminEventBus::new();
        let state = AdminApiState::new(ctx, bus);
        router(state)
    }

    #[tokio::test]
    async fn test_get_profiles_route() {
        let _guard = TEST_LOCK.lock().await;
        let temp_dir = tempfile::tempdir().unwrap();
        mihomo_platform::set_home_dir_override(temp_dir.path().to_path_buf());

        let app = setup_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/admin/api/profiles")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["content-type"], "application/json");
        
        mihomo_platform::clear_home_dir_override();
    }

    #[tokio::test]
    async fn test_get_settings_route() {
        let _guard = TEST_LOCK.lock().await;
        let temp_dir = tempfile::tempdir().unwrap();
        mihomo_platform::set_home_dir_override(temp_dir.path().to_path_buf());

        let app = setup_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/admin/api/settings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        
        mihomo_platform::clear_home_dir_override();
    }

    #[tokio::test]
    async fn test_import_profile_integration() {
        let _guard = TEST_LOCK.lock().await;
        
        let mut server = mockito::Server::new_async().await;
        let mock_yaml = "port: 7890\nmode: rule";
        let _m = server.mock("GET", "/sub")
            .with_status(200)
            .with_body(mock_yaml)
            .create_async().await;

        let temp_dir = tempfile::tempdir().unwrap();
        mihomo_platform::set_home_dir_override(temp_dir.path().to_path_buf());

        let app = setup_app();
        let payload = ImportProfilePayload {
            name: "test-import".to_string(),
            url: format!("{}/sub", server.url()),
            activate: Some(true),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/admin/api/profiles/import")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = response.status();
        let body_bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let body_str = String::from_utf8_lossy(&body_bytes);
        
        if status != StatusCode::OK {
            panic!("Import profile failed with status {}. Body: {}", status, body_str);
        }
        
        // Verify file was saved
        let config_path = temp_dir.path().join("configs").join("test-import.yaml");
        assert!(config_path.exists());
        
        mihomo_platform::clear_home_dir_override();
    }

    #[tokio::test]
    async fn test_save_invalid_yaml_returns_400() {
        let _guard = TEST_LOCK.lock().await;
        let app = setup_app();
        let payload = SaveProfilePayload {
            name: "invalid-yaml".to_string(),
            content: "key: : : : value".to_string(), // Invalid YAML
            activate: Some(false),
        };

        let response = app.oneshot(
            Request::builder()
                .method("POST")
                .uri("/admin/api/profiles/save")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        ).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        mihomo_platform::clear_home_dir_override();
    }

    #[tokio::test]
    async fn test_switch_nonexistent_profile_returns_error() {
        let _guard = TEST_LOCK.lock().await;
        let temp_dir = tempfile::tempdir().unwrap();
        mihomo_platform::set_home_dir_override(temp_dir.path().to_path_buf());
        
        let app = setup_app();
        let payload = SwitchProfilePayload {
            name: "i-do-not-exist".to_string(),
        };

        let response = app.oneshot(
            Request::builder()
                .method("POST")
                .uri("/admin/api/profiles/switch")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        ).await.unwrap();

        assert!(response.status().is_client_error() || response.status().is_server_error());
        mihomo_platform::clear_home_dir_override();
    }

    #[tokio::test]
    async fn test_delete_active_profile_rejected() {
        let _guard = TEST_LOCK.lock().await;
        let temp_dir = tempfile::tempdir().unwrap();
        mihomo_platform::set_home_dir_override(temp_dir.path().to_path_buf());
        
        let manager = mihomo_config::ConfigManager::new().unwrap();
        manager.save("active", "port: 7890").await.unwrap();
        manager.set_current("active").await.unwrap();

        let app = setup_app();
        let response = app.oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/admin/api/profiles/active")
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        mihomo_platform::clear_home_dir_override();
    }

    #[tokio::test]
    async fn test_get_rebuild_status_reflects_reality() {
        let _guard = TEST_LOCK.lock().await;
        let app = setup_app();

        let response = app.oneshot(
            Request::builder()
                .uri("/admin/api/rebuild/status")
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), 1024).await.unwrap()
        ).unwrap();
        
        assert_eq!(body["in_progress"], false);
        mihomo_platform::clear_home_dir_override();
    }

    #[tokio::test]
    async fn test_get_dns_config_route() {
        let _guard = TEST_LOCK.lock().await;
        let temp_dir = tempfile::tempdir().unwrap();
        mihomo_platform::set_home_dir_override(temp_dir.path().to_path_buf());
        let manager = mihomo_config::ConfigManager::new().unwrap();
        manager.save("default", "dns:\n  enable: true").await.unwrap();

        let app = setup_app();
        let response = app.oneshot(
            Request::builder().uri("/admin/api/dns").body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        mihomo_platform::clear_home_dir_override();
    }

    #[tokio::test]
    async fn test_get_tun_config_route() {
        let _guard = TEST_LOCK.lock().await;
        let temp_dir = tempfile::tempdir().unwrap();
        mihomo_platform::set_home_dir_override(temp_dir.path().to_path_buf());
        let manager = mihomo_config::ConfigManager::new().unwrap();
        manager.save("default", "tun:\n  enable: true").await.unwrap();

        let app = setup_app();
        let response = app.oneshot(
            Request::builder().uri("/admin/api/tun").body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        mihomo_platform::clear_home_dir_override();
    }

    #[tokio::test]
    async fn test_get_rules_route() {
        let _guard = TEST_LOCK.lock().await;
        let temp_dir = tempfile::tempdir().unwrap();
        mihomo_platform::set_home_dir_override(temp_dir.path().to_path_buf());
        let manager = mihomo_config::ConfigManager::new().unwrap();
        manager.save("default", "rules:\n  - DIRECT").await.unwrap();

        let app = setup_app();
        let response = app.oneshot(
            Request::builder().uri("/admin/api/rules").body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        mihomo_platform::clear_home_dir_override();
    }

    #[tokio::test]
    async fn test_flush_fake_ip_route() {
        let _guard = TEST_LOCK.lock().await;
        let temp_dir = tempfile::tempdir().unwrap();
        mihomo_platform::set_home_dir_override(temp_dir.path().to_path_buf());
        let app = setup_app();
        let response = app.oneshot(
            Request::builder().method("POST").uri("/admin/api/fake-ip/flush").body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        mihomo_platform::clear_home_dir_override();
    }

    #[tokio::test]
    async fn test_list_core_versions_route() {
        let _guard = TEST_LOCK.lock().await;
        let app = setup_app();
        let response = app.oneshot(
            Request::builder().uri("/admin/api/core/versions").body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        mihomo_platform::clear_home_dir_override();
    }
}