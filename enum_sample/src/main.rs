fn main() {

    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    route(four);

    route(six);
    route(IpAddressKind::V4);

    let home = IpAddress {
        address: String::from("127.0.0.1"),
        kind: IpAddressKind::V4,
    };

    let loop_back = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1")
    };

    let home2 = IpAddressKind2::V4(String::from("127.0.0.1"));
    let loop_back_2 = IpAddressKind2::V6(String::from("::1"));

    let home3 = IpAddressKind3::V4(127, 0, 0, 1);
    let loop_back_3 = IpAddressKind3::V6(String::from("::1"));
    let _move = Message::Move {
        x: 10,
        y: 20
    };

    _move.call();

    let i = 5;
    let m = Option::Some(5);
    let m1: Option<i32> = Option::Some(5);
}


enum IpAddressKind {
    V4,
    V6
}

enum IpAddressKind2 {
    V4(String),
    V6(String)
}

enum IpAddressKind3 {
    V4(u8,u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32
    },
    Write(String),
    ChangeColor(i32, i32, i32)
}

struct IpAddress {
   kind: IpAddressKind,
   address: String
}


impl Message {
    fn call(&self) {
        match self {
            Message::Write(text) => {
                println!("Writing: {}", text);
            },
            Message::ChangeColor(x,y,z)=> {
                println!("Values, {}, {}, {}", x,y,z);
            },
            Message::Move { x, y } => {
                println!("moving")
            },
            _ => {

            }
        }
        println!("call...");
    }
}


enum  Option<T> {
    Some(T),
    None
}

fn route(kind: IpAddressKind) {
    match kind  {
        IpAddressKind::V4 => {
            println!("is IP v4");
        },
        IpAddressKind::V6 => {
            println!("is IP v6");
        }
    }
}