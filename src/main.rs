// fn number() -> i32 {
//     8
// }

// fn multiply(number_one: i32, number_two: i32) -> i32 { // Two i32s will enter the function. We will call them number_one and number_two.
//     let result = number_one * number_two;
//     println!("{} times {} is {}", number_one, number_two, result);
//     result
// }

// fn times_two(number: i32) -> i32 {
//     number * 2
// }

fn main() {

    // More about printing
    // Note: this is print!, not println!
    print!("\t Start with a tab\nand move to a new line");

    // Note: After the first line you have to start on the far left.
    // If you write directly under println!, it will add the spaces
    println!("Inside quotes
you can write over
many lines
and it will print just fine.");

    println!("If you forget to write
    on the left side, the spaces
    will be added when you print.");

    println!("Here are two escape characters: \\n and \\t");

    println!("He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file."); // We used \ five times here
    println!(r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#);

    let my_string = "'Ice to see you,' he said."; // single quotes
    let quote_string = r#""Ice to see you," he said."#; // double quotes
    let hashtag_string = r##"The hashtag #IceToSeeYou had become very popular."##; // Has one # so we need at least ##
    let many_hashtags = r####""You don't have to type ### to use a hashtag. You can just use #.""####; // Has three ### so we need at least ####

    println!("{}\n{}\n{}\n{}\n", my_string, quote_string, hashtag_string, many_hashtags);

    let r#let = 6; // The variable's name is let
    let mut r#mut = 10; // This variable's name is mut

    println!("let = {}, mut = {}", r#let, r#mut);

    println!("{:?}", b"This will look like numbers");

    println!("{:?}", br##"I like to write "#"."##);

    println!("{:X}", '행' as u32); // Cast char as u32 to get the hexadecimal value
    println!("{:X}", 'H' as u32);
    println!("{:X}", '居' as u32);
    println!("{:X}", 'い' as u32);

    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}"); // Try printing them with unicode escape \u

    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);

    let number = 555;
    println!("Binary: {:b}, hexadecimal: {:x}, octal: {:o}", number, number, number);

    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Țepeș";
    println!("This is {1} {2}, son of {0} {2}.", father_name, son_name, family_name);

    println!(
        "{city1} is in {country} and {city2} is also in {country},
but {city3} is not in {country}.",
        city1 = "Seoul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea"
    );

    let letter = "a";
    println!("{:ㅎ^11}", letter);

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); // variable names city1 and city2, pad with -, one to the left, one to the right
    println!("{:-<15}{:->15}", a, b);

    // Stack, Heap and Pointer
    // let my_number = 15; // This is an i32
    // let single_reference = &my_number; //  This is a &i32
    // let double_reference = &single_reference; // This is a &&i32
    // let five_references = &&&&&my_number; // This is a &&&&&i32
    // println!("{}", five_references);

    // Mutability & Shadowing
    // let mut my_number = 8;
    // my_number = 10;

    // let my_number = 8; // This is an i32
    // println!("{}", my_number); // prints 8
    // let my_number = 9.2; // This is an f64 with the same name. But it's not the first my_number - it is completely different!
    // println!("{}", my_number); // Prints 9.2

    // let my_number = 8; // This is an i32
    // println!("{}", my_number); // prints 8
    // {
    //     let my_number = 9.2; // This is an f64. It is not my_number - it is completely different!
    //     println!("{}", my_number) // Prints 9.2
    //                               // But the shadowed my_number only lives until here.
    //                               // The first my_number is still alive!
    // }
    // println!("{}", my_number); // prints 8

    // let final_number = {
    //     let y = 10;
    //     let x = 9; // x starts at 9
    //     let x = times_two(x); // shadow with new x: 18
    //     let x = x + y; // shadow with new x: 28
    //     x // return x: final_number is now the value of x
    // };
    // println!("The number is now: {}", final_number);

    // // Pretending we are using Rust without shadowing
    // let final_number = {
    //     let y = 10;
    //     let x = 9; // x starts at 9
    //     let x_twice = times_two(x); // second name for x
    //     let x_twice_and_y = x_twice + y; // third name for x!
    //     x_twice_and_y // too bad we didn't have shadowing - we could have just used x
    // };
    // println!("The number is now: {}", final_number)

    // Display & Debug
    // let doesnt_print = ();
    // println!("This will not print: {:#?}", doesnt_print);

    // print!("This will not print a new line");
    // println!(" so this will be on the same line");

    // println!("The smallest i8 is {} and the biggest i8 is {}.", i8::MIN, i8::MAX); // hint: printing std::i8::MIN means "print MIN inside of the i8 section in the standard library"
    // println!("The smallest u8 is {} and the biggest u8 is {}.", u8::MIN, u8::MAX);
    // println!("The smallest i16 is {} and the biggest i16 is {}.", i16::MIN, i16::MAX);
    // println!("The smallest u16 is {} and the biggest u16 is {}.", u16::MIN, u16::MAX);
    // println!("The smallest i32 is {} and the biggest i32 is {}.", i32::MIN, i32::MAX);
    // println!("The smallest u32 is {} and the biggest u32 is {}.", u32::MIN, u32::MAX);
    // println!("The smallest i64 is {} and the biggest i64 is {}.", i64::MIN, i64::MAX);
    // println!("The smallest u64 is {} and the biggest u64 is {}.", u64::MIN, u64::MAX);
    // println!("The smallest i128 is {} and the biggest i128 is {}.", i128::MIN, i128::MAX);
    // println!("The smallest u128 is {} and the biggest u128 is {}.", u128::MIN, u128::MAX);

    // Printing "Hello World"
    // println!("Hello, world!");
    // println!("Hello, world number {}!", 8);
    // println!("Hello, worlds number {} and {}!", 8, 9);

    // println!("Hello, world number {}!", number());

    // multiply(8, 9); // We can give the numbers directly
    // let some_number = 10; // Or we can declare two variables
    // let some_other_number = 2;
    // multiply(some_number, some_other_number); // and put them in the function

    // let multiply_result = multiply(8, 9); // We used multiply() to print and to give the result to multiply_result
    // println!("{}",multiply_result);

    // let my_number = 8;
    // println!("Hello, number {}", my_number);

    // let my_number = {
    //     let second_number = 8;
    //         second_number + 9 // No semicolon, so the code block returns 8 + 9.
    //                           // It works just like a function
    //     };
    
    //     println!("My number is: {}", my_number);

    // let my_number = {
    //     let second_number = 8; // declare second_number,
    //         second_number + 9; // add 9 to second_number
    //                                // but we didn't return it!
    //                                // second_number dies now
    //     };
        
    // println!("My number is: {:?}", my_number); // my_number is ()

    // Type Inference
    // let small_number: u8 = 10;
    // let small_number = 10u8; // 10u8 = 10 of type u8
    // let small_number = 10_u8; // This is easier to read
    // let big_number = 100_000_000_i32; // 100 million is easy to read with _
    // let number = 0________u8;
    // let number2 = 1___6______2____4______i32;
    // println!("{}, {}", number, number2);
    
    // let my_float = 5.; // Rust sees . and knows that it is a float

    // let my_float: f64 = 5.0;
    // let my_other_float: f32 = 8.5;

    // let third_float = my_float + my_other_float as f64; // my_other_float as f64 = use my_other_float like an f64

    // let my_float = 5.0; // Rust will choose f64
    // let my_other_float = 8.5; // Here again it will choose f64

    // let third_float = my_float + my_other_float;

    // let my_float: f32 = 5.0;
    // let my_other_float = 8.5; // Usually Rust would choose f64,

    // let third_float = my_float + my_other_float; // but now it knows that you need to add it to an f32. So it chooses f32 for my_other_float too

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
