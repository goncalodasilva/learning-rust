fn main() {
    println!("Hello, world!");
    
    another_function(5);
    
    second_main();
    third_main();
}

fn another_function(x: i8) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn second_main() {
    println!("second_main");
    print_labeled_measurement(5, 'h');
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn third_main() {
    println!("third_main");
    let x = five();

    println!("The value of x is: {x}");
    
    let x = plus_one(5);
    println!("The value of x is: {x}");

}