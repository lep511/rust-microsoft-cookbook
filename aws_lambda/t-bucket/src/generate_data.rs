use chrono::{Utc, TimeZone};
use rand::Rng;
use rand::rng;

pub fn process_data(csv_data: Vec<String>) -> Vec<String> {
    let mut rng = rng();
    let mut processed_data = Vec::new();

    for line in csv_data {
        

        // Join the parts back into a CSV line
        let processed_line = parts.join(",");
        processed_data.push(processed_line);
    }

    processed_data

}

#[allow(dead_code)]
pub fn generate_random_data(n_count: i32) -> Vec<String> {
    let mut rng = rng();
    let carriers = vec!["WN", "AA", "UA", "DL", "AS", "B6", "NK", "F9", "HA", "VX"]; // Example carriers
    let cancel_codes = vec!["A", "B", "C", "D"];
    
    let mut values = Vec::new();
    
    for count in 0..n_count {
        // Basic flight info
        let year = rng.random_range(2000..=2023);
        let month = rng.random_range(1..=12);
        let day_of_month = rng.random_range(1..=28); // Simplified for all months
        let day_of_week = rng.random_range(1..=7);
        
        // Time information
        let dep_time = rng.random_range(0..2400) as f32;
        let crs_dep_time = (dep_time as i32 / 5) * 5; // Rounded to nearest 5
        
        // Generate arrival times (typically 1-4 hours after departure)
        let flight_duration = rng.random_range(60..240);
        let arr_time = (dep_time + flight_duration as f32).min(2359.0);
        let crs_arr_time = ((arr_time as i32) / 5) * 5;
        
        // Flight details
        let carrier = carriers[rng.random_range(0..carriers.len())];
        let flight_num = rng.random_range(100..4000);
        
        // Delays and ground operations
        let taxi_in = rng.random_range(1..=10) as f32;
        let taxi_out = rng.random_range(1..=20) as f32;
        
        // Status flags
        let cancelled = rand::rng().random_bool(0.1);
        let cancel_code = cancel_codes[rng.random_range(0..cancel_codes.len())];
        let diverted = rand::rng().random_bool(0.2);
      
        // Delays with random NULL values
        let carrier_delay = if rng.random_bool(0.1) { None } else { Some(rng.random_range(1..=1200) as f32) };
        let weather_delay = if rng.random_bool(0.1) { None } else { Some(rng.random_range(1..=1200) as f32) };
        let nas_delay = if rng.random_bool(0.1) { None } else { Some(rng.random_range(1..=1200) as f32) };
        let security_delay = if rng.random_bool(0.1) { None } else { Some(rng.random_range(1..=1200) as f32) };

        let carrier_delay = format_nullable(carrier_delay);
        let weather_delay = format_nullable(weather_delay);
        let nas_delay = format_nullable(nas_delay);
        let security_delay = format_nullable(security_delay);
        let flight_date = Utc.with_ymd_and_hms(
            year,
            month,
            day_of_month,
            rng.random_range(0..24),
            rng.random_range(0..60),
            rng.random_range(0..60))
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S.%f")
            .to_string();

        values.push(format!(
            "SELECT {year}, {month}, {day_of_month}, {day_of_week}, {dep_time}, {crs_dep_time}, \
            {arr_time}, {crs_arr_time}, '{carrier}', {flight_num}, {taxi_in}, {taxi_out}, \
            {cancelled}, '{cancel_code}', {diverted}, {carrier_delay}, {weather_delay}, \
            {nas_delay}, {security_delay}, TIMESTAMP '{flight_date}'"));

        if count != n_count - 1 {
            values.push(String::from("UNION ALL"));
        }

    }

    values
}

// Helper function to format Option<f32> as "NULL" or its value
#[allow(dead_code)]
fn format_nullable(value: Option<f32>) -> String {
    value.map_or_else(|| "NULL".to_string(), |v| v.to_string())
}