use std::mem;

fn main() {
    let story = String::from("Once upon a time...");

    // Prevent automatically dropping the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    println!("ptr: {:p}", ptr);
    // story has nineteen bytes
    assert_eq!(19, len);
    println!("story has {} bytes", len);

    // We can re-build a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) } ;

    assert_eq!(String::from("Once upon a time..."), s);
    println!("{}", s);
}