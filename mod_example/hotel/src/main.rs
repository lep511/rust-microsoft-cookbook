mod hoteldir;
pub mod books;

use crate::books::mail::Mail;
use crate::books::phone::Phone;
use hoteldir::Room;

fn main() {
    
    let mail = Mail::new(
        "sender@example.com".to_string(),
        "recipient@example.com".to_string(),
        "Hello".to_string(),
        "This is a test email.".to_string(),
    );
    
    let phone = Phone::new(
        "588689845".to_string(),
        "Mobile".to_string(),
    );

    println!("The mail is {} and phone number is {}", mail.from, phone.phone_number);

    hotel::go_on_vacation();
    hotel::guests::eat_meal();

    println!("-------------------------------------------------\n");

    let mut hotel = hoteldir::Hotel::new();
    hotel.add_room(Room { number: 101, beds: 2, price: 150.00, occupied: false });
    hotel.add_room(Room { number: 102, beds: 1, price: 80.00, occupied: true });
    hotel.add_room(Room { number: 201, beds: 2, price: 180.00, occupied: false });

    println!("Available rooms: {:?}", hotel.get_available_rooms());

    match hotel.check_in(101) {
        Ok(_) => println!("Checked in to room 101"),
        Err(e) => println!("Error: {}", e),
    }


    match hotel.check_out(102) {
        Ok(_) => println!("Checked out of room 102"),
        Err(e) => println!("Error: {}", e),
    }


    match hotel.get_room_info(201) {
        Some(room) => println!("Room 201 info: {:?}", room),
        None => println!("Room 201 not found"),
    }


    println!("Available rooms: {:?}", hotel.get_available_rooms());

    match hotel.check_in(101) {  // Try checking into an occupied room.
        Ok(_) => println!("Checked in to room 101"),
        Err(e) => println!("Error: {}", e),
    }
}
