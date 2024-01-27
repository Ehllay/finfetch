use std::env;

use colored::*;

// Vars
const HOSTNAME_SYMBOL: &str = "@";
const SEPARATOR: &str = "->";

fn get_env(envin: &str) -> String {
    let env = env::var(envin);
    match env {
        Ok(e) => e,
        Err(_) => "Unknown".to_string(),
    }
}

/* Fetch List (temporal)
        [
        "OS",
        "Host",
        "Kernel",
        "Uptime",
        "Packages",
        "Shell",
        "Resolution",
        "DE",
        "Theme",
        "Icons",
        "Cursor",
        "Terminal",
        "Font",
        "CPU",
        "GPU",
        "Memory",
        "Network",
        "BIOS",
    ]
*/

fn printinfo(info: &str, color: &str) {
    let fetch: String = match info {
        "OS" => whoami::distro().to_string(),
        "DE" => get_env("XDG_CURRENT_DESKTOP"),
        "Arch" => whoami::arch().to_string(),
        _ => panic!("invalid fetch type"),
    };

    println!("{}{} {}", info.color(color), SEPARATOR, fetch,)
}

// Fetching
fn userhost() -> [String; 3] {
    [
        whoami::username(),
        HOSTNAME_SYMBOL.to_string(),
        whoami::hostname(),
    ]
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

    //Get OS
    printinfo("OS", "blue");
    //Get DE/WM
    printinfo("DE", "blue");
}
