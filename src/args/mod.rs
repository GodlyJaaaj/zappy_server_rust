use clap::error::Error;
use clap::error::ErrorKind::ValueValidation;
use clap::Parser;
use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Zappy server made in rust.",
    author = "https://github.com/GodlyJaaaj"
)]
pub struct ServerArgs {
    /// Server Port, must be in range 1-65535
    #[arg(short, value_parser = parse_port, required = true)]
    port: u16,

    /// Map Width
    #[arg(short = 'x', value_parser = parse_width, required = true)]
    width: u8,

    /// Map Height
    #[arg(short = 'y', value_parser = parse_height, required = true)]
    height: u8,

    /// Team Names
    #[arg(short = 'n', num_args = 1.., required = true)]
    teams: Vec<String>,

    /// Number of authorized client per team
    #[arg(short = 'c', value_parser = parse_nb_clients, required = true)]
    clients_nb: u64,

    /// Freq (1s/freq = MAX tps)
    #[arg(short = 'f', value_parser = parse_freq, required = true)]
    freq: u16,
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

    for name in teams {
        if !unique_names.insert(name.clone()) {
            return Err(Error::raw(
                ValueValidation,
                format!("duplicates team name '{}'", name),
            ));
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

pub fn parse_freq(s: &str) -> Result<u16, String> {
    let freq: u16 = s.parse().map_err(|_| "not a valid freq number")?;

    if freq == 0 {
        return Err(String::from("freq must greater than 0"));
    }

    Ok(freq)
}
