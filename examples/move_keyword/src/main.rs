use std::thread;

fn main() {

    // In Rust, the move keyword is used to transfer ownership of variables, especially when working with closures. 
    // When you use move, the closure takes ownership of the variables it captures, rather than borrowing them. 
    // This is particularly useful in concurrent programming or when you want to ensure 
    // that the closure has its own copy of the data.

    let x = String::from("Hello");

    // Using a closure without move (borrowing x)
    let print_x = || {
        println!("{}", x); // This will cause a compile error if x is used after this closure
    };

    // Uncommenting the next line will cause a compile error
    // print_x();
    // println!("{}", x); // x is still valid here

    // Using move to take ownership of x
    let print_x_move = move || {
        println!("{}", x); // Now x is moved into the closure
    };

    print_x_move(); // This works
    // println!("{}", x); // This will cause a compile error because x has been moved


    // Using move with Threads
    let message = String::from("Hello from the thread!");

    // Using move to transfer ownership to the thread
    let handle = thread::spawn(move || {
        println!("{}", message); // message is moved into the thread
    });

    handle.join().unwrap(); // Wait for the thread to finish
    // println!("{}", message); // This will cause a compile error because message has been moved


    // Using move with Iterators
    // You can also use move with iterators to ensure that the closure captures the variables by value.
    let numbers = vec![1, 2, 3];

    let sum: i32 = numbers.iter().map(move |&x| {
        x + 1 // Here, x is moved into the closure
    }).sum();

    println!("Sum: {}", sum); // This works
    // println!("{:?}", numbers); // This will cause a compile error because numbers is borrowed

}
