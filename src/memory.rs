use human_bytes::human_bytes;

#[repr(C)]
struct MemoryInfo {
    total: u64,
    free: u64,
}

extern "C" {
    fn __get_mem_info(mem_info: *mut MemoryInfo) -> i32;
    fn __getpagesize() -> i32;
}

pub struct Memory {
    pub total_exact: u64,
    pub free_exact: u64,
    pub used_exact: u64,
    pub total_pretty: String,
    pub free_pretty: String,
    pub used_pretty: String,
}

impl MemoryInfo {
    fn new() -> MemoryInfo {
        MemoryInfo { total: 0, free: 0 }
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            total_exact: 0,
            free_exact: 0,
            used_exact: 0,
            total_pretty: "".to_string(),
            free_pretty: "".to_string(),
            used_pretty: "".to_string(),
        }
    }

    // TODO: Fix NetBSD memory info, add NetBSD ascii logo
    pub fn get_mem_info(&mut self) {
        unsafe {
            let mut mi = MemoryInfo::new();
            if __get_mem_info(&mut mi) == -1 {
                panic!("error: __get_mem_info()");
            }

            let page_size = __getpagesize();
            self.total_exact = mi.total * (page_size as u64);
            self.free_exact = mi.free * (page_size as u64);
            self.used_exact = self.total_exact - self.free_exact;

            self.total_pretty = human_bytes(self.total_exact as f64);
            self.free_pretty = human_bytes(self.free_exact as f64);
            self.used_pretty = human_bytes(self.used_exact as f64);
        }
    }
}
