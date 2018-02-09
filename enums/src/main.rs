// enum with struct

// enum IpAddrKind {
//     V4,
//     V6,
// }
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }


// putting data into each enum variant
// no need struct

// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }
// let home = IpAddrKind::V4(String.from("127.0.0.1"));


// each variant can have different types and amounts of associated data.
// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
// let home = IpAddrKind::V4(127, 0, 0, 1);
// let loopback = IpAddrKind::V6(":::1".to_String());

// embeds the address data type insisde the variants
struct IpV4Addr{
    // detail
}
struct IpV6Addr{
    // detail elided
}
enum IpAddr {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

// constructor as a function
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct QuitMessage;
struct MoveMessage {x: i32, y: i32}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
// let m = Message::Write(String::from("hello"));
// m.call();

fn main() {
    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
}