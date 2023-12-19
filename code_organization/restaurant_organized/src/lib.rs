// load the front_house module
mod front_of_house;

// define a path for accessing an item in the front_house module (absolute)
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
