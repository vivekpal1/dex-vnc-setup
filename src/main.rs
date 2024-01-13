mod vnc_control;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("VNC Server Control")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Manages VNC server for Linux on Dex")
        .arg(
            Arg::new("start")
                .short('s')
                .long("start")
                .action(clap::ArgAction::SetTrue)
                .help("Starts the VNC server"),
        )
        .arg(
            Arg::new("stop")
                .short('p')
                .long("stop")
                .action(clap::ArgAction::SetTrue)
                .help("Stops the VNC server"),
        )
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .action(clap::ArgAction::SetTrue)
                .help("Checks the status of the VNC server"),
        )
        .arg(
            Arg::new("geometry")
                .short('g')
                .long("geometry")
                .value_parser(clap::value_parser!(String))
                .help("Sets the screen geometry (e.g., 1920x1080)"),
        )
        .arg(
            Arg::new("dpi")
                .short('d')
                .long("dpi")
                .value_parser(clap::value_parser!(String))
                .help(
                    "Sets the screen DPI (e.g.,
                150)",
                ),
        )
        .arg(
            Arg::new("server-type")
                .short('t')
                .long("type")
                .value_parser(clap::value_parser!(String))
                .help("Sets the type of VNC server (e.g., tightvnc, tigervnc)"),
        )
        .get_matches();
    if *matches.get_one::<bool>("start").unwrap_or(&false) {
        let default_geometry = "default_geometry".to_string();
        let default_dpi = "default_dpi".to_string();
        let default_server_type = "tightvnc".to_string();

        let geometry = matches
            .get_one::<String>("geometry")
            .unwrap_or(&default_geometry);
        let dpi = matches
            .get_one::<String>("dpi")
            .unwrap_or(&default_dpi);
        let server_type = matches
            .get_one::<String>("server-type")
            .unwrap_or(&default_server_type);

        vnc_control::start_vnc(geometry, dpi, server_type);
    } else if *matches.get_one::<bool>("stop").unwrap_or(&false) {
        vnc_control::stop_vnc();
    } else if *matches.get_one::<bool>("check").unwrap_or(&false) {
        vnc_control::check_status();
    } else {
        println!("No valid command specified. Use --help for more information.");
    }
}