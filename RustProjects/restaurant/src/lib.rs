mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("You're added to the waitlist!");
        }

        fn _seat_at_table() {
            println!("This seat has already taken!");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("The order is taken!");
        }

        pub fn _serve_order() {
            println!("The order is served!");
        }

        fn _take_payment() {
            println!("The order has paid successfully!");
        }
    }
}

mod back_of_house {
    fn _fix_incorrect_order() {
        // cook_order();
        // Both are possible: through absolute path
        crate::front_of_house::serving::take_order();
        // or, through relative path
        // super::front_of_house::serving::serve_order();
    }

    fn _cook_order() {
        println!("The food is ready!");
    }
}

pub fn eat_at_restaurant() {
    // use absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // use relative path
    front_of_house::serving::take_order();
    // front_of_house::serving::serve_order();

}