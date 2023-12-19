// definition of the module front_of_house
// module front_of_house is the parent module of hosting and serving
mod front_of_house {
    // bodyof the module inside {}

    // inner modules
    // hosting and serving are sibling modules
    // hosting and serving are child modules of front_house
    // we make module public for it to be callable from parents modules.
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// re-exporting : make it callable in that code scope as it has been defined here
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
}

mod customer {
    use crate::front_of_house::hosting as myname;

    pub fn eat_at_restaurant() {
        myname::add_to_waitlist();
    }
}

// part of our library public API
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //  won't compile
    // seasonal_fruit is private !
    // meal.seasonal_fruit = String::from("blueberries");
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        // private field
        seasonal_fruit: String,
    }

    // all the variants of a publid enum are public !
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        // public constructor
        // needed to instantiate the struct Breakfast (we can't set the private field from the parent module directly)
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // parent (relative path)
        super::deliver_order();
    }

    fn cook_order() {}
}
