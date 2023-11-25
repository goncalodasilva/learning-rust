fn main() {
    enum_values();
    option_type();
    match_control_flow();
    concise_control_flow_w_if_let();
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
        Coin::Quarter(UsState::Alabama) => {
            println!("This is Alabama son");
            25
            
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}
fn match_control_flow() {
    let coin = Coin::Penny;
    value_in_cents(coin);
    let quarter: Coin = Coin::Quarter((UsState::Canada));
    value_in_cents(quarter);
    let alabama_quarter: Coin = Coin::Quarter((UsState::Alabama));
    value_in_cents(alabama_quarter);

    println!("========= Matching Option<T> =========");
    let six = Some(6);
    let seven = plus_one(six);
    let eigth = plus_one(seven);
    let none = plus_one(None);
    println!("========= Catch-all patterns =========");
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // any other value than 3 or 7 is catched and binded to a new var: other 
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
        // _ stands for a placeholder for anyother value when we don't need to bind it to a var
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
        // we can also use the unit value () when we don't want anything to happen
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(x: i8) {}
fn reroll() {}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(7) => {
            println!("You hit the lucky 7");
            Some(8)
        }
        Some(i) => Some(i + 1),
    }
}

fn concise_control_flow_w_if_let() {
    println!("========= Concise Control Flow with if let =========");
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // with if let you don't need the catch-all placeholder
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // we can also use the else to cover the remaining scenarios
    // jsut like we had
    let mut count = 0;
    let quarter = Coin::Quarter(UsState::Alabama);
    match quarter {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }
    // we have to instantiate again, because we moved the value before
    let quarter = Coin::Quarter(UsState::Alabama);
    // we can do the same with
    if let Coin::Quarter(state) = quarter {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}