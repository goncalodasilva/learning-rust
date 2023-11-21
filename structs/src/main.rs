fn main() {
    println!("Hello, world!");
    structs();
    tuple_structs();
    unit_like_structs();
    structs_non_owners();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

fn structs() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1,
    };
    println!("User1 = {}, with email {}", user1.username, user1.email);
    
    //user1.email = String::from("anotheremail@example.com");
    let mut user1 = user1;  // needs to be mut to be able to change any field
    user1.email = String::from("anotheremail@example.com");
    println!("User1 = {}, with email {}", user1.username, user1.email);
    
    let email = String::from("user2@example");
    let username = String::from("user2name");
    let user2 = build_user(email, username);
    println!("User2 = {}, with email {}", user2.username, user2.email);
    
    let email = String::from("user3@example");
    let username = String::from("user3name");
    let user3 = build_user_field_init_shorthand(email, username);
    println!("User3 = {}, with email {}", user3.username, user3.email);
    
    // creating instances from other instances
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("this one is different"),
        sing_in_count: user1.sing_in_count,
    };
    println!("User2 = {}, with email {}", user2.username, user2.email);
    // println!("And again... User1 = {}, with email {}", user1.username, user1.email);
    // since user1 was moved when creating this last user2
    println!("User3 active:{}, username:{}, email:{}, sign:{}", 
        user3.active, 
        user3.username, 
        user3.email, 
        user3.sing_in_count);
    let user2 = User {
        email: String::from("another@example.com"),
        ..user3
    };
    println!("User2 active:{}, username:{}, email:{}, sign:{}", 
        user2.active, 
        user2.username, 
        user2.email, 
        user2.sing_in_count);
        
    /*
    println!("User3 active:{}, username:{}, email:{}, sign:{}", 
        user3.active, 
        user3.username, 
        user3.email, 
        user3.sing_in_count);
    same here, user3 is moved when building user2
     */
        
    }
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sing_in_count: 1,
    }
}

fn build_user_field_init_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sing_in_count: 0,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let other_color = Color(32, 45, 23); // access by index
    println!("other_color has values {}, {}, {}", other_color.0, other_color.1, other_color.2);

    let random_point = Point(423, 532, 23);
    let Point(x, y, z) = random_point; // destructuring
    println!("random_point has coordinates {} {} {}", x, y, z);
}

struct AlwaysEqual; // useful when you want a type just to impl traits
fn unit_like_structs() {
    let subject = AlwaysEqual;
}
/*
struct UserNonOwner {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
*/


fn structs_non_owners() {

    /*
    let user1 = UserNonOwner {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count:1
    };
    */
    
    
    // for structs to hold only 
    // references to the values, 
    // we would need to introduce lifetimes
}
