mod args;

use std::process::ExitCode;
//use std::io::{Stdin, stdin};
//use tokio::net::TcpListener;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::args::{parse_server_args};

#[tokio::main]
async fn main() -> ExitCode {
    let server_args = parse_server_args();

    if let Err(err) = server_args {
        eprintln!("Error parsing arguments: {}", err);
        return ExitCode::from(1);
    }

    return ExitCode::from(0);
}
