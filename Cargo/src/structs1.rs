// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct Color1(u8,u8,u8);

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("Color: {} {} {}", c.red, c.green, c.blue );
    c.red = 200; // declared mutable so can be changed
    println!("Color: {} {} {}", c.red, c.green, c.blue );

    let mut c1 = Color1(255,0,0);
    println!("Color1: {} {} {} ", c1.0,c1.1,c1.2);
    c1.0 = 200;
    println!("Color1: {} {} {} ", c1.0,c1.1,c1.2);
}