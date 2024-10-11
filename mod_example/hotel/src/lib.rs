#[allow(dead_code)]
mod reception {
    pub mod booking {
        pub fn book_room() {
            println!("Booking room!")
        }
        fn cancel_booking() {
            println!("Cancelling booking!")
        }
    }

    pub mod guests_management {
        pub fn guest_checkin() {
            println!("Checking in!")
        }
        pub fn guest_checkout() {
            println!("Checking out!")
        }
        fn receive_guest_request() {
            println!("Receiving request!")
        }
        pub fn get_room_number() -> i8 {
            10
        }
    }
    #[derive(Debug)]
    pub struct Room {
        pub view: String,
        pub beds_count: i8,
        number: i8,
    }
    impl Room {
        pub fn new(view: &str, beds: i8) -> Room {
            Room {
                view: String::from(view),
                beds_count: beds,
                number: guests_management::get_room_number(),
            }
        }
    }
}

#[allow(dead_code)]
mod facilities {
    mod house_keeping {
        fn clean_room() {
            println!("Cleaning room!")
        }
        fn deliver_meal() {
            println!("Delivering meal!")
        }
    }

    mod maintenance {
        fn pool_maintenance() {
            println!("Cleaning pool!")
        }
        fn electrical_maintenance() {
            println!("Fixing electrical!")
        }
        fn building_maintenance() {
            println!("Fixing building!")
        }
    }

    pub mod restaurants {
        pub fn prepare_tables() {
            println!("Preparing table")
        }
        pub fn make_meal() {
            println!("Making meal")
        }
        #[derive(Debug)]
        pub enum Meal {
            Breakfast,
            Brunch,
            Lunch,
            Diner,
        }
    }
}

#[allow(dead_code)]
pub mod guests {
    use super::reception::guests_management::guest_checkout;
    use crate::facilities::restaurants;
    fn book_a_room() {
        super::reception::booking::book_room();
        self::go_to_beach();
    }
    fn go_to_beach() {
        println!("Going to the beach!");
    }
    fn go_to_pool() {
        println!("Going to the pool!");
    }
    pub fn eat_meal() {
        restaurants::prepare_tables();
        restaurants::make_meal();
        let my_meal = crate::facilities::restaurants::Meal::Diner;
        println!("Eating {my_meal:?}");
    }
    fn end_vacation() {
        guest_checkout();
        println!("Bye bye!");
    }
}

#[allow(dead_code)]
pub fn go_on_vacation() {
    crate::reception::booking::book_room();         // Absolute path
    reception::guests_management::guest_checkin();  // Relative path

    let my_room = reception::Room::new("Sea view", 2);

    println!("My room's is {}. {:?}", my_room.view, my_room);
}

