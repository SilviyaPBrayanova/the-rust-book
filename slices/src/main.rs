use std::any::type_name;

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

    println!("Second word is {}", second_word(&String::from("What is the second word here?")));

    let sentence = String::from("This is some long sentence");
    let first_word = &sentence[0..4];
    let second_word = &sentence[5..7]; 
    println!("First and second word are {} and {}", first_word, second_word);   

    let arr = [1, 2, 3, 4, 5];
    let all_sls = &arr[1..3];
    assert_eq!(all_sls, &[2,3]);

    // Now the fun part ;)
    let s = "S String";
    let s1 = &s;
    let s2 = &s1;
    println!("{}", type_of(s));
    println!("{}", type_of(s1));
    println!("{}", type_of(s2));
    println!("s = {}, &s = {}, &&s = {}", s, s1, s2);
    
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

fn second_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    let mut count = 0;
    let mut start = 0;
    let mut end = s.len();
    for (pos, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if count == 0 {
                start = pos;
                count += 1;
            } else if count == 1{
                end = pos;
                break;
            }
        }
    }
    &s[start..end]
}

 fn type_of<T> (_: T) -> &'static str {
    type_name::<T>()
}