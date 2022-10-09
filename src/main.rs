// const & static - Cannot Modify Value
// const MY_NUMBER: u32 = 12;
// static MY_DAYS: [&str; 4] = ["Monday", "Sunday", "Tuesday", "Wednesday"];

fn main() {

    // Arrays and Tuples
    // let mut arr: [i32; 5] = [1,2,3,4,5];
    // arr = [1,2,3,4,6];
    // println!("{}", arr[4]);

    // let mut tup: (i32, bool, char) = (1, true, 'a');
    // tup = (10, false, 'z');
    // print!("{}", tup.0)

    // Debug Printing
    // let my_number: () = {
    //     let second_number: i32 = 10;
    //     second_number + 5;
    // };

    // println!("My number is: {:#?}", my_number);

    // Variables
    // let mut x = 5;
    // x = 10;

    // let mut x = 1;
    // print!("value of x: {}", x);
    // x = 2;
    // println!("value of x: {}", x);

    // let y = true;
    // println!("value of y: {}", y);
    // let y = false;
    // println!("value of y: {}", y);

    // const STRING: &str = "hello";
    // println!("value of STRING: {}", STRING);

    // Mutability
    // let mut my_number = 8;
    // let my_number = "string";

    // println!("{}", my_number);

    // Shadowing
    // let my_number = 5;
    // println!("{}", my_number);
    // {
    //     let my_number = 10;
    //     println!("{}", my_number)
    // }
    // println!("{}", my_number);

    // let final_number: i32 = {
    //     let y = 2;
    //     let x = 3;
    //     let x = times_two(x);
    //     let x = x + y;
    //     x
    // };

    // println!("The number is now: {}", final_number);

    // let mut s1 = String::from("hello");
    // change(&mut s1);
    // println!("{}", s1);

    // Stack & Heap
    // let my_number = 15;
    // let reference1 = &my_number;
    // let reference2 = &&my_number;
    // let reference3 = &&&my_number;

    // println!("{}", ***reference3 == my_number);

    // let arr1 = [1,2,3];
    // let arr2 = arr1;

    // println!("Value: {:?}", (arr1, arr2));

    // let vec1 = vec![1,2,3];
    // let vec2 = &vec1;

    // println!("Value: {:?}", (&vec1, vec2));

    // // &str ใช้งาน Stack memory
    // let name_stack: &str = "Paul Atreides";

    // // String ใช้งาน Heap memory
    // let name_heap: String = String::from("Chani");
    
    // println!("{} {}", name_stack, name_heap);

    // // Array ใช้งาน Stack memory
    // let var_stack: [u8; 4] = [1, 3, 6, 9];

    // // Vec ใช้งาน Heap memory
    // let mut var_heap: Vec<u8> = Vec::new();
    // var_heap.push(2);
    // var_heap.push(4);
    // var_heap.push(6);
    // var_heap.push(8);

    // println!("var_stack = {:?}", var_stack);
    // println!("var_heap = {:?}", var_heap);

    // let a:i32 = 3;
    // let b:i32 = a * 10;
    // let c:i32 = add(b, 5);

    // let return_val = get_string();
    // //output: main: 0x7f86b4c05d10
    // println!("main: {:p}", return_val.as_ptr());
    // // ค่า address ที่ส่งออกมาจาก get_string ชี้ไปที่ heap อันเดียวกัน
    // // ทั้งใน get_string() และ main() = 0x7f86b4c05d10

    // Ownership & Borrowing
    // let mut say = String::from("Dog");
    // say.push_str(" and Cat");

    // let say2 = say;

    // println!("{}", say)

    // let say = String::from("Hello");
    // borrow_say(&say);
    // println!("{}", say);

    // let mut arr = vec![1,2,3];
    // println!("{:?}", arr);
    // borrow_arr(&mut arr);
    // println!("{:?}", arr)

    // let s = String::from("hello");

    // takes_ownership(s);

    // let x = 5;

    // makes_copy(x);

    // Return Ownership
    // let s1 = String::from("hello");
    
    // let (s2, len) = calcalate_length(s1);

    // println!("The length of '{}' is {}.", s2, len);

    // let s1 = String::from("hello");
    
    // let len = calcalate_length_ref(&s1);

    // println!("The length of '{}' is {}.", s1, len);



    // Advanced Printing
//     print!("\tGood morning\nGood morning");

//     println!("Good morning
// Good morning
// Good morning
// Good morning");

//     println!("When I was putting
//     together this site of games.");

//     println!("He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\"
// Then I found the file.");

//     println!(r#"He said, "You can find the file at c:\\files\\my_documents\\file.txt."
// Then I found the file."#);

//     println!("{:?}", b"This will look like numbers");

//     println!("{:X}", 'a' as u32);
//     println!("{:X}", 'H' as u32);
//     println!("{:X}", 'ก' as u32);
//     println!("{:X}", 'ฉ' as u32);

//     println!("\u{61}, \u{48}, \u{E01}, \u{E09}")

        // let number = 9;
        // let number_ref = &number;
        // println!("{:p}", number_ref);

        // let number = 555;
        // println!("Binary: {:b}, Hexadecimal: {:x}, Octal: {:o}", number, number, number);

        // let father_name = "adam";
        // let son_name = "jack";
        // let family_name = "matin";

        // println!("This is {1} {2}, son of {0} {2}.", father_name, son_name, family_name);

        // println!(
        //     "{city1} is in {country} and {city2} is also in {country},
        //     but {city3} is not in {country}.",
        //     city1 = "Bangkok",
        //     city2 = "Krabi",
        //     city3 = "Tokyo",
        //     country = "Thailand",
        // );

        // let letter = "a";
        // println!("{:*^10}", letter);
        // println!("{:*<10}", letter);
        // println!("{:*>10}", letter);

        // let title = "MORNING";
        // println!("{:-^30}", title);
        // let bar = "|";
        // println!("{:<15}{:>15}", bar, bar);
        // let a = "BANGKOK";
        // let b = "TOKYO";
        // println!("{city1:-<15}{city2:->15}", city1=a, city2=b)

        // Pointer
        // let my_num: i32 = 10;
        // let my_num_ptr = &my_num;
        // let my_num_raw_ptr = &my_num as *const i32;

        // println!(r#"{my_num}"#);
        // println!(r#"{my_num_ptr}"#);
        // println!("{:?}",my_num_raw_ptr);

        // let my_num: i32 = 10;
        // let my_num_ptr: *const i32= &my_num;

        // println!(r#"{my_num}"#);
        // println!("{:?}", my_num_ptr);
        
        // let mut my_speed: i32 = 88;
        // let my_speed_ptr: *mut i32 = &mut my_speed;
        // println!("{}", my_speed);
        // println!("{:?}", my_num_ptr);

        //  String
        // let name = "Good Morning";
        // let name2 = String::from("Good Afternoon");
        // let name3 = "Good Evening".to_string();

        // println!("{}", name);

        // println!("A String is {:?} bytes", std::mem::size_of::<String>());
        // println!("i8 is {:?} bytes", std::mem::size_of::<i8>());
        // println!("f64 is {:?} bytes", std::mem::size_of::<f64>());
        // println!("&str1 is {:?} bytes", std::mem::size_of_val("Hello"));
        // println!("&str2 is {:?} bytes", std::mem::size_of_val("Good Morning"));

        // let mut hello = String::from("Hello ");
        // hello.push('W');
        // hello.push_str("orld");

        // println!("Length: {}", hello.len());
        // println!("Capacity: {}", hello.capacity());
        // println!("Is empty?: {}", hello.is_empty());
        // println!("Contains: {}", hello.contains("Hello"));
        // println!("Replace: {}", hello.replace("World", "There"));

        // for word in hello.split_whitespace(){
        //     println!("{}", word);
        // }

        // println!("{}", hello);

        // let mut letter = String::with_capacity(4);
        // letter.push('a');
        // letter.push('b');
        // letter.push('c');

        // assert_eq!(3, letter.len());
        // assert_eq!(4, letter.capacity());
        
        // println!("{}", letter);

        // String Methods
        // {
        //     let my_string = String::from("Today is Monday");
        //     println!("Replaced: {}", my_string.replace("Monday", "Sunday"));
        // }
        // {
        //     let my_string = String::from("coconut\nbanana\napple");
        //     println!("item: {}", my_string);
        //     for line in my_string.lines(){
        //         println!("item: {}", line);
        //     }
        // }

        // {
        //     let my_string = String::from("KFC+Donut+Noodles");
        //     let tokens:Vec<&str> = my_string.split("+").collect();
        //     println!("index 1: {}", tokens[1]);
        // }
        // {
        //     let my_string = String::from("  I like a pizza\n");
        //     println!("Before Trim: {}", my_string);
        //     println!("After Trim: {}", my_string.trim());
        // }
        // {
        //     let my_string = String::from("Bangkok");
        //     println!("{}", my_string);
        //     match my_string.chars().nth(4){
        //         Some(c) => println!("Character at index 4: {}", c),
        //         None => println!("No character at index 4")
        //     }
        // }

        // Reference
        // let country1 = simple_ref();
        // println!("{}", country1);

        // let country2 = String::from("Malaysia");
        // let country_ref2 = &country2;
        // let country2 = 10;
        // println!("{}, {}", country2, country_ref2);
        
        // let mut number = 10;
        // let number_change = &mut number;
        // *number_change += 5;
        // let number_ref = &number;
        // println!("{}", number_ref);

        // let country3 = String::from("Malaysia");
        // let country4 = print_country(country3);
        // print_country(country4);

        // let country5 = String::from("Malaysia");
        // print_country2(&country5);
        // print_country2(&country5);

        // let mut country6 = String::from("Malaysia");
        // add_city(&mut country6);

        // let country7 = String::from("Malaysia");
        // add_city2(country7);
        

}
   
