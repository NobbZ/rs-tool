#![allow(unused_parens)]

use actix_web::{App, HttpServer};
use clap::{command, ArgAction, Parser};
use std::{io::Result as IoResult, net::IpAddr};
use tracing::{info, Level};
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};

use rs_tool::app;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The IP to bind to
    #[arg(short, long, default_value = "::1")]
    ip: IpAddr,
    /// The port to bind to
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Number of workers (machine dependant default, number of physical cores)
    #[arg(short, long, default_value_t = num_cpus::get_physical())]
    workers: usize,

    /// Verbosity, can be repeated twice
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,

    /// Print only warnings and errors, mutually exclusive with 'verbose'
    #[arg(short, long)]
    quiet: bool,
}

impl Cli {
    // todo: Add *proper* error!
    fn get_log_level(&self) -> Result<Level, ()> {
        match (self.quiet, self.verbose) {
            (true, 0) => Ok(Level::WARN),
            (true, _) => Err(()),
            (_, 0) => Ok(Level::INFO),
            (_, 1) => Ok(Level::DEBUG),
            (_, 2) => Ok(Level::TRACE),
            (_, _) => Err(()),
        }
    }
}

#[actix_web::main]
async fn main() -> IoResult<()> {
    let cli = Cli::parse();
    let level = cli.get_log_level().unwrap();
    let bind = (cli.ip, cli.port);

    FmtSubscriber::builder()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_max_level(level)
        .init();

    let ip_str = if bind.0.is_ipv6() {
        format!("[{}]", bind.0)
    } else {
        format!("{}", bind.0)
    };

    info!("Starting server on {ip}:{port}", ip = ip_str, port = bind.1);

    HttpServer::new(|| app!())
        .bind(bind)?
        .workers(cli.workers)
        .run()
        .await
}
