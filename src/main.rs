#![cfg(windows)]

extern crate clipboard_master;
extern crate clipboard_win;
extern crate windows_win;

#[macro_use(set_panic_message)]
extern crate lazy_panic;

extern crate text;
extern crate config;
extern crate utils;

use std::io;

use clipboard_master::{
    Master,
    CallbackResult,
};
use clipboard_win::{
    Clipboard,
    formats
};

mod cli;

fn error_callback(error: io::Error) -> CallbackResult {
    eprintln!("Error: {}", error);
    CallbackResult::Next
}

fn open_clipboard() -> Clipboard {
    loop {
        match Clipboard::new() {
            Ok(clipboard) => return clipboard,
            Err(error) => eprintln!("Failed to open clipboard. Error: {}", error)
        }
    }
}

fn get_clipboard_string(clip: &Clipboard) -> String {
    loop {
        match clip.get_string() {
            Ok(content) => return content,
            Err(error) => eprintln!("Failed to get content from Clipboard. Error: {}", error)
        }
    }
}

fn set_clipboard_string(clip: &Clipboard, content: &str) {
    loop {
        match clip.set_string(content) {
            Ok(_) => break,
            Err(error) => eprintln!("Failed to set content onto Clipboard. Error: {}", error)
        }
    }
}

static mut CLEANER: Option<text::Cleaner> = None;

fn update_clipboard() {
    if !Clipboard::is_format_avail(formats::CF_UNICODETEXT) {
        return;
    }

    let clip = open_clipboard();
    let content = get_clipboard_string(&clip);

    let cleaner = unsafe {CLEANER.as_ref().expect("Static cleaner is missing")};
    if let Some(new_text) = cleaner.clean(&content) {
        set_clipboard_string(&clip, &new_text)
    }
}

fn create_timer(delay: std::os::raw::c_ulong) -> io::Result<windows_win::raw::timer::QueueTimer> {
    windows_win::TimerBuilder::new().rust_callback(update_clipboard).single(delay).build()
}

fn main() {
    const DELAY_MS: std::os::raw::c_ulong = 500;
    set_panic_message!(lazy_panic::formatter::JustError);

    let args = cli::Args::new();
    unsafe {
        let cleaner = text::Cleaner::new(config::Config::from_file(&args.config).expect("Bad config file"));
        CLEANER = Some(cleaner)
    }

    let mut timer = Some(create_timer(std::os::raw::c_ulong::max_value()).expect("To create timer"));

    let ok_callback = move || {
        timer.take().expect("To have timer").delete(windows_win::raw::timer::Wait).expect("To delete timer");
        timer = Some(create_timer(DELAY_MS).expect("To create timer"));
        CallbackResult::Next
    };

    Master::new(ok_callback, error_callback).run().expect("Aborted")
}
