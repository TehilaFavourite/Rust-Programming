fn main() {
    let number = 12;
    let capture_number = || number;
    let a = capture_number();
    let b = capture_number();
    println!("a: {a}, b: {b}");

    let first_name = String::from("Tehila");
    let capture_first_name = || first_name;
    println!("first name: {}", capture_first_name());

}
