fn main() {
    let first_name = String::from("Tehila");
    let capture_first_name = move || { 
        println!("first name: {first_name}");
     };
    capture_first_name();
}
