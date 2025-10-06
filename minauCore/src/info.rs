use crate::display_info;
use crate::player::metadata::MetaData;
use crossterm::terminal;
use crossterm::{
    cursor::{self, MoveToPreviousLine},
    execute,
    terminal::{Clear, ClearType},
};
use smol::Timer;
use smol::lock::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use unicode_width::UnicodeWidthStr;

static LAST_CALL: once_cell::sync::Lazy<Arc<Mutex<Option<Instant>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

pub fn info<P: AsRef<str>>(msg: P) {
    let mut stdout = std::io::stdout();

    let _ = execute!(
        stdout,
        cursor::MoveToPreviousLine(1),
        Clear(ClearType::CurrentLine),
    );

    println!("{}", msg.as_ref());
}

pub fn info_with_restore<P: AsRef<str>>(
    msg: P,
    filename: String,
    path: String,
    metadata: MetaData,
) {
    info(msg);

    thread::spawn(move || {
        smol::block_on(async move {
            let call_time = Instant::now();
            {
                *LAST_CALL.lock().await = Some(call_time);
            }

            Timer::after(Duration::from_millis(2400)).await;

            let last = LAST_CALL.lock().await;
            if last.is_some_and(|t| t != call_time) {
                return;
            }

            let text_width =
                UnicodeWidthStr::width(display_info::string_info(&path, &metadata).as_str());
            let (cols, _rows) = terminal::size().unwrap_or((80, 24));
            let lines_needed = (text_width as u16).div_ceil(cols).max(1);

            for _ in 0..lines_needed {
                let _ = execute!(
                    std::io::stdout(),
                    MoveToPreviousLine(1),
                    Clear(ClearType::CurrentLine),
                );
            }
            crate::display_info::display_info(&filename, &metadata);
        });
    });
}

pub fn info_with_restore_url<P: AsRef<str>>(msg: P, url: &str) {
    let url = String::from(url);
    info(msg);

    thread::spawn(move || {
        smol::block_on(async move {
            let call_time = Instant::now();
            {
                *LAST_CALL.lock().await = Some(call_time);
            }

            Timer::after(Duration::from_millis(2400)).await;

            let last = LAST_CALL.lock().await;
            if last.is_some_and(|t| t != call_time) {
                return;
            }

            let text_width = UnicodeWidthStr::width(url.as_str());
            let (cols, _rows) = terminal::size().unwrap_or((80, 24));
            let lines_needed = (text_width as u16).div_ceil(cols).max(1);

            for _ in 0..lines_needed {
                let _ = execute!(
                    std::io::stdout(),
                    MoveToPreviousLine(1),
                    Clear(ClearType::CurrentLine),
                );
            }

            println!("{}", url);
        });
    });
}
