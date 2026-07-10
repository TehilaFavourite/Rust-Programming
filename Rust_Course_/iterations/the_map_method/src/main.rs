fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let my_iterator = numbers.into_iter();
    let squares = my_iterator.map(|number: i32| number.pow(2));

    println!("{squares:?}");

    for number in squares {
        println!("{number}");
    }
}
