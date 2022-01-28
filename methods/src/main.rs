fn main() {
    println!("Hello, world!");
    let r = Rectangle {
        height: 6,
        width: 8,
    };
    println!("Rectangle {:?}", r);
    println!("Area {}", r.area());

    let r1 = Rectangle {
        height: 5,
        width: 7,
    };

    
    let r2 = Rectangle {
        height: 4,
        width: 9,
    };

    println!("{:?} can hold {:?} is a {} statement", r, r1, r.can_hold(&r1));
    println!("{:?} can hold {:?} is a {} statement", r, r2, r.can_hold(&r2));
    println!("Square {:?}", Rectangle::square(76));
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    // The &self is actually short for self: &Self. 
    //Within an impl block, the type Self is an alias for the type that the impl block is for. 
    //We’ve chosen &self here for the same reason we used &Rectangle in the function version: 
    //we don’t want to take ownership, and we just want to read the data in the struct, not write to it. 
    //If we wanted to change the instance that we’ve called the method on as part of what the method does,
    //we’d use &mut self as the first parameter. 
    //Having a method that takes ownership of the instance by using just self as the first parameter is rare; 
    //echnique is usually used when the method transforms self into something else and you want
    // to prevent the caller from using the original instance after the transformation.
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(self: &Self, rect: &Rectangle) -> bool {
        self.height >= rect.height && self.width >= rect.width
    }

    fn square (side: u32) -> Rectangle {
        Rectangle {
            height: side,
            width: side,
        }
    }
}