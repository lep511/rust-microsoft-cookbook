// Los módulos nos permiten organizar el código dentro de una crate para facilitar la lectura y la reutilización. 
// Los módulos también nos permiten controlar la privacidad de los elementos porque el código dentro de un módulo 
// es privado por defecto. Los elementos privados son detalles de implementación interna que no están disponibles para uso externo. 
// Podemos elegir hacer públicos los módulos y los elementos que contienen, 
// lo que los expone para permitir que código externo los utilice y dependa de ellos.

// Como ejemplo, vamos a escribir una biblioteca que proporciona la funcionalidad de un restaurante. 
// Definiremos las firmas de las funciones pero dejaremos sus cuerpos vacíos para concentrarnos 
// en la organización del código más que en la implementación de un restaurante.

// En la industria de la hostelería, algunas partes de un restaurante se denominan front of house y 
// otras back of house. Front of house es donde están los clientes; 
// abarca el lugar donde los anfitriones ubican a los clientes, los camareros toman los pedidos y el pago, 
// y los camareros preparan las bebidas. 
// Back of house es donde los chefs y cocineros trabajan en la cocina, 
// los lavaplatos limpian y los gerentes hacen el trabajo administrativo.

// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

pub mod front_of_house {
    pub mod hosting {
        // Absolute path
        // use crate::front_of_house::serving::{take_order, serve_order, take_payment};

        // Relative path
        use super::serving::{take_order, serve_order, take_payment};
        
        pub fn add_to_waitlist(from_calling: String) {


            println!("Add to waitlist from {}...", from_calling);
            seat_at_table(String::from("add_to_waitlist"));
            take_order(String::from("add_to_waitlist"));
            serve_order(String::from("add_to_waitlist"));
            take_payment(String::from("add_to_waitlist"));
        }

        pub fn seat_at_table(from_calling: String) {
            println!("Seat at table from {}...", from_calling);
            
        }
    }

    mod serving {
        pub fn take_order(from_calling: String) {
            println!("Take a order from {}...", from_calling);
        }

        pub fn serve_order(from_calling: String) {
            println!("Serve order from {}...", from_calling);
        }

        pub fn take_payment(from_calling: String) {
            println!("Take payment from {}...", from_calling);
        }
    }
      
}

mod back_of_house {
    pub fn fix_incorrect_order(from_calling: String) {
        println!("Fix incorrect order from {}...", from_calling);
    }

    fn cook_order(from_calling: String) {
        println!("Cook order from {}...", from_calling);
    }
}


