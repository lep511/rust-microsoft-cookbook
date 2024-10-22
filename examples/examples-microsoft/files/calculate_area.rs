use std::fmt;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 {
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1);
    println!("{:#?}", p1);

    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
}
