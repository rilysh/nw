use std::ffi::CStr;

extern "C" {
    fn __get_cpu_model() -> *mut libc::c_char;
    fn __get_num_cpus() -> i32;
}

pub struct CPU<'a> {
    pub brand: &'a str,
    pub ncpus: i32,
}

impl<'a> CPU<'a> {
    pub fn new() -> CPU<'a> {
        CPU {
            brand: "",
            ncpus: 0,
        }
    }

    pub fn get_cpu_model(&mut self) {
        unsafe {
            let cpu_model = __get_cpu_model();
            if cpu_model.is_null() {
                panic!("error: __get_cpu_model()");
            }

            self.brand = CStr::from_ptr(cpu_model).to_str().unwrap();
        }
    }

    pub fn get_num_cpus(&mut self) {
        unsafe {
            let num_cpus = __get_num_cpus();
            if num_cpus < 0 {
                panic!("error: __get_num_cpus()");
            }

            self.ncpus = num_cpus;
        }
    }
}
