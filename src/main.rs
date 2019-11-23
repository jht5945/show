extern crate argparse;
extern crate rust_util;
extern crate json;
extern crate chrono;

mod cmd;
mod opt;

use std::{
    process::Command,
};

use chrono::prelude::*;
use rust_util::{
    XResult,
    new_box_error,
    util_os::*,
    util_cmd::*,
    util_msg::*,
};
use cmd::*;
use opt::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");

fn print_version() {
    print!(r#"show {} - {}
Copyright (C) 2019 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>

Written by Hatter Jiang
"#, VERSION, &GIT_HASH[0..7]);
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
    run_command(&vec!["netstat", "-nr"], verbose)
}

fn show_network(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&vec!["networksetup", "-listallhardwareports"], verbose)
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

fn show_cal(verbose: bool) -> XResult<()> {
    run_command(&vec!["cal", "-3"], verbose)
}


fn main() -> XResult<()> {
    let options = Options::parse_args_static();
    
    if options.version {
        print_version();
        return Ok(());
    }

    if options.cmd.len() == 0 {
        print_message(MessageType::ERROR, "Use show --help print usage.");
        return Ok(());
    }

    if options.verbose {
        print_message(MessageType::INFO, &format!("Command: {}", &options.cmd));
    }

    let commands = vec![
        CommandInfo {
            name: "ip",
            description: "Show public IP",
            support_os: vec![CommandSupportOS::Linux, CommandSupportOS::MacOS],
            command_fn: show_ip,
        },
        CommandInfo {
            name: "time",
            description: "Show time",
            support_os: vec![CommandSupportOS::Linux, CommandSupportOS::MacOS],
            command_fn: show_time,
        },
        CommandInfo {
            name: "cal",
            description: "Show calendar",
            support_os: vec![CommandSupportOS::Linux, CommandSupportOS::MacOS],
            command_fn: show_cal,
        },
        CommandInfo {
            name: "route",
            description: "Show route",
            support_os: vec![CommandSupportOS::Linux, CommandSupportOS::MacOS],
            command_fn: show_route,
        },
        CommandInfo {
            name: "network",
            description: "Show network",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_network,
        },
        CommandInfo {
            name: "list_java",
            description: "Show java list",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_list_java,
        },
        CommandInfo {
            name: "listen_tcp",
            description: "Show tcp listen",
            support_os: vec![CommandSupportOS::Linux, CommandSupportOS::MacOS],
            command_fn: show_listen_tcp,
        },
        CommandInfo {
            name: "listen_udp",
            description: "Show udp listen",
            support_os: vec![CommandSupportOS::Linux, CommandSupportOS::MacOS],
            command_fn: show_listen_udp,
        },
        CommandInfo {
            name: "install_brew",
            description: "Install brew",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_install_brew,
        },
        CommandInfo {
            name: "install_jenv",
            description: "Install jenv",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_install_jenv,
        },
        CommandInfo {
            name: "install_ports",
            description: "Install ports",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_install_ports,
        },
        CommandInfo {
            name: "install_sdkman",
            description: "Install sdkman",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_install_sdkman,
        },
        CommandInfo {
            name: "install_dart",
            description: "Install dart",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_install_dart,
        },
        CommandInfo {
            name: "wifi_info",
            description: "Show wifi info",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_wifi_info,
        },
        CommandInfo {
            name: "wifi_scan",
            description: "Show wifi scan",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_wifi_scan,
        },
    ];

    let cmd_str = options.cmd.as_str();
    match cmd_str {
        ":::" => {
            for c in commands {
                let mut support_os_str = String::new();
                for i in 0..c.support_os.len() {
                    support_os_str.push_str(match c.support_os[i] {
                        CommandSupportOS::Linux => "Linux",
                        CommandSupportOS::MacOS => "macOS",
                    });
                    if i < c.support_os.len() - 1 {
                        support_os_str.push_str(", ");
                    }
                }
                println!("{} - {}  [{}]", c.name, c.description, &support_os_str);
            }
        },
        other => {
            for c in commands {
                if c.name == cmd_str {
                    (c.command_fn)(options.verbose)?;
                    return Ok(());
                }
            }
            print_message(MessageType::ERROR, &format!("Unknown command: {}", other));
        },
    }
    Ok(())
}
