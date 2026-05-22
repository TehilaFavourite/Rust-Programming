fn main() {
    // dangling ref is a reference that points to data that has been dropped.
    let cities = vec!["New York", "Los Angeles", "Chicago"];

    let fav_city = &cities[0..2]; // fav_city is a reference to the first two elements of the cities vector
    drop(cities); // drop the cities vector, which means that fav_city is now a dangling reference
    // println!("My favorite cities are: {:?}", fav_city); // This would cause a compile error
}
