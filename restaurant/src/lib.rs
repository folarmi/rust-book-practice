// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


mod front_of_house; 
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant () {
        // absolute path
        // crate::front_of_house::hosting::add_to_waitlist();

        // // relative path
        // front_of_house::hosting::add_to_waitlist();

        hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    // fn fix_incorrect_order() {
    //     cook_order();
    //     // super is like saying go to the root and start tracing from there
    //     super::deliver_order();
    // }

    // fn cook_order() {}
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast:&str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("agbalumo")
            }
        }
    }

}

pub fn order_out () {
    let mut meal = back_of_house::Breakfast::summer("Gizdodo");
}


