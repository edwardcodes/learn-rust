//----------------------------------------------------------
//              VARIABLES
//              DATA TYPES
//-------------------------------------------------------

fn main() {
    let mut x = 15; /* by default rust variable are immutable, if we want
                    change, introduce them as `mut`*/
    println!("Initial value of x is {x}");
    x = 60.5 as i32;
    println!("After changing: {x}");

    let z = 3.65;

    let x = 60;

    let a = x + z as i32;

    println!("Treat `z` as int {a}");

    let b = x as f64 + z;

    println!("Treat `x` as float {b}");

    // ---------------------------------------------------------------
    //              VARIABLES
    //                  - Shadowing
    //                  - Constants
    // ------------------------------------------------------

    // let (first_number, second_number) = (125,250.05);
    // let large_number = 1_000_000; //large number separated by `_`
    println!("Max number of u8: {}", std::u8::MAX);

    // let exceed_number:u8 = 256; //note: the literal `256` does not fit into the type `u8` whose range is `0..=255`

    // println!("Exceed number: {exceed_number}");

    // Shadowing
    let s = 5;
    let s = 5 * 5; // overwriting the variable

    println!("The updated value of s : {s}");

    // Shadowing datatype
    let i: i32 = 5;
    let i = "i";

    println!("Updated value of i {i}");

    // Shadowing on scope
    let r = 60;
    {
        let r = 52.55;
        println!("Inside the scope `r` is {r}");
    }
    println!("Outside the scope `r` is {r}");
}
