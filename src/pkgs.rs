use std::io::BufRead;
use std::process::Command;

pub struct Pkgs {
    pub in_total: String,
}

impl Pkgs {
    pub fn new() -> Pkgs {
        Pkgs {
            in_total: "".to_string(),
        }
    }

    #[cfg(target_os = "freebsd")]
    pub fn get_total_numpkgs(&mut self) {
        let cmd = Command::new("pkg")
            .arg("info")
            .output()
            .expect("error: pkg with argument 'info', failed.");

        self.in_total = cmd.stdout.lines().count().to_string();
        self.in_total.push_str(" (pkg)");
    }

    #[cfg(target_os = "openbsd")]
    pub fn get_total_numpkgs(&mut self) {
        let cmd = Command::new("pkg_info")
            .output()
            .expect("error: pkg_info with no arguments, failed.");

        self.in_total = cmd.stdout.lines().count().to_string();
        self.in_total.push_str(" (pkg_info)");
    }

    #[cfg(target_os = "netbsd")]
    pub fn get_total_numpkgs(&mut self) {
        let cmd = Command::new("pkgin")
            .arg("list")
            .output()
            .expect("error: pkgin with argument 'list', failed.");

        self.in_total = cmd.stdout.lines().count().to_string();
        self.in_total.push_str(" (pkgin)");
    }
}
