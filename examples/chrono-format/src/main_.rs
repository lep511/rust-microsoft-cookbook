use chrono::{DateTime, NaiveDateTime, Utc, TimeZone};

fn parse_date(date_string: &str) -> Result<String, Box<dyn std::error::Error>> {
    // List of common datetime formats with time zones
    let formats = [
        "%Y-%m-%d %H:%M:%S",           // 2023-01-15 14:30:15
        "%Y-%m-%d %H:%M:%S.%f",        // 2023-01-15 14:30:15.123
        "%Y/%m/%d %H:%M:%S",           // 2023/01/15 14:30:15
        "%Y-%m-%dT%H:%M:%S",           // 2023-01-15T14:30:15
        "%Y-%m-%dT%H:%M:%S%z",         // 2023-01-15T14:30:15+00:00
        "%Y-%m-%dT%H:%M:%S.%f%z",      // 2023-01-15T14:30:15.123+00:00
        "%a, %d %b %Y %H:%M:%S %z",    // Tue, 15 Jan 2023 14:30:15 +0000
        "%A, %d-%b-%y %H:%M:%S %z",    // Friday, 21-Mar-25 14:47:21 UTC
        "%A, %d-%b-%y %H:%M:%S %Z",    // Friday, 21-Mar-25 14:47:21 UTC
        "%m/%d/%Y @ %I:%M%p",          // UTC: 03/21/2025 @ 2:47pm
        "%m/%d/%Y @ %I:%M:%S %p",      // UTC with seconds (optional support)
    ];

    // Try parsing datetime with timezone
    for format in formats {
        if let Ok(dt) = DateTime::parse_from_str(date_string, format) {
            let dt_utc = dt.with_timezone(&Utc);
            return Ok(dt_utc.format("%Y-%m-%d %H:%M:%S%.3f").to_string());
        }
    }

    // Try parsing naive datetime (assuming UTC)
    let naive_formats = [
        "%Y-%m-%dT%H:%M:%S%z",          // ISO 8601, RFC 3339: 2025-03-21T14:47:21+00:00
        "%Y-%m-%dT%H:%M:%S.%f%z",       // With microseconds
        "%a, %d %b %Y %H:%M:%S %z",     // RFC 822/2822: Fri, 21 Mar 2025 14:47:21 +0000
        "%A, %d-%b-%y %H:%M:%S %Z",     // RFC 2822 variant: Friday, 21-Mar-25 14:47:21 UTC
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M:%S.%f",
        "%Y/%m/%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M:%SZ",
        "%a, %d %b %Y %H:%M:%S %z",
        "%m/%d/%Y @ %I:%M%p",
        "%m/%d/%Y @ %I:%M:%S %p",
    ];

    for format in naive_formats {
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_string, format) {
            let dt = Utc.from_utc_datetime(&naive_dt);
            return Ok(dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string());
        }
    }

    // Try parsing as a Unix timestamp (seconds or milliseconds)
    if let Ok(timestamp) = date_string.parse::<i64>() {
        let (seconds, nanos) = if timestamp.abs() > 9_999_999_999 {
            // Assume milliseconds
            let seconds = timestamp / 1000;
            let millis_remainder = (timestamp % 1000).abs();
            (seconds, (millis_remainder * 1_000_000) as u32)
        } else {
            // Assume seconds
            (timestamp, 0)
        };

        if let chrono::LocalResult::Single(dt) = Utc.timestamp_opt(seconds, nanos) {
            return Ok(dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string());
        }
    }

    // If parsing fails, return an error
    Err(format!("Unable to parse date: {}", date_string).into())
}

fn main() {
    let test_strings = [
        "2025-01-05 09:02:03.521861",
        "2023-01-15T14:30:15Z",
        "Friday, 21-Mar-25 14:47:21",
        "1673793015",      // Unix timestamp (seconds)
        "1736067723521",   // Unix timestamp (milliseconds)
    ];

    for s in &test_strings {
        match parse_date(s) {
            Ok(timestamp) => println!("'{}' -> '{}'", s, timestamp),
            Err(e) => println!("Error: {}", e),
        }
    }
}