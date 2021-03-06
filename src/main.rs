extern crate argparse;
#[macro_use]
extern crate rust_util;
extern crate json;
extern crate chrono;

mod cmd;
mod opt;

use std::process::Command;
use chrono::prelude::*;
use rust_util::{
    XResult,
    new_box_error,
    util_os::*,
    util_cmd::*,
};
use cmd::*;
use opt::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");

fn print_version() {
    print!(r#"show {} - {}
Copyright (C) 2019-2020 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>

Written by Hatter Jiang
"#, VERSION, &GIT_HASH[0..7]);
}

fn show_ip(verbose: bool) -> XResult<()> {
    let resp = reqwest::get("https://hatter.ink/ip/ip.jsonp")?.text()?;
    if verbose {
        information!("Received response: {}", resp);
    }
    let ip_json_object = json::parse(&resp)?;
    let ip = &ip_json_object["ip"];
    if ip.is_null() {
        failure!("Get IP failed.");
    } else {
        success!("Your IP address is: {}", ip.to_string());
    }
    Ok(())
}

fn show_time(verbose: bool) -> XResult<()> {
    let resp = reqwest::get("https://hatter.ink/time/time.jsonp")?.text()?;
    if verbose {
        information!("Received response: {}", resp);
    }
    let time_json_object = json::parse(&resp)?;
    let date_time = &time_json_object["datetime"];
    if date_time.is_null() {
        failure!("Get remote time failed.");
    } else {
        success!("Remote time is: {}", date_time.to_string());
    }
    // https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html
    let local: DateTime<Local> = Local::now();
    success!("Local  time is: {}", local.format("%Y/%m/%d %H:%M:%S.%3f %z").to_string());
    Ok(())
}

fn run_command(cmd_args: &[&str], verbose: bool) -> XResult<()> {
    if verbose {
        information!("Run command: {}", cmd_args.join(" "));
    }
    let mut cmd = Command::new(cmd_args[0]);
    cmd_args.iter().skip(1).for_each(|c| {
        cmd.arg(c);
    });
    run_command_and_wait(&mut cmd)?;
    Ok(())
}

fn show_route(verbose: bool) -> XResult<()> {
    run_command(&["netstat", "-nr"], verbose)
}

fn show_network(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&["networksetup", "-listallhardwareports"], verbose)
}

fn show_listen_tcp(verbose: bool) -> XResult<()> {
    if is_linux() {
        run_command(&["netstat", "-ltnp"], verbose)
    } else if is_macos() {
        run_command(&["lsof", "-iTCP", "-sTCP:LISTEN", "-n", "-P"], verbose)
    } else {
        Err(new_box_error("Not linux or macos."))
    }
}

fn show_listen_udp(verbose: bool) -> XResult<()> {
    if is_linux() {
        run_command(&["netstat", "-lunp"], verbose)
    } else if is_macos() {
        run_command(&["lsof", "-iUDP", "-n", "-P"], verbose)
    } else {
        Err(new_box_error("Not linux or macos."))
    }
}

fn show_wifi_info(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&["/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport", "-I"], verbose)
}

fn show_wifi_scan(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&["/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport", "-s"], verbose)
}

fn show_list_java(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&["/usr/libexec/java_home", "-V"], verbose)
}

fn show_install_brew(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&["/usr/bin/ruby", "-e", r#""$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)""#], verbose)
}

fn show_install_jenv(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&["sh", "-c", "curl -L -s get.jenv.io | bash"], verbose)
}

fn show_install_ports(_verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    success!("Please access: https://www.macports.org/install.php");
    Ok(())
}

fn show_install_sdkman(verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    run_command(&["sh", "-c", r#""curl -s "https://get.sdkman.io" | bash""#], verbose)
}

fn show_install_dart(_verbose: bool) -> XResult<()> {
    if ! is_macos() {
        return Err(new_box_error("Only supports macOS."));
    }
    success!("Please run command:\n$ brew tap dart-lang/dart\n$ brew install dart");
    Ok(())
}

fn show_cal(verbose: bool) -> XResult<()> {
    run_command(&["cal", "-3"], verbose)
}


fn main() -> XResult<()> {
    let options = Options::parse_args_static();
    
    if options.version {
        print_version();
        return Ok(());
    }

    if options.cmd.is_empty() {
        failure!("Use show --help print usage.");
        return Ok(());
    }

    if options.verbose {
        information!("Command: {}", &options.cmd);
    }

    let linux_and_macos = vec![CommandSupportOS::Linux, CommandSupportOS::MacOS];

    let commands = vec![
        CommandInfo { name: "ip",
            description: "Show public IP",
            support_os: linux_and_macos.clone(),
            command_fn: show_ip,
        },
        CommandInfo { name: "time",
            description: "Show time",
            support_os: linux_and_macos.clone(),
            command_fn: show_time,
        },
        CommandInfo { name: "cal",
            description: "Show calendar",
            support_os: linux_and_macos.clone(),
            command_fn: show_cal,
        },
        CommandInfo { name: "route",
            description: "Show route",
            support_os: linux_and_macos.clone(),
            command_fn: show_route,
        },
        CommandInfo { name: "network",
            description: "Show network",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_network,
        },
        CommandInfo { name: "list_java",
            description: "Show java list",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_list_java,
        },
        CommandInfo { name: "listen_tcp",
            description: "Show tcp listen",
            support_os: linux_and_macos.clone(),
            command_fn: show_listen_tcp,
        },
        CommandInfo { name: "listen_udp",
            description: "Show udp listen",
            support_os: linux_and_macos,
            command_fn: show_listen_udp,
        },
        CommandInfo { name: "install_brew",
            description: "Install brew",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_install_brew,
        },
        CommandInfo { name: "install_jenv",
            description: "Install jenv",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_install_jenv,
        },
        CommandInfo { name: "install_ports",
            description: "Install ports",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_install_ports,
        },
        CommandInfo { name: "install_sdkman",
            description: "Install sdkman",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_install_sdkman,
        },
        CommandInfo { name: "install_dart",
            description: "Install dart",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_install_dart,
        },
        CommandInfo { name: "wifi_info",
            description: "Show wifi info",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_wifi_info,
        },
        CommandInfo { name: "wifi_scan",
            description: "Show wifi scan",
            support_os: vec![CommandSupportOS::MacOS],
            command_fn: show_wifi_scan,
        },
    ];

    let cmd_str = options.cmd.as_str();
    match cmd_str {
        ":::" => commands.iter().for_each(|c| {
            let support_os_str = c.support_os.iter().map(|o| match o {
                CommandSupportOS::Linux => "Linux",
                CommandSupportOS::MacOS => "macOS",
            }).collect::<Vec<_>>().join(", ");
            println!("{} - {}  [{}]", c.name, c.description, &support_os_str);
        }),
        other => match commands.iter().find(|c| c.name == cmd_str) {
            None => failure!("Unknown command: {}", other),
            Some(c) => (c.command_fn)(options.verbose)?,
        },
    }
    Ok(())
}
