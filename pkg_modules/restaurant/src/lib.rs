mod front_of_house;

// [ TITLE ] Modules access modifier

// Items in a parent module can’t use the private items inside child modules
// but items in child modules can use the items in their ancestor modules

fn deliver_order() {}

mod back_of_house {

    // if we make an enum public, all of its variants are then public.
    // We only need the pub before the enum keyword,
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // access modifier for struct and its fields are seprated.
    // public struct
    pub struct Breakfast {
        pub toast: String,      // public field
        seasonal_fruit: String, // private field
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
        // use super keyword to refers to deliver_order parent module's function .
        super::deliver_order();
    }

    fn cook_order() {}
}

// use crate::front_of_house::hosting;
// Before this change, external code would have to call the add_to_waitlist function 
// by using the path restaurant::front_of_house::hosting::add_to_waitlist(),
// which also would have required the front_of_house module to be marked as pub.
// Now that this pub use has re-exported the hosting module from the root module,
// external code can use the path restaurant::hosting::add_to_waitlist() instead.
pub use crate::front_of_house::hosting;

mod customer {
    
    pub fn eat_at_restaurant() {
        // abs path
        // crate::front_of_house::hosting::add_to_waitlist();
        // because of use at top of this module.
        add_to_waitlist();

        // relate path
        front_of_house::hosting::add_to_waitlist();
        
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String.from("Wheat");
        println!("I'd like {} toast please", meal.toast);
        
        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;

    }
}

// [ TITLE ] Providing New Names with the as Keyword
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}