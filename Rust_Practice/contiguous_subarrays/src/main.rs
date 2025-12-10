fn find_subarrays_with_sum(arr: &[i32], target: i32) {
    for start in 0..arr.len() {
        let mut sum = 0;
        for end in start..arr.len() {
            sum += arr[end];
            if sum == target {
                // Print the subarray from start to end
                let subarray: Vec<i32> = arr[start..=end].to_vec();
                println!("{:?}", subarray);
            }
        }
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 2];
    let target_sum = 5;

    println!("Subarrays with sum {}:", target_sum);
    find_subarrays_with_sum(&numbers, target_sum);
}
