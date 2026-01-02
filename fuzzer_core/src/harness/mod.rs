use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};

const CURLOPT_URL: c_uint = 10002;
const CURLOPT_TIMEOUT_MS: c_uint = 155;

#[allow(non_camel_case_types)]
type CURL = c_void;

#[allow(non_camel_case_types)]
type CURLcode = c_int;

#[allow(non_camel_case_types)]
type CURLoption = c_uint;

extern "C" {
    fn curl_easy_init() -> *mut CURL;
    fn curl_easy_setopt(handle: *mut CURL, option: CURLoption, ...) -> CURLcode;
    fn curl_easy_perform(handle: *mut CURL) -> CURLcode;
    fn curl_easy_cleanup(handle: *mut CURL);
}

pub fn init_target() {}

pub fn fuzz_iteration(input: &[u8]) -> bool {
    // NO catch_unwind - let ASan catch crashes and terminate the process
    // Return true if execution completed normally, false if we should save this input
    unsafe {
        let url = match CString::new(input) {
            Ok(value) => value,
            Err(_) => return true, // Invalid input, not interesting
        };

        let handle = curl_easy_init();
        if handle.is_null() {
            return true;
        }

        let _ = curl_easy_setopt(handle, CURLOPT_URL, url.as_ptr() as *const c_char);
        let _ = curl_easy_setopt(handle, CURLOPT_TIMEOUT_MS, 50 as c_long);
        let _ = curl_easy_perform(handle);

        curl_easy_cleanup(handle);
    }
    true // Execution completed normally
}
