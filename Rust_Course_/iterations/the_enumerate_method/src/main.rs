fn main() {
    let applicants = vec!["Alice", "Bob", "Charlie", "David", "Eve", "Robb"];

    let winners = applicants
        .into_iter()
        .enumerate()
        .filter_map(|(index, applicant)| {
            if index % 2 == 0 {
                Some(applicant)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!("Winners: {:?}", winners);
}
