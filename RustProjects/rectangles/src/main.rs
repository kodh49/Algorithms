fn main() {
    let shape = Rectangle {
        height: 32,
        width: 64,
    };
    let geometry = Rectangle {
        height: 30,
        width: 43,
    };
    let square = Rectangle::square(35);
    println!("Shape:{:?} | Area: {} | Perimeter: {}", shape, get_area(&shape), shape.perimeter());
    println!("Shape:{:?} | Area: {} | Perimeter: {}", square, get_area(&square), square.perimeter());
    println!("Can shape hold geometry? : {}", shape.can_hold(&geometry));
}

fn get_area(shape:&Rectangle) -> u64 {
    shape.height * shape.width
}

#[derive(Debug)]
struct Rectangle {
    height: u64,
    width: u64,
}

impl Rectangle {
    // Methods : Just like instance methods in Java, use &self in order to refer to the destined object
    fn perimeter(&self) -> u64 {
        2*(self.width+self.height)
    }
    // Associated Functions : Just like static methods in Java. Does not need &self because it should run without an instance
    fn square(length:u64) -> Rectangle {
        Rectangle {
            height: length,
            width: length,
        }
    }
    fn can_hold(&self, other:&Rectangle) -> bool {
        (self.width >= other.width) && (self.height >= other.height)
    }
}