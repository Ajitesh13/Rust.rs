// enums are types which have a few definite values

enum Movement{
    // variants
    up,
    down,
    left,
    right
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::up => println!("Avatar moving up"),
        Movement::down => println!("Avatar moving down"),
        Movement::left => println!("Avatar moving left"),
        Movement::right => println!("Avatar moving right")
    }
}

pub fn run() {
    let avatar1 = Movement::left;
    let avatar2 = Movement::up;
    let avatar3 = Movement::right;
    let avatar4 = Movement::down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}

