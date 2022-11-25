fn main() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr1 {
        kind: IpAddrKind,
        address: String,
    }

    let home1 = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback1 = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", home1);
    println!("{:?}", loopback1);

    #[derive(Debug)]
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));

    println!("{:?}", home2);
    println!("{:?}", loopback2);

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("calling: {:?}", self);
        }
    }

    let m = Message::Write(String::from("hello"));
    let q = Message::Quit;
    let mo = Message::Move { x: 5, y: 6 };
    let cc = Message::ChangeColor(12, 13, 14);
    m.call();
    q.call();
    mo.call();
    cc.call();

    #[derive(Debug)]
    enum UsState {
        Alabama,
        California,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
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

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::California);
    let quarter2 = Coin::Quarter(UsState::Alabama);

    println!("a penny is {} cents", value_in_cents(penny));
    println!("a nickel is {} cents", value_in_cents(nickel));
    println!("a dime is {} cents", value_in_cents(dime));
    println!("a quarter is {} cents", value_in_cents(quarter));
    println!("a quarter is {} cents", value_in_cents(quarter2));

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    println!("x: {}", x);
    println!("y: {:?}", y);

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => (),
    }

    let some_u8_value2 = 3u8;
    match some_u8_value2 {
        1 => println!("one!!"),
        3 => println!("three!!"),
        _ => (),
    }

    let some_u8_value3 = Some(2u8);
    match some_u8_value3 {
        Some(3) => println!("three!!!!!!"),
        _ => (),
    }

    if let Some(2) = some_u8_value3 {
        println!("two!!!!!!")
    }

    if let Some(3) = some_u8_value3 {
        println!("three!!!!!!!!!!!!!")
    } else {
        println!("NOT THREE")
    }
}
