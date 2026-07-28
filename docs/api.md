# D-Bus API

Velocity Engine exposes a D-Bus API for control and monitoring.

## Bus Name

`org.velocityos.Engine`

## Object Path

`/org/velocityos/Engine`

## Interface

`org.velocityos.Engine`

## Methods

### GetStatus()

Returns the current daemon status.

**Returns:** `s` - Status string containing active games and profile

**Example:**
```bash
velocityctl status
```

### GetActiveProfile()

Returns the currently active optimization profile.

**Returns:** `s` - Profile name

### ActivateProfile(profile)

Activates a specific optimization profile.

**Parameters:**
- `profile` (`s`): Profile name

**Returns:** nothing

**Example:**
```bash
velocityctl profile activate gaming
```

### ListProfiles()

Returns a list of available profiles.

**Returns:** `as` - Array of profile names

**Example:**
```bash
velocityctl profile list
```

### ListGames()

Returns a list of configured games.

**Returns:** `as` - Array of game names

**Example:**
```bash
velocityctl games list
```

### ReloadConfiguration()

Reloads configuration from disk.

**Returns:** nothing

**Example:**
```bash
velocityctl reload
```

### GetAppliedOptimizations()

Returns a list of currently active optimizers.

**Returns:** `as` - Array of optimizer names

**Example:**
```bash
velocityctl optimizations
```

### GameStarted(game_name, profile)

Plugin callback when a game starts.

**Parameters:**
- `game_name` (`s`): Game name
- `profile` (`s`): Profile to apply

### GameStopped(game_name)

Plugin callback when a game stops.

**Parameters:**
- `game_name` (`s`): Game name

## Events

### ProfileChanged

Emitted when the active profile changes.

**Signal:** `ProfileChanged(profile_name)`

### GameDetected

Emitted when a game is detected.

**Signal:** `GameDetected(game_name, profile)`

### GameExited

Emitted when a game exits.

**Signal:** `GameExited(game_name)`

## Permissions

The D-Bus API requires root privileges to call most methods. The daemon runs as root but minimizes its attack surface through Linux security hardening.

## Integration

### Velocity Hub

Uses `GetStatus`, `ListGames`, `ListProfiles` for UI display.

### Velocity Control

Uses `ActivateProfile`, `ReloadConfiguration` for user control.

### Velocity Monitor

Uses `GetAppliedOptimizations`, event signals for monitoring.
