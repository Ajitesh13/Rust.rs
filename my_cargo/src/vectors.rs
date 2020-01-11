// Array - Fixed list where elements are the same data types
// by default they are immutable

//use std::mem;

pub fn run (){
    let number: Vec<i32> = vec![1,2,3,4]; // parameter is the datatype
    println!("{:?}", number); // {:?} is the debugging tarit helps to print out the whole container
    println!("{} ",number[0]); // single value
    // number.push(90); // pushing to non-mutable vector is error
    println!("{:?} ",number);

    let mut mut_number: Vec<i32> = vec![6,7,8,9]; //in array we cant numbers but we can change
    mut_number.push(90);
    mut_number.push(100);
    mut_number[3] = 30;
    println!("{:?} ", mut_number);
    mut_number.pop(); // last one is gone(here: 100)
    println!("{:?} ", mut_number);
    println!("Vector length: {}", number.len());
    //Vectors are stack allocated
    println!("Vector occupies {}", std::mem::size_of_val(&number));
    println!("Vector occupies {}", std::mem::size_of_val(&mut_number));

    //get slice
    let slice: &[i32] = &number;
    println!("Slice: {:?}", slice);
    let slice1: &[i32] = &number[0..2];
    println!("Slice1: {:?}",slice1);

    //loop through vectors
    for x in number.iter(){
        println!("Number: {}",x);
    }

    //loop & mutate values
    for x in mut_number.iter_mut(){
        *x *= 2;
    }
    println!("numbes Vec: {:?}", mut_number); //printing the whole container
}