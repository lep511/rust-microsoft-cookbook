pub struct Pizza {
    pub topping: String,
    pub inches: u8,
}

impl Pizza {
    pub fn pepperoni(inches: u8) -> Self {
        Pizza::bake("pepperoni", inches)
    }

    pub fn mozzarella(inches: u8) -> Self {
        Pizza::bake("mozzarella", inches)
    }

    fn bake(topping: &str, inches: u8) -> Self {
        Pizza {
            topping: String::from(topping),
            inches,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pizza = Pizza::pepperoni(12);
        assert_eq!(pizza.topping, "pepperoni");
        assert_eq!(pizza.inches, 12);
        let pizza = Pizza::mozzarella(12);
        assert_eq!(pizza.topping, "mozzarella");
        assert_eq!(pizza.inches, 12);
        let pizza = Pizza::bake("pepperoni", 12);
        assert_eq!(pizza.topping, "pepperoni");
        assert_eq!(pizza.inches, 12);
    }
}
