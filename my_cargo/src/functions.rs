pub fn run() {
    greeting("Hello", "Jane");

    //bind functuons value to variables
    let get_sum = add(5,5);
    println!("Sum: {}", get_sum);

    let add_nums = |n1: i32,n2: i32| n1 + n2;
    println!("Closure Sum: {}", add_nums(3,3));

    let n3: i32 = 20;
    let again_add_num = |n1: i32,n2: i32| n1 + n2 + n3;
    println!("Again Cloure Sum: {}", again_add_num(3,3,));
}

fn greeting (greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}