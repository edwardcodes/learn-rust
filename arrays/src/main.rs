
// ---------------------------------------------------
//                  TUPLES
//                  ARRAYS
// // ------------------------------------------------
fn main() {

    // Tuples

    // Printing out Tuple values
    let sample_info = ("Sulekha", 10_00_00_000);
    println!("{} has a valuation of {}",sample_info.0, sample_info.1);

    println!("Printing all tuple values at once: {:?}",sample_info);
    
    // saving tuple values into variables
    let (company, valuation) = sample_info;
    println!("Company: {company}, Valuation: {valuation}");
    let single_value_tuple = sample_info.1;
    println!("Extracting Single value from tuple.....{}",single_value_tuple);

    // Nested Tuple
    let nested_tuple = (4,5.0,(3,true),"Hello");
    let decode_tuple = nested_tuple.2.1;
    println!("Decoded Tuple value -> {}",decode_tuple);

    // instantiating empty tuple
    let empty_tuple = ();
    println!("Printing empty tuple......\"{:?}\"",empty_tuple);


    // Arrays - Same Datatype
    
    let mut number_arrays = [5,12,2,-1,0];
    println!("Original array: {:?}",number_arrays); // to display all values
    number_arrays[0] =2;
    number_arrays[2] = 5;
    println!("Changed array: {:?}",number_arrays);
    let single_value = number_arrays[1];
    println!("{}",single_value);

    // array of same elements
    let array_with_same_elements = [5;10]; //5 with 10 values
    println!("{:?}",array_with_same_elements);
    let string_array = ["Unknown";5];
    println!("String array with same elements....{:?}",string_array);

    // Slicing arrays
    let mut number_array_1 = [4,5,6,-1,10,2];
    let subset_array = &number_array_1[1..4];
    //`&` are references and act as pointer to the relevant array. You cannot update the original array
    // 1..4 consider value from 1st index till 3rd

    println!("Subset/Sliced Array: {:?}",subset_array);


    //inclusion of last value without exclusion
    let subset_array_wo_exclusion = &number_array_1[1..=4];
    println!("Subset/Sliced array wo exclusion: {:?}",subset_array_wo_exclusion);
   
    // length of array
    println!("Elements in the array are....{}",number_array_1.len());

    // bytes
    println!("Total bytes memory by the array...{}",std::mem::size_of_val(&number_array_1));

    // check index value using get()
    let check_index = number_array_1.get(2);
    println!("Displaying element in index 2: {:?}",check_index);

    // returns `None` if the index out of the range
    let check_none = number_array_1.get(7);
    println!("Displaying `None` in index 7: {:?}",check_none);

    // check index value for a particular range
    let check_index_range = number_array_1.get(0..=2);
    println!("Displaying element in index range: {:?}",check_index_range);

}