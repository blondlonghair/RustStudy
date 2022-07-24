// enum IpAddrKind {
//     V4,
//     V6
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };



// enum IpAddrKind {
//     V4(String),
//     V6(String)
// }

// let home = IpAddr::V4(String::from("127.0.0.1"));

// let loopback = IpAddr::V6(String::from("::1"));



// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// let home = IpAddr::V4(127, 0, 0, 1);

// let loopback = IpAddr::V6(String::from("::1"));



// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }

// impl Message {
//     fn call(&self) {

//     }
// }

// let m = Message::Write(String::from("hello"));

// m.call();



// enum Option<T> {
//     Some(T),
//     None,
// }

// let some_number = Some(5);
// let some_string = Some("a string");

// let absent_number: Option<i32> = None;

// let x: i8 = 5;
// let y: Option<i8> = Some(5);

// let sum = x + y;



// #[derive(Debug)]
// enum UsState
// {
//     Alabama, Alaska,
// }

// enum Coin {
//     Penny,
//     Nickle,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => {
//             println!("행운의 페니!");
//             1
//         }
//         Coin::Nickle => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }



// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// let five = Some(5);
// let six = plus_one(five);
// let none = plus_one(None);



// let some_u8_value = 0u8;
// match some_u8_value {
//     1 => println!("one"),
//     3 => println!("three"),
//     5 => println!("five"),
//     7 => println!("seven"),
//     _ => (),
// }

// if let Some(3) = some_u8_value {
//     println!("three");
// }

// let mut count = 0;
// match coin {
//     Coin::Quarter(state) => println!("{:?}주의 25센트 동전!", state),
//     _ => count += 1,
// }

// if let Coin::Quarter(state) = coin {
//     println!("{:?}주의 25센트 동전!", state);
// } else {
//     count += 1;
// }

fn main() {
    
}
