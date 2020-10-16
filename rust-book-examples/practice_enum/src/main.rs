fn main() {
    println!("Hello, world!");
    create_ip_inst();
    setup_loopbacks();
    setup_loopbacks2();
    setup_loopbacks3();
    test_message();
    option_enum();

    let mut mycoin = Coin::Penny;
    value_in_cents(mycoin);
    mycoin = Coin::Quarter(UsState::Alaska);
    value_in_cents(mycoin);

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => (), //unit value
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        //only trigger on match
        println!("three");
    }

    let mut count = 0;
    mycoin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = mycoin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn create_ip_inst() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
}

fn setup_loopbacks() {
    let _loopbackv4 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let _loopbackv6 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

fn setup_loopbacks2() {
    let _loopbackv4 = IpAddr2::V4(String::from("127.0.0.1"));
    let _loopbackv6 = IpAddr2::V6(String::from("::1"));
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn setup_loopbacks3() {
    let _loopbackv4 = IpAddr3::V4(127, 0, 0, 1);
    let _loopbackv6 = IpAddr3::V6(String::from("::1"));
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message.call() method body goes here")
    }
}

fn test_message() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn option_enum() {
    let _some_number = Some(5);
    let _some_string = Some("a string"); //str slice!
    let _absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //let sum = x + y; //doesn't work!
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
