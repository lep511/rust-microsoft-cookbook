mod mathlib;
mod authentication;

use mathlib::math;
use regex::Regex;

fn main() {
    println!("{}", math::sin(45.0));
    println!("{}", math::cos(45.0));
    println!("{}", math::tan(45.0));
    println!("{:?}", math::add((1.0, 2.0), (3.0, 4.0)));

    let mut user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.get_username());
    user.set_password("even-more-secret");

    let test_date: &str = "2014-01-31";
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date ({}) match YYYY-MM-DD? {}", test_date, re.is_match(test_date));
}