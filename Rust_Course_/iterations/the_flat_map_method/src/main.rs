fn main() {
    let attendees = [
        "Bob, Mary, Kevin",
        "Mike, Robbie, Matt, Austin",
        "Piers, Liam",
    ];

    let attendees: Vec<&str> = attendees
        .iter()
        .map(|group| group.split(", "))
        .flatten()
        .collect();
    println!("{attendees:?}");
}
