# Velocity Engine

Velocity Engine is the core optimization daemon for **Velocity OS**. It automatically detects when games launch, applies optimized performance profiles, and restores previous settings when games exit.

## Features

- Automatic game detection (Steam, Proton/Wine, native Linux games)
- Configurable optimization profiles (default, gaming, competitive, battery)
- CPU governor, priority, and affinity management
- Power profile integration with power-profiles-daemon
- I/O priority management
- D-Bus API for external tools
- Full restoration on game exit
- Plugin system architecture
- Zero telemetry
- Open source (GPLv3)

## Building

```bash
cargo build --release
```

## Installation

```bash
sudo cp target/release/velocity-engine /usr/bin/
sudo cp target/release/velocityctl /usr/bin/
sudo cp config/default.toml /etc/velocity-engine/
sudo cp systemd/velocity-engine.service /usr/lib/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now velocity-engine
```

## Usage

```bash
# Check status
velocityctl status

# List profiles
velocityctl profile list

# Activate a profile
velocityctl profile activate gaming

# List configured games
velocityctl games list

# Reload configuration
velocityctl reload
```

## Configuration

Configuration is stored in TOML format at `/etc/velocity-engine/default.toml`.

See [docs/configuration.md](configuration.md) for details.

## D-Bus API

See [docs/api.md](api.md) for the complete D-Bus API reference.

## License

GPLv3 - see [LICENSE](LICENSE)
