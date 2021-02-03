// make debug display available for the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods can take ownership of self, borrow self immutably
// as we’ve done here, or borrow self mutably
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // display debug :? or pretty debug :#?
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

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

// we want to borrow the struct rather than take ownership
// of it. This way, main retains its ownership and can
// continue using rect1, which is the reason we use the &
// in the function signature and where we call the function.
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
