use std::collections::HashSet;
use clap::error::{Error};
use clap::error::ErrorKind::{MissingRequiredArgument, ValueValidation};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Zappy server made in rust.",
    author = "https://github.com/GodlyJaaaj"
)]
pub struct ServerArgs {
    /// Server Port, must be in range 1-65535
    #[arg(short, value_parser = parse_port)]
    port: u16,

    /// Map Width
    #[arg(short = 'x', value_parser = parse_width)]
    width: u8,

    /// Map Height
    #[arg(short = 'y', value_parser = parse_height)]
    height: u8,

    /// Team Names
    #[arg(short = 'n', long, num_args = 1..)]
    teams: Vec<String>,

    /// Number of authorized client per team
    #[arg(short = 'c', value_parser = parse_nb_clients)]
    clients_nb: u64,

    //Freq
    // TODO: ... -f
}

pub fn parse_server_args() -> Result<ServerArgs, clap::Error> {
    let args = ServerArgs::try_parse()?;
    validate_unique_teams(&args.teams)?;

    Ok(args)
}

pub fn parse_height(s: &str) -> Result<u8, String> {
    let height: u8 = s.parse().map_err(|_| "not a valid height")?;
    if height == 0 {
        Err(String::from("Height must be greater than 0"))
    } else {
        Ok(height)
    }
}

pub fn parse_width(s: &str) -> Result<u8, String> {
    let height: u8 = s.parse().map_err(|_| "not a valid width")?;
    if height == 0 {
        Err(String::from("Height must be greater than 0"))
    } else {
        Ok(height)
    }
}

pub fn parse_port(s: &str) -> Result<u16, String> {
    let port: u16 = s.parse().map_err(|_| "not a valid port")?;
    if port == 0 {
        Err(String::from("Port must be greater than 0."))
    } else {
        Ok(port)
    }
}

/// Validate unique team names
fn validate_unique_teams(teams: &[String]) -> Result<(), clap::Error> {
    let mut unique_names = HashSet::new();

    if teams.is_empty() {
        return Err(Error::raw(MissingRequiredArgument, "Missing team names"));
    }
    for name in teams {
        if !unique_names.insert(name.clone()) {
            return Err(Error::raw(ValueValidation, format!("duplicates team name '{}'", name)));
        }
    }
    Ok(())
}


pub fn parse_nb_clients(s: &str) -> Result<u64, String> {
    let clients_nb: u64 = s.parse().map_err(|_| "not a valid clients number")?;

    if clients_nb == 0 {
        return Err(String::from("Clients number must greater than 0"));
    }

    Ok(clients_nb)
}
