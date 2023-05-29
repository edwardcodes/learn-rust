// ------------------------------------------------------------------
// Conditions and Control Flow
// -----------------------------------------------------------------

use std::io;
fn main() {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Cannot able to read line");

    let number: i32 = number.trim().parse().expect("invalid input");

    if number > 0 {
        println!("The given number {} is positive", number);
    } else {
        println!("The given number {} is negative", number);
    }

    // returning values from loops

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {:?}", result);
    assert_eq!(result, 20);

    // for loop

    for x in 1..11 {
        if x == 5 {
            continue;
        }
        println!("x is {}", x);
    }
}
