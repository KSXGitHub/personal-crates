use clap::{Parser, ValueEnum};
use portpicker::{is_free, is_free_tcp, is_free_udp, Port};
use std::process::ExitCode;

/// Check if a port is free.
#[derive(Parser)]
struct CliArgs {
    /// Type of port to check.
    #[clap(long = "type", short = 't', value_enum, default_value_t = PortType::Both)]
    port_type: PortType,
    /// Port to check.
    port: Port,
}

/// Type of port to check.
#[derive(Clone, ValueEnum)]
enum PortType {
    Tcp,
    Udp,
    Both,
}

fn main() -> ExitCode {
    let CliArgs { port_type, port } = Parser::parse();

    let check = match port_type {
        PortType::Tcp => is_free_tcp,
        PortType::Udp => is_free_udp,
        PortType::Both => is_free,
    };

    if check(port) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
