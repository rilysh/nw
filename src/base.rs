use std::ffi::CStr;

extern "C" {
    fn __get_bsd_type() -> *mut libc::c_char;
    fn __get_bsd_release() -> *mut libc::c_char;
    fn __get_sys_uptime() -> libc::c_long;
}

pub struct Base {
    pub bsd_type: String,
    pub bsd_release: String,
    pub uptime: String,
}

impl Base {
    pub fn new() -> Base {
        Base {
            bsd_type: "".to_string(),
            bsd_release: "".to_string(),
            uptime: "".to_string(),
        }
    }

    pub fn get_bsd_type(&mut self) {
        unsafe {
            let rptr = __get_bsd_type();
            self.bsd_type = CStr::from_ptr(rptr).to_str().unwrap().to_string();
            libc::free(rptr as *mut libc::c_void);
        }
    }

    pub fn get_bsd_release(&mut self) {
        unsafe {
            let rptr = __get_bsd_release();
            if rptr.is_null() {
                panic!("error: __get_bsd_release() -> null");
            }
            self.bsd_release = CStr::from_ptr(rptr).to_str().unwrap().to_string();
            libc::free(rptr as *mut libc::c_void);
        }
    }

    pub fn get_sys_uptime(&mut self, expand: bool) {
        let uptime;

        unsafe {
            uptime = __get_sys_uptime();
        }

        let days = uptime / (60 * 60 * 24);
        let hours = (uptime / (60 * 60)) - (days * 24);
        let mins = (uptime / 60) - (days * 60 * 24) - (hours * 60);
        let secs = uptime - (days * 60 * 60 * 24) - (hours * 60 * 60) - (mins * 60);

        if expand {
            if days > 0 {
                self.uptime = format!(
                    "{} days, {} hours, {} mins, {} secs",
                    days, hours, mins, secs
                );
            } else if days == 0 && hours > 0 {
                self.uptime = format!("{} hours, {} mins, {} secs", hours, mins, secs);
            } else if days == 0 && hours == 0 && mins > 0 {
                self.uptime = format!("{} mins, {} secs", mins, secs);
            } else {
                self.uptime = format!("{} secs", secs);
            }
        } else {
            if days > 0 {
                self.uptime = format!("{}d, {}h, {}m, {}s", days, hours, mins, secs);
            } else if days == 0 && hours > 0 {
                self.uptime = format!("{}h, {}m, {}s", hours, mins, secs);
            } else if days == 0 && hours == 0 && mins > 0 {
                self.uptime = format!("{}m, {}s", mins, secs);
            } else {
                self.uptime = format!("{}s", secs);
            }
        }
    }
}
