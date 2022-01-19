use std::num::Wrapping;

fn main() {
    ///////////// INREGERS /////////////
    let the_answer: u32 = "42".parse().expect("This may not work"); 
    println!("The Earth, the Universe and everything else is {}", the_answer);

    let int: u8 = 10;
    println!("A u8 integer {}", int);

    let postfixed_type = 57u8;
    println!("let postfixed_type = 57u8; resolves to {}", postfixed_type);

    /*
    let int_overflow: u8 = 300; // this will overflow and cause a panic!
    */

    // Now this is how wrapping is intentionally used
    let mut wrapped_u8 = Wrapping(128u8);
    wrapped_u8 = wrapped_u8 + wrapped_u8;
    println!("Wrapped u8 {}", wrapped_u8);

    println!("128u8.wrapping_add(128u8) = {}",128u8.wrapping_add(128u8));
   
    ///////////// CHARACTERS /////////////
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
 
    println!("z = {}, Z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);

    ///////////// TUPLES /////////////
    let tup: (i32, f64,u8) = (-500, 3.14, 7);
    let (x, y, z) = tup;
    println!("x={}, y={}, z={}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred={}, six_point_four={}, one={}", five_hundred, six_point_four, one);

    ///////////// ARRAYS /////////////
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("Number on position 4 in array {}", a[4]);
    println!("My month {}", months[10]);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    let a = [3; 5];
    // same as 
    let a = [3, 3, 3, 3, 3];
}