// fn add_city2(mut country_name: String){
//     country_name.push_str("-Penang");
//     println!("Welcome to {}", country_name);
// }

// fn add_city(country_name:&mut String){
//     country_name.push_str("-Penang");
//     println!("Welcome to {}", country_name);
// }

// fn print_country2(country_name:&String){
//     println!("{}", country_name);
// }

// fn print_country(country_name:String) -> String{
//     println!("{}", country_name);
//     country_name
// }

// fn simple_ref() -> String{
//     let country = String::from("Malaysia");
//     let contry_ref = &country;
//     contry_ref.to_string()
// }

// fn times_two(number: i32) -> i32 {
//     number * 2
// }

// fn borrow_say(_say:&String){
//     println!("{}", _say);
// }

// fn borrow_arr(_arr:&mut Vec<i32>){
//     _arr.push(4);
// }


// fn add(i: i32, j: i32) -> i32 {
//     i + j
// }

// fn get_string() -> String {
//     let charactor = String::from("Chani");
//     //output: inside get_string: 0x7f86b4c05d10
//     println!("inside get_string: {:p}", charactor.as_ptr());
//     charactor
// }

// fn takes_ownership(some_string: String){
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32){
//     println!("{}", some_integer);
// }

// fn calcalate_length(s:String) -> (String, usize){
//     let length = s.len();
//     (s, length)
// }

// fn calcalate_length_ref(s:&String) -> (usize){
//     s.len()
// }

// fn change(s: &mut String){
//     s.push_str(" world");




