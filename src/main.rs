#![cfg(windows)]

extern crate clipboard_master;
extern crate clipboard_win;
extern crate clap;

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

fn main() {
    set_panic_message!(lazy_panic::formatter::JustError);

    let args = cli::Args::new();
    let cleaner = text::Cleaner::new(config::Config::from_file(&args.config).expect("Bad config file"));

    let ok_callback = move || {
        const RES: CallbackResult = CallbackResult::Next;

        if !Clipboard::is_format_avail(formats::CF_UNICODETEXT) {
            return RES;
        }

        let clip = open_clipboard();
        let content = get_clipboard_string(&clip);

        if let Some(new_text) = cleaner.clean(&content) {
            set_clipboard_string(&clip, &new_text)
        }

        RES
    };

    Master::new(ok_callback, error_callback).run().expect("Aborted")
}
