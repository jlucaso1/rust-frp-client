use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use std::process::ExitCode;

use crate::config::Config;
use crate::service::Service;

pub fn define_command_line_options(mut app: Command<'_>) -> Command<'_> {
    app = app
        .arg(
            Arg::new("protocol")
                .short('p')
                .long("protocol")
                .takes_value(true)
                .help("protocol")
                .default_value("tcp"),
        )
        .arg(
            Arg::new("local_port")
                .short('l')
                .long("local-port")
                .required(true)
                .takes_value(true)
                .help("local port"),
        )
        .arg(
            Arg::new("remote_addr")
                .short('r')
                .long("remote-addr")
                .required(true)
                .takes_value(true)
                .help("remote address"),
        )
        .arg(
            Arg::new("remote_port")
                .short('s')
                .long("remote-port")
                .required(true)
                .takes_value(true)
                .help("remote port"),
        )
        .arg(
            Arg::new("token")
                .short('t')
                .long("token")
                .required(false)
                .takes_value(true)
                .help("token")
                .default_value(""),
        );

    app
}

#[tokio::main]
async fn start_service(config: Config) -> Result<()> {
    let mut service = Service::new(config).await?;
    service.run().await?;

    Ok(())
}

#[derive(Debug)]
pub struct FrpcProps {
    pub protocol: String,
    pub local_port: u16,
    pub remote_port: u16,
    pub remote_addr: String,
    pub token: String,
}

impl FrpcProps {
    pub fn new(
        protocol: String,
        local_port: u16,
        remote_port: u16,
        remote_addr: String,
        token: String,
    ) -> Self {
        Self {
            protocol,
            local_port,
            remote_port,
            remote_addr,
            token,
        }
    }
}

pub fn main(matches: &ArgMatches) -> ExitCode {
    let props = FrpcProps::new(
        matches.value_of("protocol").unwrap_or("tcp").to_string(),
        matches.value_of("local_port").unwrap().parse().unwrap(),
        matches.value_of("remote_port").unwrap().parse().unwrap(),
        matches.value_of("remote_addr").unwrap().to_string(),
        matches.value_of("token").unwrap().to_string(),
    );

    let mut client_config = Config::new();
    client_config.load_config(&props).unwrap();
    start_service(client_config).unwrap();

    ExitCode::SUCCESS
}
