fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    
    println!("{}", s);
    
    two_variables_same_string();
    two_variables_same_string_w_literals();
    cloning();
    ownership_w_functions();
    references_and_borrowing();
    str_slices();
    general_slices();
}

fn two_variables_same_string() {
    let s1 = String::from("hello");
    let s2 = s1;
    
    // can't do it           println!("{}", s1);
    println!("{}", s2);
}
fn two_variables_same_string_w_literals() {
    let s1 = "hello";
    let s2 = s1;
    
    // still can't do it     println!("{}", s1);
    println!("{}", s2);
}
fn cloning() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    println!("s1={}, s2={}", s1, s2);
}

fn ownership_w_functions() {
    let s = String::from("hello");  // s comes into scope
    
    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope
    
    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward


    retern_values_transfer_ownership();
    return_values_using_tuple();

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn retern_values_transfer_ownership() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{s1}");
    //println!("{s2}");     ownership is moved to fn and ret to s3
    println!("{s3}");
}

fn gives_ownership() -> String {

    let some_string = String::from("yours");

    some_string

}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn return_values_using_tuple() {
    let s1 = String::from("hello");
    let (s1, len) = calculate_len(s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn references_and_borrowing() {
    let s1 = String::from("hello world by ref");
    let len = calculate_length_by_ref(&s1);
    println!("The length of '{}' is {}.", s1, len);

    mutating();
    two_refs_to_same_value();
    dangling_references();
}

fn calculate_length_by_ref(s: &String) -> usize {
    s.len()
}

fn mutating() {
    let mut s = String::from("hello");
    println!("Before mutating s={s}");
    change(&mut s);
    println!("After mutating s={s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn two_refs_to_same_value() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;  cannot borrow as mut as it was already borrowed (eventhough it was borrowed as inmut)

    println!("{}, {}", r1, r2);
    
    refs_scope();
}

fn refs_scope() {
    let mut s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    
    println!("{}, {}", r1, r2);
    // r1 and r2 scope ends at their last usage
    let r3 = &mut s;
    r3.push_str("fucking world");
    println!("{}", r3);
    
}

fn dangling_references() {
    // let reference_to_nothing = dangle(); in dangle we are returning a ref to a value that went out of scope

    let ref_to_smth = no_dangle();
    println!("{}", ref_to_smth);
}
/*
fn dangle() -> &String {
    let s = String::from("hello");
    
    &s
}
*/
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn str_slices() {
    let mut s = String::from("merry christmas motherf****");
    let w1 = first_word(&s); // return a usize
    
    s.clear(); // this empties the String
    
    //w1 still contains 5 but the value of s has changed
    // The solution Rust offers are slices
    
    let s = String::from("merry christmas motherf****");

    let merry = &s[..5];
    let christmas = &s[6..15];
    let rest = &s[15..];
    let all = &s[..];
    println!("{} {} {}", merry, christmas, rest);
    println!{"{}", all};

    // Atention: ONLY for UTF-8 characters

    let w1 = first_word_2(&s);
    println!("{}", w1);
}

fn first_word(s: &String) -> usize {
    //returns the index of the byte of first space
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
fn first_word_2(s: &str) -> &str {
    // &str is a immutable reference, that could 
    // point to litterals as well. 
    // &str is the type of the slices themselves
    // Having it as a parameter makes the API 
    // more general and useful, without losing
    // any functionality
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn general_slices() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}