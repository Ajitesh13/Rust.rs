use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let name = "Ajitesh";
    let status = "99%";

    println!("Args: {:?}", args);

    let command_given_by_me = args[1].clone();

    println!("Command By Me: {}", command_given_by_me);
    /*  will print if the word that i will type after $ cargo run
        example: ```$ cargo run Something``` will give output
        Command By Me: Something
    */
    if command_given_by_me == "hello" {
        println!("Hello {}, how are you? ", name)
    } else if command_given_by_me == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}