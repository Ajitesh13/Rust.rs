// Array - Fixed list where elements are the same data types
// by default they are immutable

//use std::mem;

pub fn run (){
    let number: [i32; 5] = [1,2,3,4,5]; // first parameter is the datatype and the next one is array size
    println!("{:?}", number); // {:?} is the debugging tarit helps to print out the whole container
    println!("{} ",number[0]); // single value

    let mut mut_number: [i32; 4] = [6,7,8,9]; //in array we cant numbers but we can change
    mut_number[3] = 30;
    println!("{:?} ", mut_number);
    println!("Array length: {}", number.len());
    // Array are stack allocated
    println!("Array occupies {}", std::mem::size_of_val(&number));
    println!("Array occupies {}", std::mem::size_of_val(&mut_number));

    //get slice
    let slice: &[i32] = &number;
    println!("Slice: {:?}", slice);
    let slice1: &[i32] = &number[0..2];
    println!("Slice1: {:?}",slice1);
}