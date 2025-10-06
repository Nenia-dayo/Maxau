use crate::player::metadata::MetaData;

pub fn display_info(filename: &str, metadata: &MetaData) {
    println!("{}", string_info(filename, metadata));
}

pub fn string_info(filename: &str, metadata: &MetaData) -> String {
    let parts: Vec<String> = vec![
        metadata.album().map(|a| format!("[{}] ", a)),
        metadata.artist().map(|a| format!("{} - ", a)),
        metadata.title().map(|t| t.to_string()),
    ]
    .into_iter()
    .map(|f| f.unwrap_or(String::new()).to_string())
    .collect();

    if parts.is_empty() || metadata.title().is_none() {
        filename.to_string()
    } else {
        parts.concat().to_string()
    }
}
