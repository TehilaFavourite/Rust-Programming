fn main() {
    // let random_float: f64 = rand::random();
    let random_float = rand::random::<f64>();
    println!("{}", random_float * 100.0);

    let random_int = rand::random::<u8>();
    println!("{}", random_int);
}
