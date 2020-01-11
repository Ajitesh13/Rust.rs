/*
Primitive Types:
Integers: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128(no. of bits they
    take in memory, u stands for unsigned)
floats: f32,f64
bool
char
tuples
arrays(fixed length)
*/
// Rust is a statically types language

pub fn run(){
    let x = 1; // default is "i32"
    let y = 2.5; // deafult in "f64"
    let z: i64 = 12345678909876543;

    //find max size
    println!("Max i32: {}",std::i32::MAX);
    println!("Max i32: {}",std::i64::MAX);

    //bool
    let is_active = true;
    let is_active_also: bool = true;
    //println!("{:?} ",x,y,z,is_active);

    //get bool from expression
    let is_greater = 10 > 5;
    println!(" {:?} ", is_greater); // true

    //char
    let a1 = "a";
    let face = "\u{1F600}"; // emoji unicode

    println!("{}",a1);
    println!("{}",face);

    println!("{:?}", (x,y,z,is_active,is_active_also,is_greater,a1,face));


}