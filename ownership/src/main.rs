// ------------------------------------------------------------
//            RUST OWNERSHIP
//              - Each value in rust has a variable thats called its owner
//              - There can be only one owner at a time
//              - When owner goes out of scope,the value will be dropped
// -------------------------------------------------------------

fn main() {
    // primitive types - int, float
    // ownership stays- copying the values automatically
    let x:f64 = 32.6;
    let y:f64 = x;

    println!("x: {}, y:{}",x,y); 

    // Non -primitive types - Vector, Strings
    // Ownership issue occurs - clone or reference to the actual owner
    // Either referencing to the actual owner by `&` or by cloning values



    let s1 = String::from("hello world!");
    // let s2 = s1;
    // println!("S1: {}, S2: {}",s1,s2); // issue occurs
    let s2 = &s1; 
    println!("S1: {}, S2: {}",s1,s2);

    // Same for vectors
    let vec_1 = vec![5,6,7,8,9];
    let vec_2 = vec_1.clone(); // returns the copy of value
    println!("Vec1: {:?}, Vec2: {:?}",vec_1,vec_2);

    // When owner goes out of scope,the value will be dropped
    {
        // scope
        let my_name = String::from("Edward Praveen");
        println!("The name is {}",my_name); // successful printing
    }

    // throws error, out of scope value dropped
    // println!("The name is {}",my_name); 

}