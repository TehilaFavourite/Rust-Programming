fn main() {
    // initialize an array with different methods
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let arr3 = [1; 5]; // all elements are initialized to 1
    println!("{:?}, {:?}, {:?}", arr1, arr2, arr3);
    // accessing elements of an array
    let first = arr1[0];
    let second = arr1[1];
    let third = arr1[2];
    println!("first: {}, second: {}, third: {}", first, second, third);
    // access array elements safely with bounds checking
    if let Some(fourth) = arr1.get(3) {
        println!("fourth: {}", fourth);
    } else {
        println!("Index out of bounds");
    }
    // iterating over an array
    for i in 0..arr1.len() {
        println!("arr1[{}]: {}", i, arr1[i]);
    }
    // iterating over an array with for loop
    for &item in arr1.iter() {
        println!("item: {}", item);
    }
    // iterating over an array with while loop
    let mut i = 0;
    while i < arr1.len() {
        println!("arr1[{}]: {}", i, arr1[i]);
        i += 1;
    }
}
