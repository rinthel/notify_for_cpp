extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent, Error};
use std::sync::mpsc::channel;
use std::marker::Sync;
use std::time::Duration;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::thread;

pub mod event_id {
    pub const NOTICE_WRITE: i32 = 1;
    pub const NOTICE_REMOVE: i32 = 2;
    pub const CREATE: i32 = 3;
    pub const WRITE: i32 = 4;
    pub const CHMOD: i32 = 5;
    pub const REMOVE: i32 = 6;
    pub const RENAME: i32 = 7;
    pub const RESCAN: i32 = 8;
    pub const ERROR: i32 = -1;
}

struct Notifier {
    watch_thread: Option<thread::JoinHandle<()>>,
    on_watching: bool,
}
unsafe impl Sync for Notifier {}

static mut NOTIFIER: Option<Notifier> = None;

#[no_mangle]
pub extern "C" fn nfc_start(path: *const c_char, callback: fn(i32, *const c_char) -> ()) -> i32 {
    println!("nfc_start() called");
    let (tx, rx) = channel();

    let path = unsafe {
        let path_str = CStr::from_ptr(path).to_str();
        if let Err(e) = path_str {
            eprintln!("[notify_rust] failed to decode path for watching: {}", e);
            return 0;
        }
        path_str.unwrap()
    };

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher_result: Result<RecommendedWatcher, Error> = Watcher::new(tx, Duration::from_secs(2));
    if let Err(e) = watcher_result {
        eprintln!("[notify_rust] notify initialization error: {}", e);
        return 0;
    }

    let watch_thread = thread::spawn(move || {
        let mut watcher = watcher_result.unwrap();
        if let Err(e) = watcher.watch(path, RecursiveMode::Recursive) {
            eprintln!("[notify_rust] failed to start watch: {}", e);
            return;
        }
        loop {
            unsafe {
                if !NOTIFIER.as_ref().unwrap().on_watching {
                    break;
                }
            }
            let call = |event_id: i32, pathbuf: std::path::PathBuf| {
                callback(event_id, (CString::new(pathbuf.into_os_string().into_string().unwrap()).unwrap()).as_ptr());
            };
            match rx.recv_timeout(Duration::from_secs(2)) {
                Ok(event) => {
                    // println!("{:?}", event);
                    match event {
                        DebouncedEvent::NoticeWrite(p) => call(event_id::NOTICE_WRITE, p),
                        DebouncedEvent::NoticeRemove(p) => call(event_id::NOTICE_REMOVE, p),
                        DebouncedEvent::Create(p) => call(event_id::CREATE, p),
                        DebouncedEvent::Write(p) => call(event_id::WRITE, p),
                        DebouncedEvent::Chmod(p) => call(event_id::CHMOD, p),
                        DebouncedEvent::Remove(p) => call(event_id::REMOVE, p),
                        DebouncedEvent::Rename(p1, p2) => {
                            call(event_id::REMOVE, p1);
                            call(event_id::CREATE, p2);
                        },
                        DebouncedEvent::Rescan => callback(event_id::RESCAN, 0 as *const c_char),
                        DebouncedEvent::Error(e, _) => callback(event_id::ERROR,
                            CString::new(format!("{}", e)).unwrap().as_ptr()),
                    }
                },
                Err(e) => {
                    match e {
                        std::sync::mpsc::RecvTimeoutError::Timeout => {},
                        _ => println!("[notify_rust] watch error: {:?}", e),
                    }
                }
            }
        }
    });

    unsafe {
        NOTIFIER = Some(Notifier {
            watch_thread: Some(watch_thread),
            on_watching: true,
        });
    }

    1
}

#[no_mangle]
pub extern "C" fn nfc_stop() {
    println!("nfc_release() called");
    unsafe {
        if let Some(ref mut noti) = NOTIFIER {
            noti.on_watching = false;
            if let Err(_) = noti.watch_thread.take().unwrap().join() {
                eprintln!("[notify_rust] failed to join watch thread");
            }
        }
        NOTIFIER = None;
    }
}