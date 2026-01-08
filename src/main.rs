//! SteelSeries GG for Linux - CLI
//!
//! A complete open-source replacement for SteelSeries GG on Linux.

use clap::{Parser, Subcommand};
use std::time::Duration;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use steelseries_gg::config::Config;
use steelseries_gg::devices::{discovery::print_device_summary, DeviceManager, DeviceType};
use steelseries_gg::gamesense::GameSenseServer;
use steelseries_gg::profiles::{Profile, ProfileManager};
use steelseries_gg::rgb::{Color, Effect, WaveDirection};

#[cfg(feature = "audio")]
use steelseries_gg::audio::{AudioMixer, Channel};

/// SteelSeries GG for Linux - Control your SteelSeries devices
#[derive(Parser)]
#[command(name = "ssgg")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List connected SteelSeries devices
    Devices,

    /// Control RGB lighting
    Rgb {
        #[command(subcommand)]
        action: RgbAction,
    },

    /// Manage profiles
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },

    /// Control audio mixer (Sonar replacement)
    #[cfg(feature = "audio")]
    Audio {
        #[command(subcommand)]
        action: AudioAction,
    },

    /// Start the GameSense server
    Server {
        /// Port to listen on
        #[arg(short, long, default_value = "27301")]
        port: u16,
    },

    /// Run as a daemon (device control + GameSense server)
    Daemon,
}

#[derive(Subcommand)]
enum RgbAction {
    /// Set a static color
    Color {
        /// Color as hex (e.g., FF0000) or name (red, green, blue, etc.)
        color: String,
    },

    /// Set brightness (0-100)
    Brightness {
        /// Brightness level
        level: u8,
    },

    /// Set a lighting effect
    Effect {
        /// Effect name: static, breathing, spectrum, wave, off
        name: String,

        /// Effect speed (0.1 - 5.0)
        #[arg(short, long, default_value = "1.0")]
        speed: f32,
    },

    /// Turn off all LEDs
    Off,
}

#[derive(Subcommand)]
enum ProfileAction {
    /// List all profiles
    List,

    /// Load a profile
    Load {
        /// Profile name
        name: String,
    },

    /// Save current settings as a profile
    Save {
        /// Profile name
        name: String,
    },

    /// Delete a profile
    Delete {
        /// Profile name
        name: String,
    },
}

#[cfg(feature = "audio")]
#[derive(Subcommand)]
enum AudioAction {
    /// Show current mixer state
    Status,

    /// Set channel volume
    Volume {
        /// Channel: master, game, chat, media, aux, mic
        channel: String,

        /// Volume level (0-100)
        level: u8,
    },

    /// Mute/unmute a channel
    Mute {
        /// Channel: master, game, chat, media, aux, mic
        channel: String,

        /// Mute state (true/false), omit to toggle
        #[arg(short, long)]
        state: Option<bool>,
    },

    /// Set chat mix balance
    ChatMix {
        /// Balance (-100 = game, 0 = balanced, 100 = chat)
        balance: i8,
    },
}

fn parse_color(s: &str) -> Option<Color> {
    // Try named colors
    match s.to_lowercase().as_str() {
        "red" => return Some(Color::RED),
        "green" => return Some(Color::GREEN),
        "blue" => return Some(Color::BLUE),
        "white" => return Some(Color::WHITE),
        "black" | "off" => return Some(Color::BLACK),
        "cyan" => return Some(Color::CYAN),
        "magenta" => return Some(Color::MAGENTA),
        "yellow" => return Some(Color::YELLOW),
        "orange" => return Some(Color::ORANGE),
        "purple" => return Some(Color::PURPLE),
        "pink" => return Some(Color::PINK),
        _ => {}
    }

    // Try hex
    let hex = s.trim_start_matches('#');
    if hex.len() == 6 {
        if let Ok(val) = u32::from_str_radix(hex, 16) {
            return Some(Color::from_hex(val));
        }
    }

    None
}

