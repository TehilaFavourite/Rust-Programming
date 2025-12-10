fn main() {
    // calculating statistics from an array of numbers
    let numbers = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let mean = calculate_mean(&numbers);
    let mut numbers_clone = numbers.clone();
    let median = calculate_median(&mut numbers_clone);
    // let mode = calculate_mode(&numbers);
    let variance = calculate_variance(&numbers, mean);
    let std_dev = calculate_standard_deviation(variance);
    let range = calculate_range(&numbers);
    
    println!("Mean: {}", mean);
    println!("Median: {}", median);
    // println!("Mode: {:?}", mode);
    println!("Variance: {}", variance);
    println!("Standard Deviation: {}", std_dev);
    println!("Range: {}", range);
    
}

fn calculate_mean(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}
fn calculate_median(numbers: &mut [f64]) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = numbers.len();
    if len % 2 == 0 {
        (numbers[len / 2 - 1] + numbers[len / 2]) / 2.0
    } else {
        numbers[len / 2]
    }
}
// fn calculate_mode(numbers: &[i32]) -> Vec<i32> {
//     use std::collections::HashMap;

//     let mut occurrences = HashMap::new();
//     for &num in numbers {
//         *occurrences.entry(num).or_insert(0) += 1;
//     }

//     let max_count = occurrences.values().cloned().max().unwrap_or(0);
//     occurrences
//         .into_iter()
//         .filter(|&(_, count)| count == max_count)
//         .map(|(num, _)| num)
//         .collect()
// }
fn calculate_variance(numbers: &[f64], mean: f64) -> f64 {
    let sum_of_squares: f64 = numbers.iter().map(|&x| (x - mean).powi(2)).sum();
    sum_of_squares / numbers.len() as f64
}
fn calculate_standard_deviation(variance: f64) -> f64 {
    variance.sqrt()
}
fn calculate_range(numbers: &[f64]) -> f64 {
    let min = numbers.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = numbers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    max - min
}

