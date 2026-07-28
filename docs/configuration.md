# Configuration

Velocity Engine uses TOML for configuration. The default configuration file is located at `/etc/velocity-engine/default.toml`.

## Daemon Settings

```toml
[daemon]
config_path = "/etc/velocity-engine/default.toml"
poll_interval_ms = 1000
restore_timeout_ms = 5000
enable_plugins = true
dbus_name = "org.velocityos.Engine"
```

- `poll_interval_ms`: How often to check for game processes (default: 1000ms)
- `restore_timeout_ms`: Timeout for restoration operations (default: 5000ms)
- `enable_plugins`: Whether to enable the plugin system
- `dbus_name`: D-Bus well-known name

## Profiles

Each profile defines system optimizations to apply.

```toml
[profiles.gaming]
cpu_governor = "performance"
cpu_affinity = [0, 1, 2, 3, 4, 5, 6, 7]
process_priority = -5
io_priority = "best-effort"
scheduler = "normal"
```

### Available Settings

- `cpu_governor`: CPU frequency governor (`performance`, `powersave`, `schedutil`, etc.)
- `cpu_affinity`: List of CPU cores to use
- `process_priority`: Process nice value (-20 to 19)
- `io_priority`: I/O priority class (`realtime`, `best-effort`, `idle`)
- `power_profile`: Power profile (`performance`, `balanced`, `power-saver`)
- `scheduler`: Scheduler policy (`normal`, `batch`)
- `tweaks`: Custom key-value tweaks

## Game Rules

Define games and the profiles to apply when they run.

```toml
[[games]]
name = "Example Game"
processes = ["game.exe", "game_linux"]
profile = "gaming"
custom_affinity = [0, 1, 2, 3]
custom_priority = -10
custom_io_priority = "best-effort"
plugins = ["mangohud"]
```

- `name`: Human-readable game name
- `processes`: Process names to match
- `profile`: Profile to activate
- `custom_affinity`: Override CPU affinity for this game
- `custom_priority`: Override process priority for this game
- `custom_io_priority`: Override I/O priority for this game
- `plugins`: Plugins to activate for this game

## Logging

```toml
[logging]
level = "info"
journald = true
format = "text"
```

- `level`: Log level (`trace`, `debug`, `info`, `warn`, `error`)
- `journald`: Whether to log to systemd journal
- `format`: Log format (`text`, `json`)

## Security

Velocity Engine runs with elevated permissions. To minimize risk:
- Keep configuration files owned by root with mode 0644
- Validate all configuration before applying
- Only allowlist known safe operations
- Log all state changes
