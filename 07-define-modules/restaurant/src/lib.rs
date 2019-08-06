#[cfg(test)]
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { }

        fn set_at_table() { }
    }

    pub mod serving {
        fn take_order() { }

        fn server_order() { }

        fn take_payment() { }

        pub mod back_of_house {

            pub enum Appetizer {
                Soup,
                Salad
            }

            pub struct Breakfast {
                pub toast: String,
                seasonal_fruit: String
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
                super::server_order();
            }

            fn cook_order() { }
        }
    }
}


pub fn eat_at_restaurant() {

    crate::front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::add_to_waitlist();

    let mut meal = crate::front_of_house::serving::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
