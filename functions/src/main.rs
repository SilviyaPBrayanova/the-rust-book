fn main() {
    println!("Hello, world!");
    another_function();
    another_function_i32(32i32);
    print_labeled_measurement(5, 'h');
    println!("{}",five());
    println!("plus_one(5) = {}", plus_one(5));
    println!("plus_one(five()) = {}", plus_one(five()));
}

fn another_function() {
    println!("Another function");
}

fn another_function_i32(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i8 {
    5
}

fn plus_one(x:i8) -> i8 {
    x + 1
}