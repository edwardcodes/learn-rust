// ------------------------------------------------------------
//                      VECTORS - Resizeable
// // ------------------------------------------------------------
fn main() {
    let mut number_vec: Vec<i32> = vec![4,5,6,7,8,-10,10,-2];
    println!("First element {}",number_vec[0]);

    println!("Accessing all elements in vec {:?}",number_vec);

    // Updating value
    number_vec[4] = 5;
    println!("Viewing changes in 4th index {:?}",number_vec);

    // arrray with same elements
    let same_array_elements: Vec<i32> = vec![1;5];

    let mut string_array = vec!["apple","tomato","4"];

    string_array[2] = "Orange";

    let subset_array = &number_vec[0..3];
    println!("Subset array {:?}",subset_array);

    // Checking the length of the vector array
    println!("Elements in the array {:?}",number_vec.len());

    // Getting index value
    let check_index = number_vec.get(2); // gives value
    let checks_none = number_vec.get(10); // returns none
    println!("Printing index value: {:?}", check_index);
    println!("Printing out of index value: {:?}", checks_none);

    // pushing values to the `mut` vector
    number_vec.push(30);
    number_vec.push(40);
    println!("The updated vec values are {:?}",number_vec);

    // removing values at a particular index
    println!("The vec values before removing {:?}",number_vec);
    number_vec.remove(2);
    println!("The vec values after removing {:?}",number_vec);

    // check whether value are present
    println!("Checking whether -10 is present: {:?}",number_vec.contains(&-10));


}