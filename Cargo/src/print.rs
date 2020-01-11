pub fn run(){
    //print to conosle
    println!("Hello from the print.rs file");
    println!("{}",1);

    //basic formatting
    println!("Number:{} {} {} ",1,2,3);
    println!("Number:{}{}{} ",1,2,3);

    //positional argument
    println!("{} is from {}","Ajitesh","india");
    println!("{0} like {1} and {0} like {2}","i","tea","coffee");

    //named arguments
    println!("{name} like to {love} ",name="I",love="Code");

    //placeholders traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

    //placeholder for debug traits
    println!(" {:?} ",(12,true,"hello"));

    //basic maths
    println!("10 + 10 = {} ",10+10);

}