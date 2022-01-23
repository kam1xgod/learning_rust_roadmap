// Any IP address can be either a version four or a version six address,
// but not both at the same time. That property of IP addresses makes the
// enum data structure appropriate, because enum values can only be one of its variants.
// Both version four and version six addresses are still fundamentally IP addresses,
// so they should be treated as the same type when the code is handling
// situations that apply to any kind of IP address.

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr_dif {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // no data associated.
    Move { x: i32, y: i32 },    // has named fields. like struct.
    Write(String),              // single String.
    ChangeColor(i32, i32, i32), // three i32 values.
}

// we can create structs for all enum's values.

struct QuitMessage; // unit.
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct.
struct ChangeColorMessage(i32, i32, i32); // tuple struct.

// but, like structs, enums also have impl and call method.

impl Message {
    fn call(&self) {
        // method body should be defined here.
    }
}

//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    // We can do the thing below because V4 and V6 are the same type.
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // that's a "normal" way.
    //    let home = IpAddr {
    //	kind: IpAddrKind::V4,
    //	address: String::from("127.0.0.1"),
    //    };

    //    let loopback = IpAddr {
    //	kind: IpAddrKind::V6,
    //	address: String::from("::1"),
    //    };

    // best practice here would be:
    // 1. change Enum, so it can store associated value.
    // 2.
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));
    // the name of each enum variant also becomes a function.

    let _home = IpAddr_dif::V4(127, 0, 0, 1);
    let _loopback = IpAddr_dif::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();
}

fn route(_ip_kind: IpAddrKind) {}
