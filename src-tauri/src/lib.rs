#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use libc::c_char;
use std::{
    ffi::{CStr, CString},
    sync::{Arc, Mutex},
};
use tauri::State;

type HostCallback = unsafe extern "C" fn(s: *const c_char) -> *mut c_char;

struct AppState {
    host_callback: Arc<Mutex<HostCallback>>,
}

impl AppState {
    fn new(cb: HostCallback) -> Self {
        AppState {
            host_callback: Arc::new(Mutex::new(cb)),
        }
    }
}

#[tauri::command(async)]
fn command_run<'a>(payload: &'a str, state: State<'_, AppState>) -> &'a str {
    println!("rust: from js: {}", payload);
    let msg = CString::new(payload).unwrap().into_raw();
    let response = unsafe {
        let response = state.host_callback.lock().unwrap()(msg);
        assert!(!response.is_null());
        let c_str = CStr::from_ptr(response);
        c_str.to_str().unwrap()
    };
    println!("rust: from host: {}", response);
    response
}

#[no_mangle]
pub extern "C" fn message_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern "C" fn run_tauri(cb: HostCallback) {
    tauri::Builder::default()
        .manage(AppState::new(cb))
        .invoke_handler(tauri::generate_handler![command_run])
        .run(tauri::generate_context!("./tauri.conf.json"))
        .expect("error while running tauri application");
}
