
use std::io;

fn main() {
    println!("Enter the marks of the student:");
    let mut marks = String::new();
    std::io::stdin()
    .read_line(&mut marks)
    .expect("failed to read input");

    let marks:i32 = marks.trim().parse().expect("invalid input type");

    // if..else..else if
    if marks >= 80 {
        println!("Distinction!");
    } 
    else if marks >= 70 && marks < 80 {
        println!("First Class!");
    }
    else if marks >=60 && marks < 70 {
        println!("Second Class !");
    }
    else if marks >=40 && marks < 60 {
        println!("Third Class !");
    }
    else {
        println!("Repeatuuuu!");
    }


    // if let..
    let end_result = if marks >= 40 {
        "pass"
    } else {
        "fail"
    };

    println!("The end result: {}",end_result);

    // match statement
    match marks {
        10 => println!("The number is 10"),
        2 | 3 | 5 => println!("prime number is ur marks :)"),
        0..=39 => println!("Erumai meika po da :D"),
        40..=80 => println!("poye thola!"),
        81..=99 => println!("yen centum vaangala??"),
        _ => println!("Exam ku poniya ne?"), // show this none matches
    }


}
