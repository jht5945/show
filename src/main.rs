extern crate argparse;
extern crate rust_util;
extern crate json;

// use std::{
    
// };

use argparse::{ArgumentParser, StoreTrue, Store};
use rust_util::*;

const VERSION: &str = "0.1";

fn print_version() {
    print!(r#"show {}
Copyright (C) 2019 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>

Written by Hatter Jiang
"#, VERSION);
}

fn show_ip() -> XResult<()> {
    let resp = reqwest::get("https://hatter.ink/ip/ip.jsonp")?.text()?;
    let ip_json_object = json::parse(&resp)?;
    let ip = &ip_json_object["ip"];
    if ip.is_null() {
        print_message(MessageType::ERROR, "Get IP failed.");
    } else {
        print_message(MessageType::OK, &format!("Your IP address is: {}", ip_json_object["ip"].to_string()));
    }

    Ok(())
}


fn main() -> XResult<()> {
    let mut version = false;
    let mut cmd = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("show - command line tool.");
        ap.refer(&mut version).add_option(&["-v", "--version"], StoreTrue, "Print version");
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

    match cmd.as_str() {
        "ip" => show_ip()?,
        unknown => print_message(MessageType::ERROR, &format!("Unknown command: {}", unknown)),
    }
    Ok(())
}
