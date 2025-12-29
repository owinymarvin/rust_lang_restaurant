// the front of house module, containing other modules, that then have functions in them
mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summner("rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // since the toast is pub in the struct, we can R/W from it.
    // for the seasonal fruit, it would give us errors asap.
    // meal.seasonal_fruit = String::from("blueberries"); //wont compile
}

fn deliver_order() {}
