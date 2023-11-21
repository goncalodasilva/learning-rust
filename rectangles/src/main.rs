fn main() {
    with_sep_params();
    with_tuples();
    with_structs();
    methods();
}

fn with_sep_params() {
    println!("with_sep_params");
    let width1 = 30;
    let height1 = 50;
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1));
}
    
fn area(width: u32, height: u32) -> u32 {
    width * height
}
    
fn with_tuples() {
    println!("with_tuples");
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1));
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn with_structs() {
    println!("with_structs");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1));
    println!("rect1 is {:?}", rect1); // :? means use output format for debug
    println!("rect1 is {:#?}", rect1);

    dbg_macro();
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn dbg_macro() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg! (30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}

// Rectangle struct already defined
// Now defining its methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    // method wit more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn methods() {
    println!("Methods");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    if rect1.width() {
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }
    methods_with_more_parameters();
    associated_fns();
}

fn methods_with_more_parameters() {
    println!("Methods with more parameters");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

impl Rectangle {
    //second impl block for struct
    // no problem, it can have multiple
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn associated_fns() {
    println!("Associated functions are not methods, but are associated with the struct");
    let sq = Rectangle::square(5);
    println!(
        "The area of the rectangle is {} square pixels.",
        sq.area()
    );
}