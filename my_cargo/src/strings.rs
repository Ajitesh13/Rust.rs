// Primitive str = immutable fixed-length string somewhere in memory
// string = growable, head-allocated DS: used when needed to modify string data

pub fn run(){
    let hello1 = "Hello1"; // primitive str
    println!("{} ", hello1);
    let hello = String::from("Hello"); // String
    println!("{} ", hello); //
    println!("Length: {} ", hello.len() );
    //hello.push('w'); // error as string hello is not mutable
    let mut new = String::from("Hey!");
    println!("{} ", new);
    new.push('s'); // pushing a char
    println!("{} ", new);
    new.push_str("Somethingnew!"); // pushing a string
    println!("{} ", new);
    //capacity in bytes
    println!("Capacity: {} ",new.capacity() );
    println!("{} ",new.is_empty()); // check if empty, return false
    println!("{} ", new.contains("Something")); // check if contains return true
    // Replace
    println!("Replace: {}", new.replace("Something","World"));

    //loop through string by whitespace
    for word in new.split_whitespace(){
        println!("{} ",word);
    }
     //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
     println!("{} ",s);

     //assertion testing

    assert_eq!(2, s.len()); // nothing will happen as right_parameter==left_parameter
                            // if false then program will show error

}