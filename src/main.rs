fn main() {

    // Type Inference
    let small_number: u8 = 10;
    let small_number = 10u8; // 10u8 = 10 of type u8
    let small_number = 10_u8; // This is easier to read
    let big_number = 100_000_000_i32; // 100 million is easy to read with _
    let number = 0________u8;
    let number2 = 1___6______2____4______i32;
    println!("{}, {}", number, number2);
    
    let my_float = 5.; // Rust sees . and knows that it is a float

    let my_float: f64 = 5.0;
    let my_other_float: f32 = 8.5;

    let third_float = my_float + my_other_float as f64; // my_other_float as f64 = use my_other_float like an f64

    let my_float = 5.0; // Rust will choose f64
    let my_other_float = 8.5; // Here again it will choose f64

    let third_float = my_float + my_other_float;

    let my_float: f32 = 5.0;
    let my_other_float = 8.5; // Usually Rust would choose f64,

    let third_float = my_float + my_other_float; // but now it knows that you need to add it to an f32. So it chooses f32 for my_other_float too

    // Types
    // let first_letter = 'A';
    // let space = ' '; // A space inside ' ' is also a char
    // let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    // let cat_face = '😺'; // Emojis are chars too

    // let my_number = 100; // We didn't write a type of integer,
    //                      // so Rust chooses i32. Rust always
    //                      // chooses i32 for integers if you don't
    //                      // tell it to use a different type

    // println!("{}", my_number as u8 as char); // ⚠️

    // let my_number: u8 = 100; //  change my_number to my_number: u8
    // println!("{}", my_number as char);

    // println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    // println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    // println!("Size of string containing 'ß': {}", "ß".len());
    // println!("Size of string containing '国': {}", "国".len());
    // println!("Size of string containing '𓅱': {}", "𓅱".len());
    // println!("Size of string containing 'ก': {}", "ก".len());

    // let slice = "Hello!";
    // println!("Slice is {} bytes.", slice.len());
    // let slice2 = "안녕!"; // Korean for "hi"
    // println!("Slice2 is {} bytes.", slice2.len());
    // let slice3 = "สวัสดี"; // Thai for "hi"
    // println!("Slice3 is {} bytes.", slice3.len());

    // let slice = "Hello!";
    // println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    // let slice2 = "안녕!";
    // println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
    // let slice3 = "สวัสดี";
    // println!("Slice3 is {} bytes but only {} characters.", slice3.len(), slice3.chars().count());

}
