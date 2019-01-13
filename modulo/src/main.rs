mod sound {

    fn breathe_in() {
        println!("Breathing....");
    }

    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                println!("Clariente");

                super::super::breathe_in();
            }
        }
    }

    mod voice {
    }
}


mod plant {

    pub  struct Vegatable {
        pub name: String,
        id: i32
    }

    static mut id: i32 = 0;

    impl Vegatable {
        pub fn new(name: &str) -> Vegatable {
            super::id += 1;
            Vegatable {
                name: String::from(name),
                id: super::id,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad
    }
}

mod performance_group {
    use crate::sound::instrument::woodwind;

    pub fn clarinet_trio() {
        woodwind::clarinet();
        woodwind::clarinet();
        woodwind::clarinet();
    }
}

fn main() {
    println!("Hello, world!");

    crate::sound::instrument::woodwind::clarinet();

    sound::instrument::woodwind::clarinet();

    performance_group::clarinet_trio();

    let mut v = plant::Vegatable::new("squash");

    v.name = String::from("butternut squash");

    println!("{} are delicious", v.name);
}
