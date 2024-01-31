use std::{env, fmt, vec};

use colored::*;

// Default vars
const USER_COLOR: &str = "purple";
const HOST_COLOR: &str = USER_COLOR;
const HOSTNAME_SYMBOL: &str = "@";
const SEPARATOR: &str = ":";

#[allow(dead_code)]
#[derive(Debug)]
enum Fetches {
    OS,
    Host,
    Kernel,
    Uptime,
    Packages,
    Shell,
    Resolution,
    DE,
    Theme,
    Icons,
    Cursor,
    Terminal,
    Font,
    CPU,
    GPU,
    Memory,
    Network,
    BIOS,
}

impl fmt::Display for Fetches {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{self:?}"))
    }
}

fn get_env(envin: &str) -> String {
    let env = env::var(envin);
    match env {
        Ok(e) => e,
        Err(_) => "Unknown".to_string(),
    }
}

fn getinfo(info: Fetches) -> String {
    match info {
        Fetches::OS => whoami::distro().to_string(),
        Fetches::Host => whoami::devicename(),
        Fetches::DE => get_env("XDG_CURRENT_DESKTOP"),
        Fetches::Kernel => todo!("Kernel not done yet"),
        Fetches::Uptime => todo!(),
        _ => panic!("invalid fetch type"),
    }
    .to_string()
}

// Fetching host and user names
fn userhost() -> [String; 3] {
    [
        whoami::username(),
        HOSTNAME_SYMBOL.to_string(),
        whoami::hostname(),
    ]
}

fn printfetch(fetches: Vec<Fetches>, color: &str) {
    for i in fetches {
        println!("{}{} {}", i.to_string().color(color), SEPARATOR, getinfo(i))
    }
}

fn main() {
    let host = userhost();
    println!(
        "{}{}{}\n{}",
        host[0].color(USER_COLOR),
        host[1].bold(),
        host[2].color(HOST_COLOR),
        "-".repeat(host.join("").chars().count())
    );

    //Fetches
    let fetches = vec![Fetches::OS, Fetches::Host, Fetches::DE];
    //Print items
    printfetch(fetches, "blue")
}
