fn main() {
    let spreadsheet = vec![[100, 200, 300], [123, 456, 789], [1, 2, 3]];

    let values: Vec<i32> = spreadsheet
        .into_iter()
        .flatten()
        .collect(); 
    println!("{values:?}");
}
