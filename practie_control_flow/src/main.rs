fn main() {
    exercise_1();
    exercise_2();
}
fn exercise_1() {
    println!("Fahrenheit to Celsius converter");
    let f_temp = 100;
    let c_temp = fahrenheit_to_celsius(f_temp);
    println!("{}F equals {}C", f_temp, c_temp);
}

fn fahrenheit_to_celsius(f: i32) -> i32 {
    println!("fahrenheit_to_celsius: input {f}F");
    (f - 32) * 5 / 9
}

fn exercise_2() {
    println!("Fibonacci");
    let n = 10;
    println!("fibonacci_recursive, for n = {n}");
    let fr = fibonacci_recursive(n);
    //println!("fibonacci_iterative, for n = {n}");
    //let fi = fibonacci_iterative(n);
    //println!("fibonacci_recursive: {} and fibonacci_iterative: {} , for n = {}", fr, fi, n);
    println!("fibonacci_recursive: {fr}");
}

fn fibonacci_recursive(n: i32) -> i32 {
    if n <= 1 { return n }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}