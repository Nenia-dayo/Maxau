#[macro_export]
macro_rules! err {
    ($($msg: expr), *) => {{
        use $crate::crossterm::style::Stylize;
        eprintln!("{} {}", "Error:".red().bold(), format!($($msg), *).red());
    }};
}
