use std::{fs, path::Path};

use base::Base;
use clap::{Parser, Subcommand};
use color::{add_color, AnsiColor};
use prettytable::{format, row, Table};

use crate::{cpu::CPU, memory::Memory, pkgs::Pkgs, user::User};

mod base;
mod color;
mod cpu;
mod memory;
mod pkgs;
mod user;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get the type of the BSD system
    Type,

    /// Get the release version of the system
    Rel,

    /// Current logged-in account username
    User,

    /// System hostname
    Host,

    /// Default shell of logged-in account
    Shell,

    /// Total system uptime
    Uptime,

    /// Install packages count
    Pkgs,

    /// Model name of the CPU
    CPU,

    /// Total number of CPU cores
    Cores,

    /// Amounts of memory
    Mem,

    /// Print the FreeBSD ASCII logo
    Freebsd,

    /// Print the OpenBSD ASCII logo
    Openbsd,

    /// Print the NetBSD ASCII logo
    Netbsd,
}

#[derive(Parser, Debug)]
#[command(arg_required_else_help = false)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Commands>,

    #[arg(short, long)]
    extended: bool,
}

fn print_with_logo(bsd_type: &str, all_info: &str) {
    let logo_path = format!("ascii/{}.txt", bsd_type.to_lowercase());
    let ascii_file = fs::read_to_string(logo_path);
    match ascii_file {
        Ok(mut buf) => {
            buf = buf
                .replace("$RED", add_color(AnsiColor::DRed))
                .replace("$YELLOW", add_color(AnsiColor::DYellow))
                .replace("$PURPLE", add_color(AnsiColor::DPurple))
                .replace("$WHITE", add_color(AnsiColor::DWhite));
            let mut table = Table::new();

            table.set_format(*format::consts::FORMAT_CLEAN);
            table.add_row(row![&buf, all_info]);

            let mut vec = Vec::new();
            table.print(&mut vec).unwrap();
            std::str::from_utf8(&vec)
                .unwrap()
                .lines()
                .for_each(|e| println!("{e}"));

            print!("{}", AnsiColor::None);
        }

        Err(why) => eprintln!("error: {why}"),
    }
}

fn add_userhost_info(user: &mut User, out: &mut String) {
    out.push_str(&format!(
        "{}{}{}@{}{}\n",
        add_color(AnsiColor::LRed),
        user.user,
        add_color(AnsiColor::DGreen),
        add_color(AnsiColor::LRed),
        user.host
    ));

    let mut len = user.user.len() + user.host.len() + 1;
    while len > 0 {
        out.push('-');
        len -= 1;
    }
}

fn add_mixing_colors(info: &mut String) {
    let normal_colors = [
        AnsiColor::Red,
        AnsiColor::Green,
        AnsiColor::Yellow,
        AnsiColor::Blue,
        AnsiColor::Purple,
        AnsiColor::Cyan,
        AnsiColor::White,
    ];
    let dark_colors = [
        AnsiColor::DRed,
        AnsiColor::DGreen,
        AnsiColor::DYellow,
        AnsiColor::DBlue,
        AnsiColor::DPurple,
        AnsiColor::DCyan,
        AnsiColor::DWhite,
    ];

    info.push_str(&format!(
        "\n\n{}\u{2588}\u{2588}\u{2588}",
        add_color(AnsiColor::Black)
    ));
    normal_colors.map(|e| info.push_str(&format!("{}\u{2588}\u{2588}\u{2588}", add_color(e))));
    info.push_str(&format!(
        "\n{}\u{2588}\u{2588}\u{2588}",
        add_color(AnsiColor::DBlack)
    ));
    dark_colors.map(|e| info.push_str(&format!("{}\u{2588}\u{2588}\u{2588}", add_color(e))));
}

