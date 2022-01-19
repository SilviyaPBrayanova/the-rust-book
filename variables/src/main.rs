fn main() {

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner scope x: {}", x);
    }
    println!("Outer scope x: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("{} spaces", spaces);

    /* The following two lines do not compile
        let mut spaces = "   ";
        spaces = spaces.len();
    */
}
