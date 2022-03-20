/*!
Code in this file is copied from:

Github: https://github.com/Canop/file-size
Author: Denys Séguret (https://github.com/Canop)


With changes.
!*/


/// produce the most precise and nearest ISO size writing
/// fitting in 4 characters of the given integer size
pub fn format_size(size: u64) -> String {
    // if you have more efficient or prettier, please tell me
    match size {
        0..=9_999 => size.to_string(),
        10_000..=999_499 => format!("{:.0} kB", (size as f64) / 1_000.0),
        999_500..=9_950_000 => format!("{:.1} MB", (size as f64) / 1_000_000.0),
        9_950_001..=999_499_999 => format!("{:.0} MB", (size as f64) / 1_000_000.0),
        999_500_000..=9_950_000_000 => format!("{:.1} GB", (size as f64) / 1_000_000_000.0),
        9_950_000_001..=999_499_999_999 => format!("{:.0} GB", (size as f64) / 1_000_000_000.0),
        999_500_000_000..=9_950_000_000_000 => format!("{:.1} TB", (size as f64) / 1_000_000_000_000.0),
        9_950_000_000_001..=999_499_999_999_999 => format!("{:.0} TB", (size as f64) / 1_000_000_000_000.0),
        999_500_000_000_000..=9_950_000_000_000_000 => format!("{:.1} PB", (size as f64) / 1_000_000_000_000_000.0),
        9_950_000_000_000_001..=999_499_999_999_999_935 => format!("{:.0} PB", (size as f64) / 1_000_000_000_000_000.0),
        _ => "huge".to_string(), // good enough to me
    }
}
