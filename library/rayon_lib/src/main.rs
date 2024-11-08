use rand::Rng;
use rayon::prelude::*;

fn main() {
    
    let mut rng = rand::thread_rng();
    
    {
        let nums: Vec<i32> = (0..10000000).map(|_| rng.gen_range(1..=100)).collect();
        let start = std::time::Instant::now();
        // 1. For loop
        let mut _nums_squared: Vec<i32> = Vec::new();
        for num in nums {
            _nums_squared.push(num * num * num);
        }
        println!("With for.. Took {:?}", start.elapsed());
    }
    {
        let nums: Vec<i32> = (0..10000000).map(|_| rng.gen_range(1..=100)).collect();
        let start = std::time::Instant::now();
        // 2. Iterator
        let _nums_squared: Vec<i32> = nums.iter()
            .map(|&x| x * x * x)
            .collect();

        println!("With iter.. Took {:?}", start.elapsed());
    }
    {
        let nums: Vec<i32> = (0..10000000).map(|_| rng.gen_range(1..=100)).collect();
        let start = std::time::Instant::now();
        // 2. Iterator
        let _nums_squared: Vec<i32> = nums.par_iter()
            .map(|&x| x * x * x)
            .collect();

        println!("With Rayon.. Took {:?}", start.elapsed());

    }
}
