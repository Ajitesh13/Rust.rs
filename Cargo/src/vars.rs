//variables hold primitive data or reference to data
// variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Brad";
    let age = 37;
    // age = 38 // give error as variable are by default immutable in rust
    println!("my name is {} and I am {}",name,age);
    let mut age = 38;
    println!("my name is {} and I am {}",name,age);
    age = 39;
    println!("my name is {} and I am {}",name,age);

    //define constant
    const ID: i32 = 001;
    println!("ID: {}",ID);

    //Assign multiple vars
    let(my_name,my_age) = ("Brad",37);
    println!("{} is {} ", my_name,my_age);
}