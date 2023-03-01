use clap::{Parser, Subcommand, ValueEnum};
use miractl::Mira;
use std::str::FromStr;

// See https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html
// for example documentation

#[derive(Debug, Parser)]
#[command(name = "miractl")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Refresh the display
    Refresh {},

    /// Reset the display to its default settings
    Reset {},

    /// (debugging only)
    #[command(hide = true)]
    Status {},

    /// Set the display mode
    Mode { mode: DisplayMode },

    /// Set display parameters
    Set {
        /// Warm front-light value [0-254]
        #[arg(long = "warm", value_name = "VALUE")]
        warm: Option<u8>,

        /// Cold front-light value [0-254]
        #[arg(long = "cold", value_name = "VALUE")]
        cold: Option<u8>,

        /// Speed [1-7]
        #[arg(long = "speed", value_name = "VALUE")]
        speed: Option<u8>,

        /// Mode
        #[arg(long = "refresh-mode", value_name = "MODE")]
        refresh_mode: Option<RefreshMode>,

        /// Contrast [0-15]
        #[arg(long = "contrast", value_name = "VALUE")]
        contrast: Option<u8>,

        /// Colour filters [WHITE:BLACK] [0-127]
        #[arg(long = "filter", value_name = "VALUE")]
        filter: Option<String>,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum RefreshMode {
    Direct,
    Grey,
    A2,
}

#[derive(Debug, Clone, ValueEnum)]
enum DisplayMode {
    Speed,
    Text,
    Image,
    Video,
    Read,
}

impl From<&RefreshMode> for miractl::RefreshMode {
    fn from(other: &RefreshMode) -> miractl::RefreshMode {
        match other {
            RefreshMode::Direct => miractl::RefreshMode::Direct,
            RefreshMode::Grey => miractl::RefreshMode::Grey,
            RefreshMode::A2 => miractl::RefreshMode::A2,
        }
    }
}

impl From<&DisplayMode> for miractl::DisplayMode {
    fn from(other: &DisplayMode) -> miractl::DisplayMode {
        match other {
            DisplayMode::Speed => miractl::DisplayMode::Speed,
            DisplayMode::Text => miractl::DisplayMode::Text,
            DisplayMode::Image => miractl::DisplayMode::Image,
            DisplayMode::Video => miractl::DisplayMode::Video,
            DisplayMode::Read => miractl::DisplayMode::Read,
        }
    }
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match Mira::new() {
        Ok(mira) => match &cli.command {
            Commands::Refresh {} => {
                mira.refresh().unwrap();
            }
            Commands::Reset {} => {
                mira.reset().unwrap();
            }
            Commands::Status {} => {
                let status = mira.get_status();
                println!("Warm light: {}", status.warm_light);
                println!("Cold light: {}", status.cold_light);
                println!("Speed: {}", status.speed);
            }
            Commands::Mode { mode } => {
                mira.set_display_mode(mode.into())
                    .expect("Failed to set display mode");
            }
            Commands::Set {
                warm,
                cold,
                speed,
                refresh_mode,
                filter,
                contrast,
            } => {
                if let Some(warm) = warm {
                    mira.set_warm_light(*warm)
                        .expect("Failed to set warm light");
                }
                if let Some(cold) = cold {
                    mira.set_cold_light(*cold)
                        .expect("Failed to set cold light");
                }
                if let Some(speed) = speed {
                    mira.set_speed(*speed).expect("Failed to set speed");
                }
                if let Some(mode) = refresh_mode {
                    mira.set_refresh_mode(mode.into()).unwrap();
                }
                if let Some(contrast) = contrast {
                    mira.set_contrast(*contrast)
                        .expect("Failed to set contrast");
                }
                if let Some(filter) = filter {
                    if let Some((white, black)) = parse_pair(filter, ':') {
                        mira.set_colour_filter(white, black)
                            .expect("Failed to set colour filter");
                    } else {
                        eprintln!(
                            "Failed to parse pair; should be `WHITE:BLACK'"
                        )
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}
