use rand::{thread_rng, Rng};
use chrono::{Utc, TimeZone};

pub fn generate_random_data(n_count: i32) -> Vec<String> {
    let mut rng = thread_rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    
    let mut values = Vec::new();
    
    for _ in 0..n_count {
        let id: i32 = rng.gen_range(1000..9999);
        
        // Generate 3 random chars
        let str_val: String = (0..3)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect();
        
        let number: i32 = rng.gen_range(100..999);
        
        // Generate random timestamp
        let timestamp = Utc.with_ymd_and_hms(2025, 1, 15, 
            rng.gen_range(0..24),
            rng.gen_range(0..60),
            rng.gen_range(0..60))
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S.%f")
            .to_string();
        
        values.push(format!("({}, '{}', {}, TIMESTAMP '{}')", 
            id, str_val, number, timestamp));
    }
    
    // println!("{};", values.join(","));

    values
}