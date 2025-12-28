fn fix_incorrect_order() {
    cook_order();
    super::deliver_order(); // super imports from the crate::back_of_house::deliver_order();
}
fn cook_order() {}
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
impl Breakfast {
    pub fn summner(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}