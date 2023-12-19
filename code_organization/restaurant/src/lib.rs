// definition of the module front_of_house
// module front_of_house is the parent module of hosting and serving
mod front_of_house {
    // bodyof the module inside {}

    // inner modules
    // hosting and serving are sibling modules
    // hosting and serving are child modules of front_house
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
