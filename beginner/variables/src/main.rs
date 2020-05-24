use std::io;
use rand::Rng;

fn main() {

    // Variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // const MAX_POINTS: u32 = 100_000;

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // Data types

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}",&guess);

    let overflow: u8 = 255;

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;

    println!("{} {} {}", x, y, z);

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let y = {
        5 + 7
    };
    let y = rand::thread_rng().gen_range(1, 101);

    if y > 5 {
        println!("Greater than 5")
    } else if y == 5 {
        println!("Five on the money")
    } else {
        println!("Less than 5")
    }
    print(double(double(y)));

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..999).rev() {
        println!("{}", number)
    }
}

fn double(x: i32) -> i32 {
    x * 2
}

fn print(x: i32)  {
    println!("{}", x);
}