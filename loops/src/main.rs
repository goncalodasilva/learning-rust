fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    second_main();
    third_main();
    forth_main();
    fifth_main();
    six_main();
}

fn second_main() {
    println!("Second main");
    
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    
    println!("End count = {count}");
}

fn third_main() {
    println!("Third main");
    
    let mut number = 3;
    
    while number != 0 {
        println!("{number}!");
        
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn forth_main() {
    println!("Forth main");
    
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    
    while index < 5 {
        println!("thevalue is: {}", a[index]);
        index += 1;
    }
}

fn fifth_main() {
    println!("Forth main");
    
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {element}");
    }
}

fn six_main() {
    println!("Sixth main");
    
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}