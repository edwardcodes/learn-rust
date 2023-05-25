// ------------------------------------------------------------
//            RUST OWNERSHIP
//              - Each value in rust has a variable thats called its owner
//              - There can be only one owner at a time
//              - When owner goes out of scope,the value will be dropped
// -------------------------------------------------------------

fn main() {
    // primitive types - int, float
    // ownership stays- copying the values automatically
    let x: f64 = 32.6;
    let y: f64 = x;

    println!("x: {}, y:{}", x, y);

    // Non -primitive types - Vector, Strings
    // Ownership issue occurs - clone or reference to the actual owner
    // Either referencing to the actual owner by `&` or by cloning values

    let s1 = String::from("hello world!");
    // let s2 = s1;
    // println!("S1: {}, S2: {}",s1,s2); // issue occurs
    let s2 = &s1;
    println!("S1: {}, S2: {}", s1, s2);

    // Same for vectors
    let vec_1 = vec![5, 6, 7, 8, 9];
    let vec_2 = vec_1.clone(); // returns the copy of value
    println!("Vec1: {:?}, Vec2: {:?}", vec_1, vec_2);

    // When owner goes out of scope,the value will be dropped
    {
        // scope
        let my_name = String::from("Edward Praveen");
        println!("The name is {}", my_name); // successful printing
    }

    // throws error, out of scope value dropped
    // println!("The name is {}",my_name);

    // Another Scenario
    let stack_num = 32;
    let mut heap_vec = vec![4, 5, 6];

    stack_fn(stack_num); // won't change immutable stack_num
    println!("The value inside the main fn of stack_num: {}", stack_num);

    heap_fn(&heap_vec);
    // put `&` as reference to heap_vec and in heap_fn to avoid showing below errors
    // this moves ownership from heap_vec to var
    // borrow of moved value: `heap_vec`value borrowed here after move
    // use `&mut` if you want to update the value
    println!("The value inside the main fn of heap_vec: {:?}", heap_vec);

    let large_data_1 = String::from("This is first long string");
    let large_data_2 = String::from("This is seconf long string");

    let combined_data = vec![&large_data_1, &large_data_2];
    // use vec! for better memory instead of combining in traditional way
    println!("combined data: {:#?}",combined_data);



}

fn stack_fn(mut var: i32) {
    var = 56;
    println!("Var: {}", var);
}

fn heap_fn(var: &Vec<i32>) {
    println!("Var: {:?}", var);
}