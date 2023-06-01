// ------------------------------------------------------------------
//              STACK IMPLEMENTATION
// ------------------------------------------------------------------

/// Setting the capacity
fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec = Vec::with_capacity(maxsize);
    vec // return vec
}

/// removes the last element
fn pop(stack: &mut Vec<u32>) -> Option<u32> {

    let popped_value = stack.pop(); // removes the value
    println!("The popped value is {:#?}", popped_value);
    popped_value
}

// Option<u32> accepts to return u32 or None if none values are present
/// pushes new element if maxsize hasn't attained
fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {

    if stack.len() == maxsize {
        println!("cannot add more!");
    } else {
        stack.push(item);
        println!("Current Stack: {:#?}", stack);
    }
}
// Since we are not updating anything on stack, we are just referencing instead of `mut` reference
/// Displays size of the stack
fn size(stack: &Vec<u32>) -> usize {
    stack.len() // returns the current length/size of the stack.
}

/// Get inputs from user
fn user_input() -> u32 {
    let mut number = String::new();
    std::io::stdin()
        .read_line(&mut number)
        .expect("failed to read input");

    let number: u32 = number.trim().parse().expect("Invalid input type");
    number // returns input
}

fn main() {
    println!("Lets create a stack for our use!");
    println!("Please mention the size of the stack");
    let size_stack = user_input();

    let mut stack = new_stack(size_stack as usize); // u32 -> usize

    // add loop to ask inputs from user continuously
    loop {
        println!("\n\n ****** MENU ***** \n\n");
        println!("1. Push \n 2. Pop \n 3. Display \n 4. Size \n 5. Exit");
        let choice = user_input();
        match choice {
            1 => {
                println!("Enter the value to be inserted:");
                let item = user_input();
                push(&mut stack, item, size_stack as usize);
            }
            2 => println!("The element which is popped is: {:#?}", pop(&mut stack)),
            3 => println!("The elements are: {:#?}", stack),
            4 => println!("The Size of the stack is: {}", size(&stack)), // to avoid moving ownership (referencing)
            5 => break, // println!("\n Exiting..!"),
            _ => println!("Wrong Selection. Try again?"),
        }
        println!("do you want to continue 1 => Yes / 0 => No");
        let status = user_input();
        if status == 1 {
            continue;
        } else {
            break;
        }
    }
}
