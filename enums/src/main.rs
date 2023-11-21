fn main() {
    enum_values();
    option_type();
    match_control_flow();
}

fn enum_values() {
    println!("enum values");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    //let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
enum IpAddrKind {
    V4,
    V6,
}
fn route(ip_kind: IpAddrKind) {}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn option_type() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; x and y are different types
    // println!("sum: {}", sum);  
    println!("x and y are different types");
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Canada,
    California
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
            print!("State quarter from {:?}", state);
            25
        },
    }
}
fn match_control_flow() {
    let coin = Coin::Penny;
    value_in_cents(coin);
    let quarter: Coin = Coin::Quarter((UsState::Canada));
    value_in_cents(quarter);
}