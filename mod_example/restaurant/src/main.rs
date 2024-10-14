use restaurant::front_of_house::hosting::{seat_at_table, add_to_waitlist};

fn main() {
    add_to_waitlist(String::from("main"));
    seat_at_table(String::from("main"));
}