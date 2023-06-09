use std::io;

fn main() {
    println!("Enter the marks of the student:");
    let mut marks = String::new();
    std::io::stdin()
        .read_line(&mut marks)
        .expect("failed to read input");

    let marks: i32 = marks.trim().parse().expect("invalid input type");

    // if..else..else if
    if marks >= 80 {
        println!("Distinction!");
    } else if marks >= 70 && marks < 80 {
        println!("First Class!");
    } else if marks >= 60 && marks < 70 {
        println!("Second Class !");
    } else if marks >= 40 && marks < 60 {
        println!("Third Class !");
    } else {
        println!("Repeatuuuu!");
    }

    // if let..
    let end_result = if marks >= 40 { "pass" } else { "fail" };

    println!("The end result: {}", end_result);

    // match statement
    let mut mom_comments: String = String::new();
    match marks {
        10 => mom_comments.push_str("The number is 10"),
        2 | 3 | 5 => mom_comments.push_str("prime number is ur marks :)"),
        0..=39 => mom_comments.push_str("Erumai meika po da :D"),
        40..=80 => mom_comments.push_str("poye thola!"),
        81..=99 => mom_comments.push_str("yen centum vaangala??"),
        _ => mom_comments.push_str("Exam ku poniya ne?"), // show this none matches
    }
    println!("My mom comments: {}", mom_comments);

    // let..match statement
    let mut mom_comments = match marks {
        10 => "The number is 10",
        2 | 3 | 5 => "prime number is ur marks :)",
        0..=39 => "Erumai meika po da :D",
        40..=80 => "poye thola!",
        81..=99 => "yen centum vaangala??",
        _ => "Exam ku poniya ne?", // show this none matches
    };
    println!("My mom comments_updated: {}", mom_comments);

    // while loop
    let mut number = String::new();

    std::io::stdin()
    .read_line(&mut number)
    .expect("failed to read input");

    let mut number:u8 = number.trim().parse().expect("invalid type");

    number += 1; 
    // to avoid displaying number given is already div by 2 & 5
    // it will show next number
    while (number % 2 == 0 && number % 5 == 0) != true {
        number += 1;
    };
    println!("The number after your no divisible by 2 & 5 is {}",number);

    // for loop
    
    let mut some_vec = vec![4,5,6,7,8];

    for i in 0..some_vec.len() {
        println!("The value of index {} position is {}",i,some_vec[i]);
    }

    println!("{:#?}",some_vec);
