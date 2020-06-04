
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

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
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 50,
    };

    println!("{:?}'s area is {}", rect1, rect1.area());
    println!("{:?}'s area is {}", rect2, rect2.area());
    println!("{:?}'s area is {}", rect3, rect3.area());
    println!(
        "Can {:?} hold {:?}? {}",
        rect1,
        rect2,
        rect1.can_hold(&rect2)
    );
    println!(
        "Can {:?} hold {:?}? {}",
        rect1,
        rect3,
        rect1.can_hold(&rect3)
    );
}
