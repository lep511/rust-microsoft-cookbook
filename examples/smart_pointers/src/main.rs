use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::cell::RefCell;

fn main() {

    // Smart pointers in Rust are data structures that not only act like regular pointers but also provide additional features, 
    // such as automatic memory management and ownership semantics. They help manage memory safely and efficiently, 
    // adhering to Rust's ownership model. Here are the most commonly used smart pointers in Rust:
    //
    // Box<T>: A smart pointer that allocates memory on the heap and manages its lifetime.
    // Rc<T>: A reference-counted smart pointer that allows multiple owners of the data.
    // Ref<T>: A smart pointer that provides immutable access to a value.
    // RefMut<T>: A smart pointer that provides mutable access to a value.
    // Mutex<T>: A smart pointer that provides thread-safe mutual exclusion.

    // Box<T>: A smart pointer that allocates memory on the heap.
    println!("Example of Box<T>");
    let a = Box::new(5); // Allocates an integer on the heap
    println!("Value in box: {}", a);

    // Rc<T>: A reference-counted smart pointer.
    // Rc stands for "Reference Counted." It is a smart pointer that enables multiple ownership of the same data.
    // It keeps track of the number of references to the data, and when the last reference goes out of scope, 
    // the data is deallocated.
    // Rc is useful in scenarios where you want to share data between 
    // multiple parts of your program without transferring ownership.
    
    let b = Rc::new(10);
    let c = Rc::clone(&b); // Increments the reference count

    println!("\nExample of Rc<T>");
    println!("Value of b: {}", b);
    println!("Value of c: {}", c);
    println!("Reference count: {}", Rc::strong_count(&b)); // Shows the number of references

    // Ref<T>: A smart pointer that provides immutable access to a value.
    // RefMut<T>: A smart pointer that provides mutable access to a value.
    // These smart pointers provide a way to access data without taking ownership.

    println!("\nExample of Ref<T>");
    let d = Rc::new(2);
    let e = Rc::as_ref(&d);
    println!("Value of e: {}", e);

    // Arc<T>
    // Arc stands for "Atomic Reference Counted." It is similar to Rc, but it is thread-safe, 
    // allowing multiple threads to own the same data. It uses atomic operations to manage 
    // the reference count.
    // Use Arc when you need to share data across threads safely.

    println!("\nExample of Arc<T>");
    let a = Arc::new(5);
    let a_clone = Arc::clone(&a);

    let handle = thread::spawn(move || {
        println!("Value in thread: {}", a_clone);
    });

    handle.join().unwrap();
    println!("Value in main thread: {}", a);  
    
    // RefCell<T>
    // RefCell is a mutable smart pointer that allows for interior mutability. 
    // It enables you to mutate data even when the RefCell itself is immutable. 
    // It uses runtime borrow checking to ensure that you do not have mutable 
    // and immutable references at the same time.
    // RefCell when you need to mutate data in a context where you cannot change the outer structure.

    println!("\nExample of RefCell<T>");
    let value = RefCell::new(5);
    
    *value.borrow_mut() += 1; // Mutably borrow and modify the value
    
    // The dereference operator "*" is used to access the actual value that the RefMut points to. 
    // It "unwraps" the smart pointer to get to the underlying value.

    println!("Value: {}", value.borrow()); // Borrow immutably to read the value
}
