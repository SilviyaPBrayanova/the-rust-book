fn main() {
    println!("{}",first_word(&String::from("Hello, world!")));

    println!("{}", first_word_slice(&String::from("A whole new world")));
    println!("{}", first_word_slice(&String::from("A_whole_new_world")));

    let mut important = String::from("Bear neccesities");
    let word = first_word_slice(&important);
    println!("{}", word);
    important.clear();
    // println!("{}", sliced); this will not compile

    /* Recall from the borrowing rules that if we have an immutable reference to something, 
    we cannot also take a mutable reference. Because clear needs to truncate the String, 
    it needs to get a mutable reference. 
    The println! after the call to clear uses the reference in word, so the immutable reference 
    must still be active at that point. Rust disallows the mutable reference in clear 
    and the immutable reference in word from existing at the same time, and compilation fails. 
    Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors 
    at compile time!
    */

    println!("{}", first_word_slice_slice("s: &str"));
    println!("{}", first_word_slice_slice(&String::from("s: &str")));
}

fn first_word( s: &String) -> usize{
    let bytes = s.as_bytes();
    for (pos, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return pos;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (pos, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..pos];
        }
    }
    s // same as &s[..]
} 


fn first_word_slice_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (pos, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..pos];
        }
    }
    s // same as &s[..]
} 