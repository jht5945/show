extern crate argparse;
extern crate rust_util;
extern crate json;
extern crate chrono;

use std::{
    process::Command,
};

use argparse::{ArgumentParser, StoreTrue, Store};
use chrono::prelude::*;
use rust_util::*;

const VERSION: &str = "0.1";

fn print_version() {
    print!(r#"show {}
Copyright (C) 2019 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>

Written by Hatter Jiang
"#, VERSION);
}

fn show_ip(verbose: bool) -> XResult<()> {
    let resp = reqwest::get("https://hatter.ink/ip/ip.jsonp")?.text()?;
    if verbose {
        print_message(MessageType::INFO, &format!("Received response: {}", resp));
    }
    let ip_json_object = json::parse(&resp)?;
    let ip = &ip_json_object["ip"];
    if ip.is_null() {
        print_message(MessageType::ERROR, "Get IP failed.");
    } else {
        print_message(MessageType::OK, &format!("Your IP address is: {}", ip.to_string()));
    }
    Ok(())
}

fn show_time(verbose: bool) -> XResult<()> {
    let resp = reqwest::get("https://hatter.ink/time/time.jsonp")?.text()?;
    if verbose {
        print_message(MessageType::INFO, &format!("Received response: {}", resp));
    }
    let time_json_object = json::parse(&resp)?;
    let date_time = &time_json_object["datetime"];
    if date_time.is_null() {
        print_message(MessageType::ERROR, "Get remote time failed.");
    } else {
        print_message(MessageType::OK, &format!("Remote time is: {}", date_time.to_string()));
    }
    // https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html
    let local: DateTime<Local> = Local::now();
    print_message(MessageType::OK, &format!("Local  time is: {}", local.format("%Y/%m/%d %H:%M:%S.%3f %z").to_string()));
    Ok(())
}

fn run_command(cmd_args: &Vec<&str>, verbose: bool) -> XResult<()> {
    if verbose {
        print_message(MessageType::INFO, &format!("Run command: {}", cmd_args.join(" ")));
    }
    let mut cmd = Command::new(cmd_args[0]);
    for i in 1..cmd_args.len() {
        cmd.arg(cmd_args[i]);
    }
    run_command_and_wait(&mut cmd)?;
    Ok(())
}

fn show_route(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&vec!["netstat", "-nr"], verbose)
}

fn show_network(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&vec!["networksetup", "-listallhardwareports"], verbose)
}

fn show_java(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&vec!["/usr/libexec/java_home", "-V"], verbose)
}

fn show_listen_tcp(verbose: bool) -> XResult<()> {
    if is_linux() {
        return run_command(&vec!["netstat", "-ltnp"], verbose);
    } else if is_macos() {
        return run_command(&vec!["lsof", "-iTCP", "-sTCP:LISTEN", "-n", "-P"], verbose);
    } else {
        return Err(new_box_error("Not linux or macos."))
    }
}

fn show_listen_udp(verbose: bool) -> XResult<()> {
    if is_linux() {
        return run_command(&vec!["netstat", "-lunp"], verbose);
    } else if is_macos() {
        return run_command(&vec!["lsof", "-iUDP", "-n", "-P"], verbose);
    } else {
        return Err(new_box_error("Not linux or macos."));
    }
}

fn show_wifi_info(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&vec!["/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport", "-I"], verbose)
}

fn show_wifi_scan(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&vec!["/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport", "-s"], verbose)
}

fn show_list_java(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&vec!["/usr/libexec/java_home", "-V"], verbose)
}

fn show_install_brew(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&vec!["/usr/bin/ruby", "-e", r#""$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)""#], verbose)
}

fn show_install_jenv(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&vec!["sh", "-c", "curl -L -s get.jenv.io | bash"], verbose)
}

fn show_install_ports(_verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    print_message(MessageType::OK, "Please access: https://www.macports.org/install.php");
    Ok(())
}

fn show_install_sdkman(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&vec!["sh", "-c", r#""curl -s "https://get.sdkman.io" | bash""#], verbose)
}

fn show_install_dart(_verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    print_message(MessageType::OK, "Please run command:\n$ brew tap dart-lang/dart\n$ brew install dart");
    Ok(())
}


fn main() -> XResult<()> {
    let mut version = false;
    let mut verbose = false;
    let mut cmd = String::new();
    {
        // sub command: https://github.com/tailhook/rust-argparse/blob/master/examples/subcommands.rs
        let mut ap = ArgumentParser::new();
        ap.set_description("show - command line tool.");
        ap.refer(&mut version).add_option(&["-v", "--version"], StoreTrue, "Print version");
        ap.refer(&mut verbose).add_option(&["-V", "--verbose"], StoreTrue, "Verbose print");
        ap.refer(&mut cmd).add_argument("CMD", Store, "Command");
        ap.parse_args_or_exit();
    }
    
    if version {
        print_version();
        return Ok(());
    }

    if cmd.len() == 0 {
        print_message(MessageType::ERROR, "Use show --help print usage.");
        return Ok(());
    }

    if verbose {
        print_message(MessageType::INFO, &format!("Command: {}", &cmd));
    }

    match cmd.as_str() {
        "ip" => show_ip(verbose)?,
        "time" => show_time(verbose)?,
        "java" => show_java(verbose)?,
        "route" => show_route(verbose)?,
        "network" => show_network(verbose)?,
        "list_java" => show_list_java(verbose)?,
        "listen_tcp" => show_listen_tcp(verbose)?,
        "listen_udp" => show_listen_udp(verbose)?,
        "install_brew" => show_install_brew(verbose)?,
        "install_jenv" => show_install_jenv(verbose)?,
        "install_ports" => show_install_ports(verbose)?,
        "install_sdkman" => show_install_sdkman(verbose)?,
        "install_dart" => show_install_dart(verbose)?,
        "wifi_info" => show_wifi_info(verbose)?,
        "wifi_scan" => show_wifi_scan(verbose)?,
        unknown => print_message(MessageType::ERROR, &format!("Unknown command: {}", unknown)),
    }
    Ok(())
}
