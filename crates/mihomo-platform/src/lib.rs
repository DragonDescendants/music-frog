pub mod android;
pub mod android_bridge;
pub mod desktop;
pub mod paths;
pub mod traits;

pub use android::*;
pub use android_bridge::*;
pub use desktop::*;
pub use paths::*;
pub use traits::*;

pub fn apply_data_dir_override<P: DataDirProvider>(provider: &P) {
    if let Some(path) = provider.data_dir() {
        paths::set_home_dir_override(path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct MockProvider {
        path: PathBuf,
    }

    impl DataDirProvider for MockProvider {
        fn data_dir(&self) -> Option<PathBuf> {
            Some(self.path.clone())
        }
    }

    #[test]
    fn test_apply_data_dir_override() {
        let path = PathBuf::from("/test/path");
        let provider = MockProvider { path: path.clone() };
        apply_data_dir_override(&provider);
        assert_eq!(paths::get_home_dir().unwrap(), path);
        paths::clear_home_dir_override();
    }

    #[test]
    fn test_apply_data_dir_override_none() {
        struct NoneProvider;
        impl DataDirProvider for NoneProvider {
            fn data_dir(&self) -> Option<PathBuf> { None }
        }
        let provider = NoneProvider;
        paths::clear_home_dir_override();
        apply_data_dir_override(&provider);
        // Should not change anything
    }

    #[test]
    fn test_apply_default_data_dir_override() {
        let provider = DesktopDataDirProvider::default();
        apply_data_dir_override(&provider);
    }
}
