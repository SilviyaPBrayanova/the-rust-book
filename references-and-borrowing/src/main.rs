fn main() {
    let s = String::from("Str ref");
    println!("\"{}\" size is {}", s, calculate_length(&s));
    let mut t = String::from("");
    change(&mut t);
    println!("Instance mutated by a function {}", t);
    
    /*
    Mutable references have one big restriction: 
    you can have only one mutable reference to a particular piece of data at a time. 
    This code that attempts to create two mutable references to s will fail:
    
    let t1 = &mut t;
    let t2 = &mut t;
    println!("{}{}", t1, t2);
    */

    let t1 = &t;
    let t2 = &t;
    // let t3 = &mut t; // this does not
    //println!("{}{}{}", t1, t2, t3);
    println!("{}{}", t1, t2);
    /*
    The scopes of the immutable references r1 and r2 end after the println! where they are last used, 
    which is before the mutable reference r3 is created. 
    These scopes don’t overlap, so this code is allowed. 
    The ability of the compiler to tell that a reference is no longer being used 
    at a point before the end of the scope is called Non-Lexical Lifetimes (NLL for short) 
    */
    let t3 = &mut t;
    println!("{}", t3);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.\

/* This will NOT compile.
fn change(some_string: &String) {
    some_string.push_str(", world");
}
*/

// References must be declared as mutable if we are intending to mutate them 
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
  /* From: The BOOK
   * We call the action of creating a reference borrowing. 
   * As in real life, if a person owns something, you can borrow it from them. 
   * When you’re done, you have to give it back. You don’t own it.
   */