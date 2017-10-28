#![cfg(windows)]

extern crate clipboard_master;
extern crate clipboard_win;
extern crate clap;

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

use utils::ResultExt;

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

fn run() -> Result<i32, String> {
    let args = cli::Args::new()?;
    let cleaner = text::Cleaner::new(config::Config::from_file(&args.config)?);

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

    Master::new(ok_callback, error_callback).run().map(|_| 0i32).map_err_to_string("Aborted.")
}

fn main() {
    use std::process::exit;

    let code: i32 = match run() {
        Ok(res) => res,
        Err(error) => {
            eprintln!("{}", error);
            1
        }
    };

    exit(code);
}
