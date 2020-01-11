pub fn run() {
    //primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("arr_values: {:?}", (arr1, arr2));

    /*
    with non-primitives, if you assign another variable to a piece of data
    the first variable will no loneger hold that value. You'll need to use a
    reference (&) to point to the resource
    */

    //vector
    let vec1 = vec![1,2,3];
    // let vec2 = vec1; // if we write this way then vec1 will loses its value
    // println!("{:?}", (vec1,vec2)); // error as vec1 no longer holds its vales

    let vec2 = &vec1;

    println!("vac_values: {:?}", (&vec1,vec2));


}