fn main() {
    test_clone();
    test_to_owned();
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn test_clone() {
    let s1: &'static str = "I am static";
    let s2 = "I am boxed and owned".to_string();

    let c1 = s1;
    let c2 = s2.clone();

    print_type_of(&c1);
    print_type_of(&c2);
    print_type_of(&s1);
    print_type_of(&s2);

    println!("{:?}", c1);
    println!("{:?}", c2);

    println!("{:?}", c1 == s1);  // prints true
    println!("{:?}", c2 == s2);  // prints true
}

fn test_to_owned() {
    let s1: &'static str = "I am static";
    let s2 = "I am boxed and owned".to_string();

    let c1 = s1.to_owned();
    let c2 = s2.to_owned();

    println!("--------------------\n");
    print_type_of(&c1);
    print_type_of(&c2);
    print_type_of(&s1);
    print_type_of(&s2);
    
    println!("{:?}", c1);
    println!("{:?}", c2);

    println!("{:?}", c1 == s1);
    println!("{:?}", c2 == s2);
}