#[cfg(feature = "audio")]
fn parse_channel(s: &str) -> Option<Channel> {
    match s.to_lowercase().as_str() {
        "master" => Some(Channel::Master),
        "game" => Some(Channel::Game),
        "chat" => Some(Channel::Chat),
        "media" => Some(Channel::Media),
        "aux" => Some(Channel::Aux),
        "mic" => Some(Channel::Mic),
        _ => None,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let level = if cli.debug { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Create DeviceManager once for all commands that need it
    let needs_device_manager = matches!(
        cli.command,
        Commands::Devices | Commands::Rgb { .. } | Commands::Daemon
    );

    let manager = if needs_device_manager {
        Some(DeviceManager::new()?)
    } else {
        None
    };

    match cli.command {
        Commands::Devices => {
            cmd_devices(manager.as_ref().unwrap())?;
        }

        Commands::Rgb { action } => {
            cmd_rgb(manager.as_ref().unwrap(), action)?;
        }

        Commands::Profile { action } => {
            cmd_profile(action)?;
        }

        #[cfg(feature = "audio")]
        Commands::Audio { action } => {
            cmd_audio(action)?;
        }

        Commands::Server { port } => {
            cmd_server(port).await?;
        }

        Commands::Daemon => {
            cmd_daemon(manager.unwrap()).await?;
        }
    }

    Ok(())
}

fn cmd_devices(manager: &DeviceManager) -> anyhow::Result<()> {
    print_device_summary(manager);
    Ok(())
}

fn cmd_rgb(manager: &DeviceManager, action: RgbAction) -> anyhow::Result<()> {
    // Find the first keyboard
    let keyboard_info = manager
        .first_device_of_type(DeviceType::Keyboard)
        .ok_or_else(|| anyhow::anyhow!("No keyboard found"))?;

    println!("Using keyboard: {}", keyboard_info.name);

    // Open the device
    let mut device = manager.open_device(keyboard_info)?;

    match action {
        RgbAction::Color { color } => {
            let color =
                parse_color(&color).ok_or_else(|| anyhow::anyhow!("Invalid color: {}", color))?;

            println!("Setting color to {}", color);

            // Build the color packet (9 zones)
            let mut data = vec![0x21, 0xFF];
            for _ in 0..9 {
                data.push(color.r);
                data.push(color.g);
                data.push(color.b);
            }

            // Pad to 64 bytes
            let mut report = vec![0u8; 65];
            report[1..1 + data.len()].copy_from_slice(&data);
            device.write(&report)?;

            println!("Done!");
        }

        RgbAction::Brightness { level } => {
            let level = level.min(100);
            println!("Setting brightness to {}%", level);

            let mut report = vec![0u8; 65];
            report[1] = 0x22;
            report[2] = level;
            device.write(&report)?;

            println!("Done!");
        }

        RgbAction::Effect { name, speed } => {
            let effect = match name.to_lowercase().as_str() {
                "breathing" => Effect::Breathing {
                    color: Color::PURPLE,
                    speed,
                },
                "spectrum" => Effect::Spectrum { speed },
                "wave" => Effect::Wave {
                    colors: vec![
                        Color::RED,
                        Color::ORANGE,
                        Color::YELLOW,
                        Color::GREEN,
                        Color::CYAN,
                        Color::BLUE,
                        Color::PURPLE,
                    ],
                    speed,
                    direction: WaveDirection::LeftToRight,
                },
                "off" => Effect::Off,
                _ => Effect::Static {
                    color: Color::WHITE,
                },
            };

            println!("Setting effect: {:?}", effect);
            // Note: Full effect implementation would require a background loop
            println!("(Note: Animated effects require running as daemon)");
        }

        RgbAction::Off => {
            println!("Turning off LEDs");

            let mut report = vec![0u8; 65];
            report[1] = 0x21;
            report[2] = 0xFF;
            // All zeros for black
            device.write(&report)?;

            println!("Done!");
        }
    }

    Ok(())
}

fn cmd_profile(action: ProfileAction) -> anyhow::Result<()> {
    let mut manager = ProfileManager::new()?;

    match action {
        ProfileAction::List => {
            let profiles = manager.list();
            if profiles.is_empty() {
                println!("No profiles found.");
            } else {
                println!("Profiles:");
                for name in profiles {
                    println!("  - {}", name);
                }
            }
        }

        ProfileAction::Load { name } => {
            if let Some(profile) = manager.get(&name) {
                println!("Loading profile: {}", profile.name);
                // TODO: Apply profile settings to devices
                println!("Profile loaded!");
            } else {
                println!("Profile not found: {}", name);
            }
        }

        ProfileAction::Save { name } => {
            let profile = Profile::new(name.clone());
            // TODO: Capture current device settings
            manager.set(profile)?;
            println!("Profile saved: {}", name);
        }

        ProfileAction::Delete { name } => {
            manager.delete(&name)?;
            println!("Profile deleted: {}", name);
        }
    }

    Ok(())
}

#[cfg(feature = "audio")]
fn cmd_audio(action: AudioAction) -> anyhow::Result<()> {
    let mut mixer = AudioMixer::new()?;

    match action {
        AudioAction::Status => {
            println!("Audio Mixer Status:");
            println!();
            for (channel, state) in mixer.all_channels() {
                let mute_str = if state.muted { " (muted)" } else { "" };
                println!(
                    "  {:8} {:3}%{}",
                    channel.to_string(),
                    (state.volume * 100.0) as u8,
                    mute_str
                );
            }
            println!();
            println!("  Chat Mix: {:+.0}%", mixer.chat_mix() * 100.0);
        }

        AudioAction::Volume { channel, level } => {
            let ch = parse_channel(&channel)
                .ok_or_else(|| anyhow::anyhow!("Invalid channel: {}", channel))?;

            let volume = (level.min(100) as f32) / 100.0;
            mixer.set_volume(ch, volume)?;
            println!("{} volume set to {}%", ch, level.min(100));
        }

        AudioAction::Mute { channel, state } => {
            let ch = parse_channel(&channel)
                .ok_or_else(|| anyhow::anyhow!("Invalid channel: {}", channel))?;

            let muted = match state {
                Some(s) => {
                    mixer.set_mute(ch, s)?;
                    s
                }
                None => mixer.toggle_mute(ch)?,
            };

            println!("{} {}", ch, if muted { "muted" } else { "unmuted" });
        }

        AudioAction::ChatMix { balance } => {
            let balance = (balance.clamp(-100, 100) as f32) / 100.0;
            mixer.set_chat_mix(balance)?;
            println!("Chat mix set to {:+.0}%", balance * 100.0);
        }
    }

    Ok(())
}

async fn cmd_server(port: u16) -> anyhow::Result<()> {
    info!("Starting GameSense server on port {}", port);

    let server = GameSenseServer::new("127.0.0.1", port);
    server.run().await?;

    Ok(())
}

async fn cmd_daemon(manager: DeviceManager) -> anyhow::Result<()> {
    info!("Starting SteelSeries GG daemon");

    let config = Config::load()?;

    // Start GameSense server in background
    let gs_port = config.gamesense.port;
    tokio::spawn(async move {
        let server = GameSenseServer::new("127.0.0.1", gs_port);
        if let Err(e) = server.run().await {
            tracing::error!("GameSense server error: {}", e);
        }
    });

    info!("GameSense server started on port {}", config.gamesense.port);

    // Use provided device manager
    print_device_summary(&manager);

    // Load default profile if configured
    if let Some(ref profile_name) = config.default_profile {
        if let Ok(profile_manager) = ProfileManager::new() {
            if let Some(profile) = profile_manager.get(profile_name) {
                info!("Loading default profile: {}", profile.name);
                // TODO: Apply profile
            }
        }
    }

    info!("Daemon running. Press Ctrl+C to stop.");

    // Keep running
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
