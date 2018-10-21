extern crate notify;

// use notify::{RecommendedWatcher, Watcher, RecursiveMode};
// use std::sync::mpsc::channel;
// use std::time::Duration;

#[no_mangle]
pub extern "C" fn nfc_init() {
    println!("nfc_init() called");
}

#[no_mangle]
pub extern "C" fn nfc_start(callback: fn(i32) -> ()) {
    println!("nfc_start() called");
}

#[no_mangle]
pub extern "C" fn nfc_release() {
    println!("nfc_release() called");
}