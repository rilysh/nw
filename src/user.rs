use std::ffi::CStr;

#[repr(C)]
struct UserInfo {
    shell: *mut libc::c_char,
    user: *mut libc::c_char,
    host: *mut libc::c_char,
}

impl UserInfo {
    fn new() -> UserInfo {
        UserInfo {
            shell: "".as_ptr() as *mut libc::c_char,
            user: "".as_ptr() as *mut libc::c_char,
            host: "".as_ptr() as *mut libc::c_char,
        }
    }
}

extern "C" {
    fn __get_userinfo(user_info: *mut UserInfo) -> *mut libc::c_char;
}

pub struct User<'a> {
    pub user: &'a str,
    pub shell: &'a str,
    pub host: String,
}

impl<'a> User<'a> {
    pub fn new() -> User<'a> {
        User {
            user: "",
            shell: "",
            host: "".to_string(),
        }
    }

    pub fn get_user_info(&mut self) {
        unsafe {
            let mut user_info = UserInfo::new();
            __get_userinfo(&mut user_info);

            self.user = CStr::from_ptr(user_info.user).to_str().unwrap();
            self.shell = CStr::from_ptr(user_info.shell).to_str().unwrap();
            let host = CStr::from_ptr(user_info.host);
            self.host = host.to_str().unwrap().to_string();

            libc::free(host.as_ptr() as *mut libc::c_void);
        }
    }
}
