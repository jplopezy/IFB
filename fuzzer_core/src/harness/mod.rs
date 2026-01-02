use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;

const CURLOPT_URL: c_uint = 10002;
const CURLOPT_TIMEOUT_MS: c_uint = 155;
const CURLOPT_CONNECTTIMEOUT_MS: c_uint = 156;
const CURLOPT_WRITEFUNCTION: c_uint = 20011;
const CURLOPT_WRITEDATA: c_uint = 10001;
const CURLOPT_NOPROGRESS: c_uint = 43;
const CURLOPT_VERBOSE: c_uint = 41;
const CURLOPT_NOSIGNAL: c_uint = 99;
const CURLOPT_NOBODY: c_uint = 44; // HEAD request (faster, no body transfer)

#[allow(non_camel_case_types)]
type CURL = c_void;

#[allow(non_camel_case_types)]
type CURLcode = c_int;

#[allow(non_camel_case_types)]
type CURLoption = c_uint;

type WriteCallback = extern "C" fn(*mut c_char, usize, usize, *mut c_void) -> usize;

extern "C" {
    fn curl_easy_init() -> *mut CURL;
    fn curl_easy_setopt(handle: *mut CURL, option: CURLoption, ...) -> CURLcode;
    fn curl_easy_perform(handle: *mut CURL) -> CURLcode;
    fn curl_easy_cleanup(handle: *mut CURL);
}

// Write callback that discards all output (silent mode for faster fuzzing)
extern "C" fn write_callback(_ptr: *mut c_char, _size: usize, _nmemb: usize, _userdata: *mut c_void) -> usize {
    // Return size to indicate success, but don't actually write anything
    _size * _nmemb
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

        // Set URL - this is what we're fuzzing (URL parsing logic)
        let _ = curl_easy_setopt(handle, CURLOPT_URL, url.as_ptr() as *const c_char);
        
        // ULTRA SHORT timeouts - fail fast, don't wait for network
        let _ = curl_easy_setopt(handle, CURLOPT_TIMEOUT_MS, 1 as c_long); // 1ms total timeout
        let _ = curl_easy_setopt(handle, CURLOPT_CONNECTTIMEOUT_MS, 1 as c_long); // 1ms connect timeout
        
        // Disable progress meter
        let _ = curl_easy_setopt(handle, CURLOPT_NOPROGRESS, 1 as c_long);
        
        // Disable verbose output (no stderr spam)
        let _ = curl_easy_setopt(handle, CURLOPT_VERBOSE, 0 as c_long);
        
        // Disable signals (safer for fuzzing)
        let _ = curl_easy_setopt(handle, CURLOPT_NOSIGNAL, 1 as c_long);
        
        // Set write callback to discard output (silent mode)
        let _ = curl_easy_setopt(handle, CURLOPT_WRITEFUNCTION, write_callback as WriteCallback);
        let _ = curl_easy_setopt(handle, CURLOPT_WRITEDATA, ptr::null_mut::<c_void>());
        
        // Use HEAD request (NOBODY) - faster, no body transfer
        // This will still parse the URL and attempt connection, but fail fast
        let _ = curl_easy_setopt(handle, CURLOPT_NOBODY, 1 as c_long);
        
        // Try to connect only (don't transfer data) - but this might not work with all protocols
        // The key is: we want curl to PARSE the URL and validate it, but fail fast on network
        
        // Perform the request - will fail fast due to 1ms timeout
        // This exercises curl's URL parsing, validation, and connection setup logic
        // but times out immediately, making it super fast
        let _ = curl_easy_perform(handle);

        curl_easy_cleanup(handle);
    }
    true // Execution completed normally
}
