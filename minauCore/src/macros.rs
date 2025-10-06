#[macro_export]
macro_rules! err {
    ($($msg: expr), *) => {{
        use crossterm::style::Stylize;
        eprintln!("{} {}", "Error:".red().bold(), format!($($msg), *).red());
    }};
}
