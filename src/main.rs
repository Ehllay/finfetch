use clap::Parser;
use colored::*;

use libmacchina::{
    traits::BatteryReadout as _, traits::GeneralReadout as _, traits::KernelReadout as _,
    traits::MemoryReadout as _, traits::PackageReadout as _, BatteryReadout, GeneralReadout,
    KernelReadout, MemoryReadout, PackageReadout,
};

use std::io::{self, BufWriter, Write};
use std::time::Instant;
use std::{fmt, vec};

// Default vars
const USER_COLOR: &str = "purple";
const HOST_COLOR: &str = USER_COLOR;
const HOSTNAME_SYMBOL: &str = "@";
const SEPARATOR: &str = ":";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'o', long, help = "Only prints host info")]
    hostonly: bool,
    #[clap(short, long, help = "Only prints fetch info")]
    fetchonly: bool,
    #[clap(long, help = "Do not print colors")]
    stdout: bool,
    //Debug
    #[clap(long, help = "Tells how much time it took to run the fetch")]
    time: bool,
}

#[allow(dead_code, clippy::upper_case_acronyms)]
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
    Battery,
}

struct Readouts {
    general_readout: GeneralReadout,
    package_readout: PackageReadout,
    battery_readout: BatteryReadout,
    kernel_readout: KernelReadout,
    memory_readout: MemoryReadout,
}

impl fmt::Display for Fetches {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

fn getinfo(info: Fetches, readout: &Readouts) -> String {
    match info {
        Fetches::OS => whoami::distro(),
        Fetches::Host => whoami::devicename(),
        Fetches::DE => readout.general_readout.desktop_environment().unwrap(),
        Fetches::Kernel => readout.kernel_readout.os_release().unwrap(),
        Fetches::Uptime => uptime(readout),
        Fetches::Packages => packages(readout),
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
        Fetches::Memory => memory(readout),
        Fetches::Network => todo!("Network"),
        Fetches::Battery => format!("{}%", readout.battery_readout.percentage().unwrap()),
    }
    .to_string()
}

//Get and format uptime
fn uptime(readout: &Readouts) -> String {
    let mut time = readout.general_readout.uptime().ok().unwrap();
    let days = if time > 86400 {
        let d = time / 86400;
        time %= 86400;
        d.to_string() + "d"
    } else {
        String::new()
    };
    let hours = if time > 3600 {
        let h = time / 3600;
        time %= 3600;
        h.to_string() + "h"
    } else {
        String::new()
    };
    let mins = if time > 60 {
        let m = time / 60;
        time %= 60;
        m.to_string() + "m"
    } else {
        String::new()
    };
    let secs = (time % 60).to_string() + "s";

    format!("{days} {hours} {mins} {secs}")
        .trim_start()
        .to_owned()
}

fn packages(readout: &Readouts) -> String {
    let packages = &readout.package_readout.count_pkgs()[0];
    format!("{} ({})", packages.1, packages.0.to_string())
}

// Get terminal name and strip newline (why libmacchina)
fn term(readout: &Readouts) -> String {
    readout
        .general_readout
        .terminal()
        .unwrap()
        .to_owned()
        .replace('\n', "")
}

fn joingpus(readout: &Readouts) -> String {
    let gpus = readout.general_readout.gpus().unwrap();
    gpus.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn memory(readout: &Readouts) -> String {
    let total = readout.memory_readout.total();
    let used = readout.memory_readout.used();

    format!(
        "{} / {}",
        kib_to_appropriate(used.unwrap()),
        kib_to_appropriate(total.unwrap())
    )
}

fn kib_to_appropriate(i: u64) -> String {
    match i {
        i if i >= 1024 => format!("{} MiB", (i / 1024)),
        i if i >= 1048576 => format!("{} GiB", (i / 1048576)),
        _ => i.to_string(),
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

fn printhost(
    handle: &mut BufWriter<io::StdoutLock<'_>>,
    host: [String; 3],
    user_color: &str,
    host_color: &str,
    format: bool,
) {
    if format {
        writeln!(
            handle,
            "{}{}{}\n{}",
            host[0].color(user_color),
            host[1].bold(),
            host[2].color(host_color),
            "-".repeat(host.join("").chars().count())
        )
        .expect("Could not write to buffer");
    } else {
        writeln!(
            handle,
            "{}{}{}\n{}",
            host[0],
            host[1],
            host[2],
            "-".repeat(host.join("").chars().count())
        )
        .expect("Could not write to buffer");
    }
}

fn printfetch(
    mut handle: io::BufWriter<io::StdoutLock<'_>>,
    fetches: Vec<Fetches>,
    color: &str,
    readout: &Readouts,
    format: bool,
) {
    for i in fetches {
        writeln!(
            handle,
            "{}{} {}",
            format
                .then(|| i.to_string().color(color))
                .unwrap_or_else(|| i.to_string().into()),
            SEPARATOR,
            getinfo(i, readout)
        )
        .expect("Could not write to buffer")
    }
}

fn main() {
    let args = Args::parse();

    let readouts = Readouts {
        general_readout: GeneralReadout::new(),
        package_readout: PackageReadout::new(),
        battery_readout: BatteryReadout::new(),
        kernel_readout: KernelReadout::new(),
        memory_readout: MemoryReadout::new(),
    };

    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());
    let _ = handle.flush();

    let host = userhost();

    //Fetches
    let fetches = vec![
        Fetches::OS,
        Fetches::Host,
        Fetches::Kernel,
        Fetches::Packages,
        Fetches::Uptime,
        Fetches::DE,
        Fetches::Shell,
        Fetches::Terminal,
        Fetches::CPU,
        Fetches::GPU,
        Fetches::Memory,
    ];

    let instant: Option<Instant> = if args.time {
        Some(Instant::now())
    } else {
        None
    };

    let format = !args.stdout;

    if args.hostonly {
        printhost(&mut handle, host, USER_COLOR, HOST_COLOR, format);
    } else if args.fetchonly {
        printfetch(handle, fetches, "blue", &readouts, format)
    } else {
        printhost(&mut handle, host, USER_COLOR, HOST_COLOR, format);
        printfetch(handle, fetches, "blue", &readouts, format)
    }

    if args.time {
        println!(
            "Took {:?}ms",
            instant.expect("Couldn't get time").elapsed().as_millis()
        );
    }
}
