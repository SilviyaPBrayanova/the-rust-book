use std::io;

fn main() {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Input error");

    let number: i32  = buf.trim().parse().expect("");

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0{
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0{
        println!("{} is divisible by 3", number);
    } else if number % 2 == 0{
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3 or 2", number);
    }

    let condition = true;
    let var = if condition {5} else {6};
    println!("{}", var);
}
