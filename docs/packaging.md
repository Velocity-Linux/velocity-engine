# Arch Linux Packaging

This directory contains the official Arch Linux package definition for **Velocity Engine**.

## Files

| File | Purpose |
|------|---------|
| `PKGBUILD` | Build script for `makepkg`. Builds the project from source using `cargo`. |
| `.SRCINFO` | AUR-ready metadata derived from the `PKGBUILD`. |
| `.namcap` | Optional Namcap analyzer configuration. |
| `.packaging/velocity-engine.install` | Pacman install/remove hooks for service teardown. |
| `.packaging/package.conf` | Central packaging metadata for local tooling. |
| `.github/workflows/package.yml` | CI workflow that builds the Arch package on tag or dispatch. |
| `docs/packaging.md` | This file. |

## Building Locally

Install build dependencies:

```bash
sudo pacman -S --needed cargo rust pkg-config dbus libdbus-1-dev
```

Build the package:

```bash
makepkg -s
```

Install the resulting package:

```bash
sudo pacman -U velocity-engine-*.pkg.tar.zst
```

## Package Contents

- `/usr/bin/velocity-engine` — Main daemon binary
- `/usr/bin/velocityctl` — CLI control tool
- `/usr/lib/systemd/system/velocity-engine.service` — Systemd service unit
- `/etc/velocity-engine/default.toml` — Default configuration file

## Runtime Dependencies

- `dbus` — D-Bus IPC
- `power-profiles-daemon` — Power profile integration

## Versioning

The package version is derived from the `version` field in `Cargo.toml`. The `PKGBUILD` reads it dynamically via `pkgver()`.

## CI/CD

The `.github/workflows/package.yml` workflow:

1. Builds the Arch package inside an Ubuntu container with `devtools`
2. Uploads the `.pkg.tar.zst` alongside `.SRCINFO` and `PKGBUILD` to the GitHub Release