fn add_cpu_info(info: &mut String) {
    let mut cpu = CPU::new();
    cpu.get_cpu_model();
    cpu.get_num_cpus();
    info.push_str(format!(
        "\n{}CPU:{} {}",
        add_color(AnsiColor::LYellow),
        add_color(AnsiColor::LWhite),
        cpu.brand,
    ).as_str());

    let ncpus_fmt = if cpu.ncpus > 1 {
        format!(
            "\n{}N-CPUs:{} {}",
            add_color(AnsiColor::LYellow),
            add_color(AnsiColor::LWhite),
            cpu.ncpus,
        )
    } else {
        format!(
            "\n{}N-CPU:{} {}",
            add_color(AnsiColor::LYellow),
            add_color(AnsiColor::LWhite),
            cpu.ncpus,
        )
    };

    info.push_str(&ncpus_fmt);
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Some(Commands::Type) => {
            let mut base = Base::new();
            base.get_bsd_type();
            println!("{}", base.bsd_type);
        }

        Some(Commands::Rel) => {
            let mut base = Base::new();
            base.get_bsd_release();
            println!("{}", base.bsd_release);
        }

        Some(Commands::User) => {
            let mut user = User::new();
            user.get_user_info();
            println!("{}", user.user);
        }

        Some(Commands::Host) => {
            let mut user = User::new();
            user.get_user_info();
            println!("{}", user.host);
        }

        Some(Commands::Shell) => {
            let mut user = User::new();
            user.get_user_info();
            println!("{}", user.shell);
        }

        Some(Commands::Uptime) => {
            let mut base = Base::new();
            base.get_sys_uptime(args.extended);
            println!("{}", base.uptime);
        }

        Some(Commands::Pkgs) => {
            let mut pkgs = Pkgs::new();
            pkgs.get_total_numpkgs();
            println!("{}", pkgs.in_total);
        }

        Some(Commands::CPU) => {
            let mut cpu = CPU::new();
            cpu.get_cpu_model();
            println!("{}", cpu.brand);
        }

        Some(Commands::Cores) => {
            let mut cpu = CPU::new();
            cpu.get_num_cpus();
            println!("{}", cpu.ncpus);
        }

        Some(Commands::Mem) => {
            let mut mem = Memory::new();
            mem.get_mem_info();
            if args.extended {
                println!(
                    "Total: {}, Used: {}, Free: {}",
                    mem.total_pretty, mem.used_pretty, mem.free_pretty
                );
            } else {
                println!(
                    "Total: {}, Used: {}, Free: {}",
                    mem.total_exact, mem.used_exact, mem.free_exact
                );
            }
        }

        Some(Commands::Freebsd) => print_with_logo("freebsd", ""),

        Some(Commands::Openbsd) => print_with_logo("openbsd", ""),

        Some(Commands::Netbsd) => print_with_logo("netbsd", ""),

        None => {
            let mut info = String::new();

            // User
            let mut user = User::new();
            user.get_user_info();
            add_userhost_info(&mut user, &mut info);

            // Base
            let mut base = Base::new();
            base.get_bsd_release();
            base.get_bsd_type();
            base.get_sys_uptime(args.extended);
            info.push_str(&format!(
                "\n{}OS:{} {}",
                add_color(AnsiColor::LYellow),
                add_color(AnsiColor::LWhite),
                base.bsd_type
            ));
            info.push_str(&format!(
                "\n{}Release:{} {}",
                add_color(AnsiColor::LYellow),
                add_color(AnsiColor::LWhite),
                base.bsd_release
            ));

            // Pkgs
            let mut pkgs = Pkgs::new();
            pkgs.get_total_numpkgs();
            info.push_str(&format!(
                "\n{}Packages:{} {}",
                add_color(AnsiColor::LYellow),
                add_color(AnsiColor::LWhite),
                pkgs.in_total
            ));

            // (From base)
            info.push_str(&format!(
                "\n{}Uptime:{} {}",
                add_color(AnsiColor::LYellow),
                add_color(AnsiColor::LWhite),
                base.uptime,
            ));

            // (From user)
            let shell_name = Path::new(user.shell).file_name().unwrap();
            info.push_str(&format!(
                "\n{}Shell:{} {}",
                add_color(AnsiColor::LYellow),
                add_color(AnsiColor::LWhite),
                shell_name.to_str().unwrap(),
            ));

            // CPU
            add_cpu_info(&mut info);

            // Memory
            let mut mem = Memory::new();
            mem.get_mem_info();
            info.push_str(&format!(
                "\n{}Memory:{} {}/{} ({})",
                add_color(AnsiColor::LYellow),
                add_color(AnsiColor::LWhite),
                mem.used_pretty,
                mem.free_pretty,
                mem.total_pretty,
            ));

            // Mixed colors
            add_mixing_colors(&mut info);

            // Print everything
            print_with_logo(&base.bsd_type, &info);
        }
    }
}
