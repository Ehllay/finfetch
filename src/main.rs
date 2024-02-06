use libmacchina::{
    traits::GeneralReadout as _, traits::KernelReadout as _, traits::MemoryReadout as _,
    GeneralReadout, KernelReadout, MemoryReadout,
};

use std::{fmt, vec};

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

struct Readouts {
    general_readout: GeneralReadout,
    kernel_readout: KernelReadout,
    memory_readout: MemoryReadout,
}

impl fmt::Display for Fetches {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{self:?}"))
    }
}

fn getinfo(info: Fetches, readout: &Readouts) -> String {
    match info {
        Fetches::OS => whoami::distro(),
        Fetches::Host => whoami::devicename(),
        Fetches::DE => readout.general_readout.desktop_environment().unwrap(),
        Fetches::Kernel => readout.kernel_readout.os_release().unwrap(),
        Fetches::Uptime => uptime(readout),
        Fetches::Packages => todo!("Packages is hard"),
        Fetches::Shell => readout
            .general_readout
            .shell(
                libmacchina::traits::ShellFormat::Relative,
                libmacchina::traits::ShellKind::Default,
            )
            .unwrap(),
        Fetches::Resolution => todo!("Resolution"),
        Fetches::Theme => todo!("WM theme"),
        Fetches::Cursor => todo!("Cursor"),
        Fetches::Icons => todo!("Icons"),
        Fetches::Terminal => term(readout),
        Fetches::Font => todo!("Font is hard"),
        Fetches::CPU => readout.general_readout.cpu_model_name().unwrap(),
        Fetches::GPU => joingpus(readout),
        Fetches::Memory => readout.memory_readout.total().unwrap().to_string(),
        _ => panic!("invalid fetch type"),
    }
    .to_string()
}

//Get and format uptime
fn uptime(readout: &Readouts) -> String {
    let time = readout.general_readout.uptime().ok().unwrap();
    let days = if time > 86400 {
        (time / 86400).to_string() + "d"
    } else {
        "".to_string()
    };
    let hours = if time > 3600 {
        (time / 3600).to_string() + "h"
    } else {
        "".to_string()
    };
    let mins = if time > 60 {
        (time / 60).to_string() + "m"
    } else {
        "".to_string()
    };
    let secs = (time % 60).to_string() + "s";

    format!("{days} {hours} {mins} {secs}")
        .trim_start()
        .to_owned()
}

// Get terminal name and strip newline (why libmacchina)
fn term(readout: &Readouts) -> String {
    readout
        .general_readout
        .terminal()
        .unwrap()
        .to_owned()
        .replace("\n", "")
}

fn joingpus(readout: &Readouts) -> String {
    let gpus = readout.general_readout.gpus().unwrap();
    gpus.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

// Fetching host and user names
fn userhost() -> [String; 3] {
    [
        whoami::username(),
        HOSTNAME_SYMBOL.to_string(),
        whoami::hostname(),
    ]
}

fn printfetch(fetches: Vec<Fetches>, color: &str, readout: &Readouts) {
    for i in fetches {
        println!(
            "{}{} {}",
            i.to_string().color(color),
            SEPARATOR,
            getinfo(i, readout)
        )
    }
}

fn main() {
    let readouts = Readouts {
        general_readout: GeneralReadout::new(),
        kernel_readout: KernelReadout::new(),
        memory_readout: MemoryReadout::new(),
    };

    let host = userhost();
    println!(
        "{}{}{}\n{}",
        host[0].color(USER_COLOR),
        host[1].bold(),
        host[2].color(HOST_COLOR),
        "-".repeat(host.join("").chars().count())
    );

    //Fetches
    let fetches = vec![
        Fetches::OS,
        Fetches::Host,
        Fetches::Kernel,
        Fetches::Uptime,
        Fetches::DE,
        Fetches::Shell,
        Fetches::Terminal,
        Fetches::CPU,
        Fetches::GPU,
    ];
    //Print items
    printfetch(fetches, "blue", &readouts);
}
