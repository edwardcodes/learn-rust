// ----------------------------------------------------------------
//      Reference Rules
//          - One mutable reference in a scope
//          - Many immutable references
//          - Mutable and Immutable cannot coexist
//          - Scope of reference
//          - Data must not change when immutable references are in scope
// --------------------------------------------------------------------


fn main() {

// One mutable reference in a scope
let mut heap_num = vec![4,5,6];
let ref1 = &mut heap_num;
let ref2 = &mut heap_num;
// error at ref2 - cannot borrow `heap_num` as mutable more than once at a time
println!("ref1: {:?}, ref2: {:?}",ref1,ref2);

// Many immutable references
let mut heap_num_1 = vec![4,5,6];
let ref1_1 = &heap_num_1;
let ref2_1 = &heap_num_1;
// this time ref2_1 doesn't throw error as we referenced as immutables
println!("ref1: {:?}, ref2: {:?}",ref1_1,ref2_1);

// Mutable and Immutable cannot coexist
let mut heap_num_2 = vec![4,5,6];
let ref1_2 = &heap_num_2;
let ref2_2 = &heap_num_2;
let ref3_2 = &mut heap_num_2;
// error at ref3 - cannot borrow `heap_num_2` as mutable because it is also borrowed as immutable
println!("ref1: {:?}, ref2: {:?}, ref3: {:?}",ref1_2,ref2_2, ref3_2);

// Scope of reference - Solving Mutable and Immutable cannot coexist issue
let mut heap_num_3 = vec![4,5,6];
let ref1_3 = &heap_num_3;
let ref2_3 = &heap_num_3;
println!("ref1: {:?}, ref2: {:?}",ref1_3,ref2_3);
// mut reference after printing out immutables - scope in memory completes
let ref_3_3 = &mut heap_num_3;
println!("ref3: {:?}",ref_3_3);

// Data must not change when immutable references are in scope
// Data cannot be changed when others are using it
let mut heap_num_4 = vec![4,5,6];
let ref1_4 = &heap_num_4;
let ref2_4 = &heap_num_4;

heap_num_4.push(58); // trying to add new one when it is already ref by ref1_4,ref2_4
// error - cannot borrow `heap_num_4` as mutable because it is also borrowed as immutable
println!("ref1: {:?}, ref2: {:?}",ref1_4,ref2_4);



}
