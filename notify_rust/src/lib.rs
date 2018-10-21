extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent, Error};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::marker::Sync;
use std::time::Duration;

use std::ffi::CStr;
use std::os::raw::c_char;

struct Notifier {
    watcher: RecommendedWatcher,
    receiver: Receiver<DebouncedEvent>,
}
unsafe impl Sync for Notifier {}

static mut notifier: Option<Notifier> = None;

#[no_mangle]
pub extern "C" fn nfc_init() {
    println!("nfc_init() called");
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher_result: Result<RecommendedWatcher, Error> = Watcher::new(tx, Duration::from_secs(2));
    if let Err(e) = watcher_result {
        eprintln!("notify initialization error: {}", e);
        return;
    }

    unsafe {
        notifier = Some(Notifier {
            watcher: watcher_result.unwrap(),
            receiver: rx,
        });
    }
    println!("notify initialization successs");
}

#[no_mangle]
pub extern "C" fn nfc_start(path: *const c_char, callback: fn(i32) -> ()) {
    let noti = unsafe { notifier.as_mut().unwrap() };

    let path = unsafe { CStr::from_ptr(path).to_str().expect("failed to decode path for watching") };
    println!("nfc_start() called with path: {}", path);

    let watch_result = noti.watcher.watch(path, RecursiveMode::Recursive);
    if let Err(e) = watch_result {
        eprintln!("failed to start watch: {}", e);
        return;
    }

    loop {
        match noti.receiver.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

#[no_mangle]
pub extern "C" fn nfc_release() {
    println!("nfc_release() called");
}