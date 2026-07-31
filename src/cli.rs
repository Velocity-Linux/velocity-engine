use velocity_engine::error::Result;
use clap::{Parser, Subcommand};
use zbus::Connection;

#[derive(Parser)]
#[command(name = "velocityctl")]
#[command(about = "Velocity Engine control CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
    Games {
        #[command(subcommand)]
        action: GamesAction,
    },
    Reload,
    Optimizations,
}

#[derive(Subcommand)]
enum ProfileAction {
    List,
    Activate { name: String },
}

#[derive(Subcommand)]
enum GamesAction {
    List,
}

async fn get_dbus_connection() -> Result<Connection> {
    Connection::system()
        .await
        .map_err(|e| velocity_engine::error::EngineError::DBus(format!("Cannot connect to D-Bus: {}", e)))
}

async fn handle_status(conn: &Connection) -> Result<()> {
    let proxy = zbus::Proxy::new(
        conn,
        "org.velocityos.Engine",
        "/org/velocityos/Engine",
        "org.velocityos.Engine",
    )
    .await?;

    let status: String = proxy.call("GetStatus", &()).await?;
    println!("Status: {}", status);
    Ok(())
}

async fn handle_profile_list(conn: &Connection) -> Result<()> {
    let proxy = zbus::Proxy::new(
        conn,
        "org.velocityos.Engine",
        "/org/velocityos/Engine",
        "org.velocityos.Engine",
    )
    .await?;

    let profiles: Vec<String> = proxy.call("ListProfiles", &()).await?;
    println!("Available profiles:");
    for profile in profiles {
        println!("  - {}", profile);
    }
    Ok(())
}

async fn handle_profile_activate(conn: &Connection, name: &str) -> Result<()> {
    let proxy = zbus::Proxy::new(
        conn,
        "org.velocityos.Engine",
        "/org/velocityos/Engine",
        "org.velocityos.Engine",
    )
    .await?;

    proxy.call("ActivateProfile", &(name,)).await?;
    println!("Profile '{}' activated", name);
    Ok(())
}

async fn handle_games_list(conn: &Connection) -> Result<()> {
    let proxy = zbus::Proxy::new(
        conn,
        "org.velocityos.Engine",
        "/org/velocityos/Engine",
        "org.velocityos.Engine",
    )
    .await?;

    let games: Vec<String> = proxy.call("ListGames", &()).await?;
    println!("Configured games:");
    for game in games {
        println!("  - {}", game);
    }
    Ok(())
}

async fn handle_reload(conn: &Connection) -> Result<()> {
    let proxy = zbus::Proxy::new(
        conn,
        "org.velocityos.Engine",
        "/org/velocityos/Engine",
        "org.velocityos.Engine",
    )
    .await?;

    proxy.call("ReloadConfiguration", &()).await?;
    println!("Configuration reloaded");
    Ok(())
}

async fn handle_optimizations(conn: &Connection) -> Result<()> {
    let proxy = zbus::Proxy::new(
        conn,
        "org.velocityos.Engine",
        "/org/velocityos/Engine",
        "org.velocityos.Engine",
    )
    .await?;

    let opts: Vec<String> = proxy.call("GetAppliedOptimizations", &()).await?;
    println!("Active optimizers:");
    for opt in opts {
        println!("  - {}", opt);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Status => {
            let conn = get_dbus_connection().await;
            match conn {
                Ok(conn) => handle_status(&conn).await,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Profile { action } => {
            let conn = get_dbus_connection().await;
            match conn {
                Ok(conn) => match action {
                    ProfileAction::List => handle_profile_list(&conn).await,
                    ProfileAction::Activate { name } => handle_profile_activate(&conn, &name).await,
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Games { action } => {
            let conn = get_dbus_connection().await;
            match conn {
                Ok(conn) => match action {
                    GamesAction::List => handle_games_list(&conn).await,
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Reload => {
            let conn = get_dbus_connection().await;
            match conn {
                Ok(conn) => handle_reload(&conn).await,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Optimizations => {
            let conn = get_dbus_connection().await;
            match conn {
                Ok(conn) => handle_optimizations(&conn).await,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
