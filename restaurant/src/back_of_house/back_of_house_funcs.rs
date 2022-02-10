
fn fix_incorrect_order() {
    cook_order();
   // super::front_of_house::serving::serve_order();
}
fn cook_order() {}

#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruits: String,
}

impl Breakfast {
    pub fn summer(toast: &String) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruits: String::from("peaches"),
        }
    }
}

#[derive(Debug)]
pub enum Appetizers {
    Soup,
    Salad,
}
