use std::fmt;

enum IpAddrKind {
    V4,
    V6
}


#[derive(Debug)]
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Messages {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}


struct IpAddr {
    kind: IpAddrKind,
    address: String
}

impl Messages {
    fn call(&self) {

    }
}


impl IpAddrKind {
    fn to_string(&self) -> String {
        match self {
            IpAddrKind::V4 => String::from("V4"),
            IpAddrKind::V6 => String::from("V6"),
        }
    }
}

impl IpAddrKind2 {
    fn to_string(&self) -> String {
        match self {
            IpAddrKind2::V4(x, y, z, w) =>  format!("is address {}.{}.{}.{} and IP Kind {}", x, y, z, w, String::from("V4")),
            IpAddrKind2::V6(address) => format!("is address {} and IP Kind {}", address, String::from("V6")),
        }
    }
}


fn main() {

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loop_back = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from(":1")
    };
    
    println!("home is address {} and IP Kind {}", home.address, home.kind.to_string());
    println!("loop_back is address {} and IP Kind {}", loop_back.address, loop_back.kind.to_string());

    let home2 = IpAddrKind2::V4(127,0,0,1);
    let loop_back2 = IpAddrKind2::V6(String::from(":1"));

    println!("home2 is {}", home2.to_string());
    println!("loop_back2 is {}", loop_back2.to_string());
    
    let m = Messages::Write(String::from("hello"));
    m.call();


    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;


    if let Some(3) = some_number {
        println!("three");
    }

    let coin = Coin::Dime;
    value_in_coin(coin);

    /*  */let mut count = 0
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}