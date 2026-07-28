# Architecture

Velocity Engine is designed around a modular, event-driven architecture optimized for low overhead and high reliability.

## Components

```
┌─────────────────────────────────────────────────────────────┐
│                      Daemon Service                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │ ConfigLoader │  │ ProfileMgr   │  │ GameDetector     │  │
│  └──────┬───────┘  └──────┬───────┘  └────────┬─────────┘  │
│         │                 │                    │            │
│         ▼                 ▼                    ▼            │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                  DaemonEngine                        │  │
│  │  - Manages state                                      │  │
│  │  - Coordinates detection and optimization             │  │
│  │  - Handles restoration                                 │  │
│  └─────────────────────┬────────────────────────────────┘  │
│                        │                                    │
│         ┌──────────────┼──────────────┐                    │
│         ▼              ▼              ▼                     │
│  ┌──────────────┐ ┌──────────┐ ┌──────────────────────┐   │
│  │ OptimizerMgr │ │ D-Bus API│ │ Plugin System        │   │
│  │              │ │          │ │                      │   │
│  │ - CPU Opt    │ │ - RPC    │ │ - Steam              │   │
│  │ - Power Opt  │ │ - Events │ │ - Gamescope          │   │
│  │ - IO Opt     │ │ - Status │ │ - MangoHud           │   │
│  │ - Scheduler  │ │          │ │ - OBS                │   │
│  └──────────────┘ └──────────┘ └──────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Data Flow

1. **Detection Phase**: GameDetector scans `/proc` for configured game processes
2. **Decision Phase**: ProfileManager maps detected games to optimization profiles
3. **Application Phase**: OptimizerManager applies profiles via individual optimizers
4. **Monitoring Phase**: Active games are tracked in a thread-safe set
5. **Restoration Phase**: On game exit, all optimizers restore previous state

## Plugin System

Plugins communicate with the daemon through a defined API. They:
- Are isolated from the core daemon
- Communicate through D-Bus or a defined IPC mechanism
- Have clear permission scopes
- Can trigger game detection and optimization

## Threading Model

- Main event loop runs on a single tokio task
- D-Bus API runs on the async runtime
- All state access uses `tokio::sync::RwLock` for safety
- No background polling loops - detection is event-driven where possible

## Error Handling

All errors use `thiserror` for structured error types. No `unwrap()` in production paths. Errors are logged and propagated appropriately.
