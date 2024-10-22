use rand::seq::SliceRandom;
use rand::Rng;
use uuid::Uuid;

const COLORS: &[&str] = &[
    "Red", "Green", "Blue", "Yellow", "Orange", "Purple", "Pink", "Brown",
    "Black", "White", "Gray", "Silver", "Gold", "Cyan", "Magenta", "Maroon",
    "Navy", "Olive", "Teal", "Aqua", "Lime", "Coral", "Aquamarine",
    "Turquoise", "Violet", "Indigo", "Plum", "Crimson", "Salmon", "Coral",
    "Khaki", "Beige",
];

const PRODUCTS: &[&str] = &[
    "Shoes", "Sweatshirts", "Hats", "Pants", "Shirts", "T-Shirts", "Trousers",
    "Jackets", "Shorts", "Skirts", "Dresses", "Coats", "Jeans", "Blazers",
    "Socks", "Gloves", "Belts", "Bags", "Shoes", "Sunglasses", "Watches",
    "Jewelry", "Ties", "Hair Accessories", "Makeup", "Accessories",
];

struct Context {
    vars: Vars,
}

struct Vars {
    id: String,
    name: String,
    price: f64,
}

fn generate_product(context: &mut Context) {
    let mut rng = rand::thread_rng();

    let color = COLORS.choose(&mut rng).unwrap();
    let name = PRODUCTS.choose(&mut rng).unwrap();

    context.vars.id = Uuid::new_v4().to_string();
    context.vars.name = format!("{} {}", color, name);
    context.vars.price = rng.gen_range(0.0..=100.0);

    // Call done function or handle completion logic here
}

fn main() {
    let mut context = Context {
        vars: Vars {
            id: String::new(),
            name: String::new(),
            price: 0.0,
        },
    };

    for _ in 0..5 {
        generate_product(&mut context);
        println!("ID: {}", context.vars.id);
        println!("Name: {}", context.vars.name);
        println!("Price: {:.2}", context.vars.price);
        println!();
    }
}