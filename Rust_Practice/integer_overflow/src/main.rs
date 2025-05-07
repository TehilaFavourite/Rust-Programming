
fn main() {
    // // experiment with integer overflow scenarios
    // let large_int = std::i32::MAX;
    // let result = large_int + 1;
    // println!("Integer overflow: {}", result);

    // // experiment with floating point overflow scenarios
    // let large_float = std::f64::MAX;
    // let result = large_float + 1.0;
    // println!("Floating point overflow: {}", result);
    // // result: OverflowError
    // // Note: This error is expected due to the limitations of floating point arithmetic
    //       and can't be avoided in this simple program.

    let interger = 10;
    let float = 11.0;
    let string = "Hello, world!";
    let tuple = (20, 5.8, "Hello");
    let array = [1, 2, 3, 4, 5];

    println!("Integer: {}", interger);
    println!("Float: {}", float);
    println!("String: {}", string);
    println!("Tuple: {:?}", tuple);
    println!("Array: {:?}", array);

    let boolean = true;
    let character = 'A';
    let option_value: Option<i32> = Some(42);
    let result = if boolean { "True" } else { "False" };
    println!("Boolean: {}", result);
    println!("Character: {}", character);
    println!("Option value: {:?}", option_value);

    let a = 90;
    println!("first outer a: {}", a);
    let a = a + 10;
    {
        let a = a * 2;
        println!("first Inner a: {}", a);
        let a = a - 30;
        println!("second Inner a: {}", a);
    }
    println!("second Outer a: {}", a);

    let mut x = 5;
    x += 1;
    println!("x: {}", x);
    let y = 10;
    let z = x + y;
    println!("z: {}", z);

    // experiment with type annotations and type inference
    let a: i32 = 10; // explicit type annotation
    let b = 20; // type inference
    let c = a + b; // type inference
    println!("a: {}, b: {}, c: {}", a, b, c);
    

    let d: f64 = 15.5; // explicit type annotation
    let e = 4.5; // type inference
    let f = d + e; // type inference
    println!("d: {}, e: {}, f: {}", d, e, f);
    

    let is_active: bool = true; // explicit type annotation
    let status = if is_active { "Active" } else { "Inactive" }; // type inference
    println!("Status: {}", status);

    // program to convert between different types
    let int_value: i32 = 42;
    let float_value: f64 = int_value as f64; // convert int to float
    let string_value: String = int_value.to_string(); // convert int to string
    let char_value: char = 'A'; // character
    let char_to_int: i32 = char_value as i32; // convert char to int
    let int_to_char: char = 'B' as char; // convert int to char
    let float_to_int: i32 = float_value as i32; // convert float to int
    let int_to_float: f64 = int_value as f64; // convert int to float
    let string_to_int: i32 = string_value.parse().unwrap(); // convert string to int
    let int_to_string: String = int_value.to_string(); // convert int to string
    let string_to_float: f64 = string_value.parse().unwrap(); // convert string to float
    let float_to_string: String = float_value.to_string(); // convert float to string
    
    
    println!("int_value: {}", int_value);
    println!("float_value: {}", float_value);
    println!("string_value: {}", string_value);
    println!("char_value: {}", char_value);
    println!("char_to_int: {}", char_to_int);
    println!("int_to_char: {}", int_to_char);
    println!("float_to_int: {}", float_to_int);
    println!("int_to_float: {}", int_to_float);
    println!("string_to_int: {}", string_to_int);
    println!("int_to_string: {}", int_to_string);
    println!("string_to_float: {}", string_to_float);
    println!("float_to_string: {}", float_to_string);
    
}


