mod col1;
mod col2;
mod col3;

use rand::seq::SliceRandom;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let column1: Vec<&str> = col1::get_col1();
    let column2: Vec<&str> = col2::get_col2();
    let column3: Vec<&str> = col3::get_col3();

    // Randomly select one value from each column
    let mut rng = rand::thread_rng();
    let mut words: Vec<&str> = Vec::new();

    if let Some(value) = column1.choose(&mut rng) {
        words.push(&value);
    }
    if let Some(value) = column2.choose(&mut rng) {
        words.push(&value);
    }
    if let Some(value) = column3.choose(&mut rng) {
        words.push(&value);
    }

    let prompt: String = format!("Generates an offensive sentence in Shakespeare's style with the following three words: {}, {} and {}.", words[0], words[1], words[2]);
    println!("{}", prompt);

    Ok(())
}