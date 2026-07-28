#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_config_validation() {
        let toml_content = r#"
            [daemon]
            poll_interval_ms = 1000
            restore_timeout_ms = 5000
            dbus_name = "org.velocityos.Engine"

            [profiles.gaming]
            cpu_governor = "performance"
            process_priority = -5
            io_priority = "best-effort"

            [profiles.default]
            cpu_governor = "schedutil"

            [[games]]
            name = "Test Game"
            processes = ["test.exe"]
            profile = "gaming"

            [logging]
            level = "info"
            journald = false
            format = "text"
        "#;

        let config: Config = toml::from_str(toml_content).expect("Failed to parse config");
        assert!(config.profiles.contains_key("gaming"));
        assert!(config.profiles.contains_key("default"));
        assert_eq!(config.games.len(), 1);
        assert_eq!(config.games[0].name, "Test Game");
    }

    #[test]
    fn test_profile_manager_get_profile() {
        let mut profiles = HashMap::new();
        profiles.insert("gaming".to_string(), ProfileConfig::default());

        let manager = ProfileManager::new(profiles, vec![]);
        assert!(manager.get_profile("gaming").is_some());
        assert!(manager.get_profile("nonexistent").is_none());
    }

    #[test]
    fn test_profile_manager_find_profile_for_process() {
        let mut profiles = HashMap::new();
        profiles.insert("gaming".to_string(), ProfileConfig::default());

        let games = vec![GameRule {
            name: "Test".to_string(),
            processes: vec!["test.exe".to_string()],
            profile: "gaming".to_string(),
            custom_affinity: None,
            custom_priority: None,
            custom_io_priority: None,
            plugins: vec![],
        }];

        let manager = ProfileManager::new(profiles, games);
        assert!(manager.find_profile_for_process("test.exe").is_some());
        assert!(manager.find_profile_for_process("other.exe").is_none());
    }

    #[test]
    fn test_game_detector_active_tracking() {
        // This is a simple unit test - in real implementation
        // we would mock process detection
        let games = vec![GameRule {
            name: "Test".to_string(),
            processes: vec!["nonexistent.exe".to_string()],
            profile: "gaming".to_string(),
            custom_affinity: None,
            custom_priority: None,
            custom_io_priority: None,
            plugins: vec![],
        }];

        let detector = GameDetector::new(games);
        // No game is running, so detected list should be empty
        let detected = futures::executor::block_on(detector.check_games()).unwrap();
        assert!(detected.is_empty());
    }

    #[test]
    fn test_system_utils_parse_cpu_list() {
        // Access through the module
        let result = SystemUtils::parse_cpu_list("0-3,5,7-8");
        assert_eq!(result, vec![0, 1, 2, 3, 5, 7, 8]);
    }
}
