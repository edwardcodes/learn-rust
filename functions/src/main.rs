// -----------------------------------------------------------
//                  FUNCTIONS AND INPUTS
// ----------------------------------------------------------
fn main() {
    fn_with_inputs("Edward", 34);

    let name_info = "Anto";
    let age_info = 32;

    fn_with_inputs(name_info, age_info);

    let answer: i32 = func_with_inputs_outputs(10, 15);
    println!("Multiplication {}", answer);

    let (mul, add, sub) = func_with_inputs_mul_outputs(15, 10);
    println!("Multi {}, Addition {}, subtraction {}", mul, add, sub);

    // Code block
    let full_name: String = {
        let first_name = "Edward";
        let last_name = "Praveen";
        format!("{} {}", first_name, last_name)
    };

    println!("Full name -> {}", full_name);

    // INPUTS
    let mut n: String = String::new(); // new empty string
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");

    // Converting input string to other datatype
    let n: f64 = n.trim().parse().expect("invalid input");
    // parse() is used into another type
    println!("{:?}", n);
}

fn fn_with_inputs(name: &str, age: i32) {
    println!("{} is {} years old", name, age);
}

fn func_with_inputs_outputs(num1: i32, num2: i32) -> i32 {
    return num1 * num2;
}

fn func_with_inputs_mul_outputs(num1: i32, num2: i32) -> (i32, i32, i32) {
    return (num1 * num2, num1 + num2, num1 - num2);
}
