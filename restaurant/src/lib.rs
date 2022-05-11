// Load the contents of the module from another file
// with the same name as the module.
mod front_of_house;

fn serve_order() {
}

mod back_of_house {
  
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }

  fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
  }

  fn cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  // Absolute path
  hosting::add_to_waitlist();

  // Relative path
  hosting::add_to_waitlist();

  // Order a breakfast in the summer with Rye toast.
  let mut meal = back_of_house::Breakfast::summer("Rye");

  // Change our mind about the bread.
  meal.toast = String::from("Wheat");

  println!("I'd like {} toast please", meal.toast);

  // meal.seasonal_fruit = String::from("blueberries");
}
