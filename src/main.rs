use std::{env, fmt, vec};

use colored::*;

// Vars
const HOSTNAME_SYMBOL: &str = "@";
const SEPARATOR: &str = ":";

#[allow(dead_code)]
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
        match self {
            Fetches::OS => write!(f, "OS"),
            Fetches::Host => write!(f, "Host"),
            Fetches::Kernel => write!(f, "Kernel"),
            Fetches::Uptime => write!(f, "Uptime"),
            Fetches::Packages => write!(f, "Packages"),
            Fetches::Shell => write!(f, "Shell"),
            Fetches::Resolution => write!(f, "Resolution"),
            Fetches::DE => write!(f, "DE"),
            Fetches::Theme => write!(f, "Theme"),
            Fetches::Icons => write!(f, "Icons"),
            Fetches::Cursor => write!(f, "Cursor"),
            Fetches::Terminal => write!(f, "Terminal"),
            Fetches::Font => write!(f, "Font"),
            Fetches::CPU => write!(f, "CPU"),
            Fetches::GPU => write!(f, "GPU"),
            Fetches::Memory => write!(f, "Memory"),
            Fetches::Network => write!(f, "Network"),
            Fetches::BIOS => write!(f, "BIOS"),
        }
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
        _ => panic!("invalid fetch type"),
    }
    .to_string()
}

// Fetching
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
        host[0].color("purple"),
        host[1].bold(),
        host[2].color("blue"),
        "-".repeat(host.join("").chars().count())
    );

    //Fetches
    let fetches = vec![Fetches::OS, Fetches::Host, Fetches::DE];
    //Print items
    printfetch(fetches, "blue")
}
