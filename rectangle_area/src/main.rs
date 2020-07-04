struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.height > other_rect.height && self.width > other_rect.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 10,
    };

    println!("The area of the rectangle is {} square pixels.", &rect1.area());

    println!("rect1 can hold rect2: {}", &rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", &rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(25);
    println!("The area of the rectangle is {} square pixels.", &rect4.area());
}

