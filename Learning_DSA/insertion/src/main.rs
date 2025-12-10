fn main() {
    let mut my_vec = vec![10, 20, 30, 40];
    println!("before push, my array was {:?}", my_vec);
    
    // Append to end (most common)
    my_vec.push(50);
    println!("after push, my array is {:?}", my_vec);
    
    // Insert at a specific index (shifts right)
    my_vec.insert(2, 25);
    println!("after inserting, my array is {:?}", my_vec);
    
    // Safe insert without panic (check bounds)
    // Always check bounds if index may be out of range to avoid runtime panic.
    let index = 10;
    let value = 95;
    
    if index <= my_vec.len() {
        my_vec.insert(index, value);
        println!("Inserted at {}: {:?}", index, my_vec);
    } else {
        println!("Index {} out of bounds (len = {}) — skipping insert", index, my_vec.len()); 
    }
    
    // Convert fixed-size array to Vec, then insert
    let fixed = [10, 20, 30];
    let mut v = fixed.to_vec();
    v.insert(0, 5);
    println!("Vec after insert: {:?}", v);
    
    // Insert keeping vector sorted (find index with binary search)
    let mut sorted = vec![1, 3, 5, 9, 11];
    let value = 7;
    // find insertion point using binary_search
    match sorted.binary_search(&value) {
        Ok(pos) => sorted.insert(pos, value), // equal element -> insert before it
        Err(insert_pos) => sorted.insert(insert_pos, value), // Err gives place to insert to keep sorted
    }
    println!("After sorted insert: {:?}", sorted);
    
}