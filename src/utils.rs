use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_iso(time: SystemTime) -> Result<String, Box<dyn std::error::Error>> {
    let total_secs = time.duration_since(UNIX_EPOCH)?.as_secs();

    let secs = total_secs % 60;
    let total_mins = total_secs / 60;
    let mins = total_mins % 60;
    let total_hours = total_mins / 60;
    let hours = total_hours % 24;
    let days_since_epoch = total_hours / 24;

    let z = days_since_epoch as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };

    Ok(format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, m, d, hours, mins, secs
    ))
}
pub fn get_size_str(size: &u64) -> String {
    const SUFFIX: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut i = 0;
    let mut val = *size as f64;

    while val >= 1024.0 && i < SUFFIX.len() - 1 {
        val /= 1024.0;
        i += 1;
    }

    format!("{:.2}{}", val, SUFFIX[i])
}
