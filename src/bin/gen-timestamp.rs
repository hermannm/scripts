use std::time::SystemTime;

use chrono::{DateTime, SecondsFormat, Utc};

fn main() {
    let now = DateTime::<Utc>::from(SystemTime::now());
    let formatted = now.to_rfc3339_opts(SecondsFormat::Secs, true);
    println!("{formatted}")
}
