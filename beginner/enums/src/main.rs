enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("something")
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let quit_message = Message::Quit.call();

    let x: Option<String> = Some("test".to_string());
    assert_eq!(x.is_some(), true);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; How do we unwrap this?
}

