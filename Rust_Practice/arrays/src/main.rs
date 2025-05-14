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
    
}
