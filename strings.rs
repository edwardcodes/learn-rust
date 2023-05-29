
// ------------------------------------------
//          Strings
//              -- &str
//              -- String
// ------------------------------------------
fn main() {

    let some_string = "Fixed length string"; // &str default string datatype
    println!("Printing value of some_string.......{some_string}");

    let mut growable_string = String::from("This string will grow");
    println!("Printing value of growable_string..... \"{growable_string}\"");

    growable_string.push('s'); // appends the character
    println!("The appended string is \"{growable_string}\"");

    growable_string.pop();
    println!("The updated string after pop() is \"{growable_string}\"");

    growable_string.push_str(" Adds long sentence");
    println!("Appending long string: \"{growable_string}\"");

    println!("Basic function on strings,
    is_empty(): {},
    length: {},
    Bytes: {},
    Contains `add`: {},
    Contains `Add`: {}",
    growable_string.is_empty(),
    growable_string.len(), 
    growable_string.capacity(), 
    growable_string.contains("add"),
    growable_string.contains("Add")
);

    // trim whitespaces

    growable_string.push_str("     ");
    println!("Before removing whitespace: {}", growable_string.len());
    let str_len = growable_string.trim().len();
    println!("After removing whitespace using trim: {}", str_len);

    // number to string
    let number = 6;
    let num2str = number.to_string();
    println!("Is the number really a string {}", number.to_string()=="6");

    let my_name = "Edward".to_string(); // by default creates datatype as &str


    // empty string
    let empty_string = String::new();
    println!("Check empty string \"{}\" and its length {}",empty_string, empty_string.len());

    // concatenation
    let s_1 = "Edward".to_string();
    let s_2 = "Praveen".to_string();
    let s_3 = format!("{} {}",s_1,s_2);
    println!("{}",s_3);

}
