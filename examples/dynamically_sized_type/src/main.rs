trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

fn create_shape(shape_type: &str) -> Box<dyn Shape> {
    match shape_type {
        "circle" => Box::new(Circle { radius: 1.0 }),
        "rectangle" => Box::new(Rectangle { width: 2.0, height: 3.0 }),
        _ => panic!("Unknown shape type"),
    }
}

fn main() {

    // In Rust, dyn is used to indicate a dynamically sized type, particularly when working with trait objects. 
    // This allows you to use traits as types, enabling polymorphism. Here are some examples to illustrate how to use dyn effectively.

    // Using dyn to Create a Trait Object:

    let circle: Box<dyn Shape> = Box::new(Circle { radius: 5.0 });
    let rectangle: Box<dyn Shape> = Box::new(Rectangle { width: 4.0, height: 6.0 });

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());

    // Using dyn in Function Parameters:

    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 2.0, height: 5.0 };

    print_area(&circle);
    print_area(&rectangle);

    // Using dyn with Collections:

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle { width: 3.0, height: 4.0 }),
    ];

    for shape in shapes {
        println!("Area: {}", shape.area());
    }

    // Using dyn with Return Types

    let shape = create_shape("circle");
    println!("Circle area: {}", shape.area());

    let shape = create_shape("rectangle");
    println!("Rectangle area: {}", shape.area());

}

