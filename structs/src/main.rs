    fn main() {
    println!("Hello, world!");
    let mut user = User{
        active: true,
        username: String::from("Somebody Important"),
        email: String::from("somebody@example.com"),
        sign_in_count: 1,
    };
    user.email = String::from("some.body@example.com");
    
    let user = create_user(String::from("Silver"), String::from("silver@silver.org"));
    println!("{} {} {} {}", user.username, user.email, user.active, user.sign_in_count); 

    let user2 = User {
        email: String::from("silver.john@example.com"),
        ..user
    };

    println!("{} {} {} {}", user2.username, user2.email, user2.active, user2.sign_in_count); 

    struct Point(i32, i32,i32);
    struct Color(i32, i32, i32);
    let p1 = Point(123, 234, 345);
    let c1 = Color(34, 56, 78); 
    println!("Point({},{},{})", p1.0, p1.1, p1.2);
    println!("Color({},{},{})", c1.0, c1.1, c1.2);

    let h = 5;
    let w = 6;
    println!("Area of {} by {} = {}",  w, h, area(w, h));

    println!("Area of ({},{}) = {}", 4, 7, area_tuple((4, 7)));

    let d = Dimensions {
        width: 7,
        height: 9,
    };
    
    println!("Area of ({},{}) = {}", d.height, d.width, area_struct(&d));
    println!("Area of ({},{}) = {}", d.height, d.width, area_struct(&d));
    println!("Area of ({},{}) = {:?}", d.height, d.width, d);
    println!("Area of ({},{}) = {:#?}", d.height, d.width, d);
    dbg!(d);

    let r = Rectangle {
        w: dbg!(8u32),
        h: 9,
    };

    dbg!(&r);
}

#[derive(Debug)] // outer attribute that provides default Debug formatter
struct Dimensions {
    width: u32,
    height: u32,

}

#[derive(Debug)]
struct Rectangle  {
    w: u32,
    h: u32,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,                                                                 
}

fn create_user(username: String, email: String) -> User {
    User {        
        username,
        email,
        //username: username,
        //email: email,
        sign_in_count: 0,
        active: true,
    }
}

fn area(width: u32, heigth: u32) -> u32 {
    width * heigth
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(dimensions: &Dimensions) -> u32 {
    dimensions.height * dimensions.width
}