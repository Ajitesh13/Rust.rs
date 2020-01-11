pub fn run(){
    //if-else
    let age = 16;
    let new_age: u8 = 20;
    let check_id: bool = true;

    if age>=18 {
        println!("In the if block");
    }else if check_id == false {
        println!("In the else-if block");
    }else {
        println!("In the else block");
    }

    //Shorthand if, can be used in place of ternary operator

    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of Age: {}", is_of_age);
}