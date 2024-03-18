use clap::Parser;
use colored::*;

use libmacchina::{
    traits::BatteryReadout as _, traits::GeneralReadout as _, traits::KernelReadout as _,
    traits::MemoryReadout as _, traits::PackageReadout as _, BatteryReadout, GeneralReadout,
    KernelReadout, MemoryReadout, PackageReadout,
};

use serde_derive::{Deserialize, Serialize};
use std::io::{self, BufWriter, Write};
use std::time::Instant;
use std::{fmt, fs::read_to_string, str::FromStr};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'o', long, help = "Only prints host info")]
    hostonly: bool,
    #[clap(short, long, help = "Only prints fetch info")]
    fetchonly: bool,
    #[clap(long, help = "Do not print colors")]
    stdout: bool,
    #[clap(long, help = "Don't display architecture")]
    noarch: bool,
    //Debug
    #[clap(long, help = "Tells how much time it took to run the fetch")]
    time: bool,
    #[clap(long, help = "Prints out where the config file is located")]
    config_path: bool,
}

#[derive(Serialize, Deserialize)]
struct Config {
    fetches: Vec<String>,
    prefix: Vec<String>,
    color: String,
    user_color: String,
    fake_user: String,
    host_color: String, // By default use user color
    hostname_symbol: String,
    separator: String,
    separator_color: String,
    alignment: bool,
    display_os_arch: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            fetches: [
                "OS", "Host", "Kernel", "Packages", "Uptime", "DE", "Shell", "Terminal", "CPU",
                "GPU", "Memory",
            ]
            .iter()
            .map(|v| v.to_string())
            .collect(),
            prefix: vec![],
            color: String::from("blue"),
            user_color: String::from("green"),
            fake_user: String::new(),
            host_color: String::from("green"),
            hostname_symbol: String::from("@"),
            separator: String::from(":"),
            separator_color: String::from("white"),
            alignment: true,
            display_os_arch: true,
        }
    }
}

impl Config {
    fn to_fetches(&self) -> Vec<Fetches> {
        self.fetches
            .iter()
            .filter_map(|s| Fetches::from_str(s).ok())
            .collect()
    }
}

#[allow(clippy::upper_case_acronyms)]
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
impl FromStr for Fetches {
    type Err = String;

    fn from_str(fetch: &str) -> Result<Self, Self::Err> {
        match fetch {
            "OS" => Ok(Fetches::OS),
            "Host" => Ok(Fetches::Host),
            "Kernel" => Ok(Fetches::Kernel),
            "Packages" => Ok(Fetches::Packages),
            "Uptime" => Ok(Fetches::Uptime),
            "Shell" => Ok(Fetches::Shell),
            "Resolution" => Ok(Fetches::Resolution),
            "DE" => Ok(Fetches::DE),
            "Theme" => Ok(Fetches::Theme),
            "Icons" => Ok(Fetches::Icons),
            "Cursor" => Ok(Fetches::Cursor),
            "Terminal" => Ok(Fetches::Terminal),
            "Font" => Ok(Fetches::Font),
            "CPU" => Ok(Fetches::CPU),
            "GPU" => Ok(Fetches::GPU),
            "Memory" => Ok(Fetches::Memory),
            "Network" => Ok(Fetches::Network),
            "Battery" => Ok(Fetches::Battery),
            _ => Err(String::from("Unknown fetch type")),
        }
    }
}

impl fmt::Display for Fetches {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

struct Readouts {
    general_readout: GeneralReadout,
    package_readout: PackageReadout,
    battery_readout: BatteryReadout,
    kernel_readout: KernelReadout,
    memory_readout: MemoryReadout,
}

fn getinfo(info: &Fetches, readout: &Readouts, noarch: bool) -> String {
    match info {
        Fetches::OS => distro(noarch),
        Fetches::Host => host(),
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

fn distro(noarch: bool) -> String {
    let arch = if noarch { "" } else { std::env::consts::ARCH };
    format!("{} {}", whoami::distro(), arch)
        .trim_end()
        .to_string()
}

fn host() -> String {
    if cfg!(target_os = "linux") {
        read_to_string("/sys/devices/virtual/dmi/id/product_name")
            .expect("Unknown")
            .replace('\n', "")
    } else {
        // TODO: Other OS support
        whoami::devicename()
    }
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
fn userhost(separator: &String, fake_user: &String) -> [String; 3] {
    let user = if fake_user.is_empty() {
        whoami::username()
    } else {
        fake_user.to_string()
    };

    [user, separator.to_string(), whoami::hostname()]
}

fn printhost(
    handle: &mut BufWriter<io::StdoutLock<'_>>,
    host: [String; 3],
    format: bool,
    config: &Config,
) {
    if format {
        writeln!(
            handle,
            "{}{}{}\n{}",
            host[0].color(&*config.user_color),
            host[1].bold(),
            host[2].color(&*config.host_color),
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
    readout: &Readouts,
    format: bool,
    config: Config,
    noarch: bool,
) {
    for (j, i) in fetches.iter().enumerate() {
        let max_width = fetches
            .iter()
            .map(|f| f.to_string().len())
            .max()
            .unwrap_or(0);

        let label = if format {
            let colored_item = format!("{}", i).color(&*config.color);
            let colored_separator = config.separator.color(&*config.separator_color);
            if config.alignment {
                if !config.prefix.is_empty() {
                    format!(
                        "{}{}{}{}",
                        config.prefix[j].color(&*config.color),
                        colored_item,
                        colored_separator,
                        " ".repeat(max_width - colored_item.chars().count() + 1)
                    )
                } else {
                    format!(
                        "{}{}{}",
                        colored_item,
                        colored_separator,
                        " ".repeat(max_width - colored_item.chars().count() + 1)
                    )
                }
            } else {
                format!("{}{} ", colored_item, colored_separator,)
            }
        } else {
            format!("{}{} ", i, &config.separator)
        };

        writeln!(handle, "{}{}", label, getinfo(i, readout, noarch),)
            .expect("Could not write to buffer")
    }
}

fn main() -> Result<(), confy::ConfyError> {
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

    let conf: Config = match confy::load("finfetch", "config") {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error loading config: {}", error);
            Config::default()
        }
    };

    let fetches: Vec<Fetches> = conf.to_fetches();

    let host = userhost(&conf.hostname_symbol, &conf.fake_user);

    //Fetches
    let instant: Option<Instant> = if args.time {
        Some(Instant::now())
    } else {
        None
    };

    let format = !args.stdout;
    let noarch = args.noarch || !conf.display_os_arch;

    if args.config_path {
        match confy::get_configuration_file_path("finfetch", "config") {
            Ok(path) => println!("{}", path.display()),
            Err(err) => {
                println!("Error retrieving config file path {}", err)
            }
        }
    } else if args.hostonly {
        printhost(&mut handle, host, format, &conf);
    } else if args.fetchonly {
        printfetch(handle, fetches, &readouts, format, conf, noarch)
    } else {
        printhost(&mut handle, host, format, &conf);
        printfetch(handle, fetches, &readouts, format, conf, noarch)
    }

    if args.time {
        println!(
            "Took {:?}ms",
            instant.expect("Couldn't get time").elapsed().as_millis()
        );
    }

    Ok(())
}
