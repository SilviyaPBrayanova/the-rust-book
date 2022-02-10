// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

mod front_of_house;
mod back_of_house;

use back_of_house::back_of_house_funcs;
pub use crate::front_of_house::hosting;
pub use crate::front_of_house::serving;



pub fn eat_at_restaurant () {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house_funcs::Breakfast::summer(&String::from("Ray"));
    meal.toast = String::from("wheat");
    println!("Breakfast : {:?}", meal);
    let order1 = back_of_house_funcs::Appetizers::Salad;
    let order2 = back_of_house_funcs::Appetizers::Soup;

    println!("Appetizers: {:?} and {:?}", order1, order2);
}
