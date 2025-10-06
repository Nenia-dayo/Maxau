use crate::display_info;
use crate::player::metadata::MetaData;
use crossterm::terminal;
use crossterm::{
    cursor::{self, MoveToPreviousLine},
    execute,
    terminal::{Clear, ClearType},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant, sleep};
use unicode_width::UnicodeWidthStr;

static LAST_CALL: once_cell::sync::Lazy<Arc<Mutex<Option<Instant>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

pub fn info<P: AsRef<str>>(msg: P) {
    let mut stdout = std::io::stdout();

    execute!(
        stdout,
        cursor::MoveToPreviousLine(1),
        Clear(ClearType::CurrentLine),
    )
    .unwrap();

    println!("{}", msg.as_ref());
}

pub fn info_with_restore<P: AsRef<str>>(
    msg: P,
    filename: String,
    path: String,
    metadata: MetaData,
) {
    info(msg);

    tokio::spawn(async move {
        let call_time = Instant::now();
        {
            *LAST_CALL.lock().await = Some(call_time);
        }

        sleep(Duration::from_millis(2400)).await;

        let last = LAST_CALL.lock().await;
        if last.is_some_and(|t| t != call_time) {
            return;
        }

        let text_width =
            UnicodeWidthStr::width(display_info::string_info(&path, &metadata).as_str());
        let (cols, _rows) = terminal::size().unwrap_or((80, 24));
        let lines_needed = (text_width as u16).div_ceil(cols).max(1);

        for _ in 0..lines_needed {
            execute!(
                std::io::stdout(),
                MoveToPreviousLine(1),
                Clear(ClearType::CurrentLine),
            )
            .unwrap();
        }
        crate::display_info::display_info(&filename, &metadata);
    });
}

pub fn info_with_restore_url<P: AsRef<str>>(msg: P, url: &str) {
    let url = String::from(url);
    info(msg);

    tokio::spawn(async move {
        let call_time = Instant::now();
        {
            *LAST_CALL.lock().await = Some(call_time);
        }

        sleep(Duration::from_millis(2400)).await;

        let last = LAST_CALL.lock().await;
        if last.is_some_and(|t| t != call_time) {
            return;
        }

        let text_width = UnicodeWidthStr::width(url.as_str());
        let (cols, _rows) = terminal::size().unwrap_or((80, 24));
        let lines_needed = (text_width as u16).div_ceil(cols).max(1);

        for _ in 0..lines_needed {
            execute!(
                std::io::stdout(),
                MoveToPreviousLine(1),
                Clear(ClearType::CurrentLine),
            )
            .unwrap();
        }

        println!("{}", url);
    });
}
