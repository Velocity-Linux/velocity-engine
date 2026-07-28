#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;
    use velocity_engine::config::ConfigLoader;
    use velocity_engine::error::Result;

    #[tokio::test]
    async fn test_config_loader_creation() -> Result<()> {
        let toml_content = r#"
            [daemon]
            config_path = "/tmp/test.toml"
            poll_interval_ms = 1000
            restore_timeout_ms = 5000
            dbus_name = "org.velocityos.Engine"

            [profiles.gaming]
            cpu_governor = "performance"

            [profiles.default]
            cpu_governor = "schedutil"

            [[games]]
            name = "Test"
            processes = ["test.exe"]
            profile = "gaming"

            [logging]
            level = "info"
            journald = false
            format = "text"
        "#;

        let file = NamedTempFile::new().expect("Failed to create temp file");
        std::fs::write(file.path(), toml_content).expect("Failed to write config");

        let loader = ConfigLoader::new(file.path()).await?;
        let config = loader.get().await;

        assert_eq!(config.daemon.poll_interval_ms, 1000);
        assert!(config.profiles.contains_key("gaming"));
        assert_eq!(config.games.len(), 1);
        assert_eq!(config.games[0].name, "Test");

        Ok(())
    }

    #[tokio::test]
    async fn test_config_reload() -> Result<()> {
        let toml_content = r#"
            [daemon]
            config_path = "/tmp/test.toml"
            poll_interval_ms = 1000
            restore_timeout_ms = 5000
            dbus_name = "org.velocityos.Engine"

            [profiles.gaming]
            cpu_governor = "performance"

            [profiles.default]
            cpu_governor = "schedutil"

            [[games]]
            name = "Test"
            processes = ["test.exe"]
            profile = "gaming"

            [logging]
            level = "info"
            journald = false
            format = "text"
        "#;

        let file = NamedTempFile::new().expect("Failed to create temp file");
        std::fs::write(file.path(), toml_content).expect("Failed to write config");

        let loader = ConfigLoader::new(file.path()).await?;
        let config = loader.get().await;
        assert_eq!(config.daemon.poll_interval_ms, 1000);

        Ok(())
    }
}
