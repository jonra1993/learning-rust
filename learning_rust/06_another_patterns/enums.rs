/*
Enums allow you to define a type by enumerating its possible variants. 
*/

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// This sintax does not requiere a structure
#[derive(Debug)]

enum SimpleIpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum AnotherIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        dbg!(&self);
    }
}

fn main() {
    // The variants of the enum are namespaced under its identifier, and we use a double colon to separate the two
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //println!("user2 is {:?}", user2);
    dbg!(&home);



    let home = SimpleIpAddr::V4(String::from("127.0.0.1"));
    let loopback = SimpleIpAddr::V6(String::from("::1"));

    dbg!(&loopback);


    let home = AnotherIpAddr::V4(127, 0, 0, 1);
    let loopback = AnotherIpAddr::V6(String::from("::1"));

    dbg!(&home);

    let m = Message::Write(String::from("hello"));
    m.call();
}
