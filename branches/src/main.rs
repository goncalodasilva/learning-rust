fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    sencond_main();
}

fn sencond_main() {
    println!("Second main");

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}