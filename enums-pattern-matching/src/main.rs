use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddress {
        address: String::from("127.0.0.1"),
        ip_kind: IpAddrKind::V4,
    };

    let loopback = IpAddress {
        address: String::from("::1"),
        ip_kind: IpAddrKind::V6
    };

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    let home_ip = Ip::V4(String::from("127.0.0.1"));
    let loopback_ip = Ip::V6(String::from("::1"));

    println!("Home Ip : {:?}", home_ip);
    println!("Home Ip : {:?}", loopback_ip);

    let yet_another_home = YetAnotherIp::V4(127, 0, 0, 1);
    let yet_another_loopback =  YetAnotherIp::V6(String::from("::1"));

    println!("Yet another home : {:?}", yet_another_home);
    println!("Yet another loopback : {:?}", yet_another_loopback);

    let std_home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let std_loopback = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    println!("Standard ip v4 : {}", std_home);
    println!("Standard ip v6 : {}", std_loopback);

    let msg_move = Message::Move{x: 23, y: 45};
    let msg_quit = Message::Quit;
    let msg_write = Message::Write(String::from("write message"));
    let msg_change_color = Message::ChangeColor(0, 0, 0);
    msg_move.call();
    msg_change_color.call();
    msg_write.call();
    msg_quit.call();

    ////////// Option<T> //////////
    let some_number = Some(7);
    let no_number: Option<u32> = None;

    println!("Some {:?}, Unwrapped {}, None {:?}", some_number, some_number.unwrap(), no_number);
    println!("{}", value_in_cents(&Coin::Quarter(UsState::Alaska)));

    let x = Some(5);
    let none: Option<u32> = None;
    println!("{:?}", plus_one(x));
    println!("{:?}", plus_one(none));

    let dice_roll = 9;
    match dice_roll {
        3 => println!("Nice hat on"),
        7 => println!("Nice hat off"),
        other => println!("Move {}", other),
    }

    match dice_roll {
        3 => println!("Nice hat on"),
        7 => println!("Nice hat off"),
        _ => println!("Reroll"), // we take some action but do not intend to use the value
    }

    match dice_roll {
        3 => println!("Nice hat on"),
        7 => println!("Nice hat off"),
        _ => (), // we take no actions 
    }


    /////////////////// if let ///////////////////
    let config_max = Some(43u8);
    if let Some(max) = config_max {
        println!("{}", max);
    }

    let mut count = 0;
    let coin1 = Coin::Quarter(UsState::Alaska);
    let coin2 = Coin::Nickel;
    
    if let Coin::Quarter(state) = &coin1 {
        println!("Quarter from {:?}", state);
    } else {
        count += 1;
    }

    if let Coin::Quarter(state1) = &coin2 {
        println!("Quarter from {:?}", state1);
    } else {
        count += 1;
    }
    println!("count : {}", count);

    count = coins(&coin1, count);
    count = coins(&coin2, count);
    println!("count : {}", count);
}

fn coins(coin: &Coin, count:  u8) -> u8 {
    if let Coin::Quarter(state) = coin {
        println!("Quarter from {:?}", state);
    } else {
        return count + 1;
    }
    count
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), 
    }
} 

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State {:?}",  state);
            25},
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddress {
    ip_kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Ip {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum YetAnotherIp {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}",  ip_kind);
}


#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// equivalent to

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// the advantage of the enum is that we have a single type that holds all types of messages
// enums also can have methods

impl Message {

    fn call(&self) {
        println!("{:?}", self);
    } 
}