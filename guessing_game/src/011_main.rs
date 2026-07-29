use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn route(ip_kind: IpAddrKind) {}
enum Message {
    Ouit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let loopback = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    let some_number = Option::Some(5);
    let some_char = Option::Some('e');

    let absent_number: Option<i32> = Option::None;
}
