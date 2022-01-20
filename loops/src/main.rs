fn main() {
    let mut counter: u8 = 1;
    loop {
        if counter < 5 {      
            println!("again!");
            counter = counter + 1;
        } else {
            break;
        }        
    }   
    
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);


    ///////////////////// RETURNING VALUE FROM LOOP /////////////////////
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // the value of the expression after the "break" keyword will be assigned to result
            break counter * 2; 
        }
    };

    println!("The result is {}", result);

    ///////////////////// WHILE LOOP /////////////////////
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    ///////////////////// FOR LOOP /////////////////////
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!("///////////// Fahrenheit to Celcius /////////////////");
    for g in [12f32, 23f32, 32f32, 45f32, 98f32, 120f32] {
        println!("{}F is {}", g, fahr_to_celcius(g));
    }    

    println!("///////////// Fibonacci /////////////////");
    for n in 0..10 {
        println!("{} Fib = {}", n, nth_fib(n));
    }
}

fn fahr_to_celcius(f: f32) -> f32 {
    return (f - 32f32)*5f32/9f32;
}

fn nth_fib(n: u8) -> u128 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2{
        return 1;
    }

    return nth_fib(n-1) + nth_fib(n-2);
}