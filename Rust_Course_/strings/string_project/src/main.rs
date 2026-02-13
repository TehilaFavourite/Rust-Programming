use std::io;

fn main() {
    let mut cash = String::from("salary");
    make_money(&mut cash);
    println!("{cash}");

    let shout = trim_and_capitalize("     hallo Rust    ");
    println!("{shout}");

    let metals = elements("Gold!Silver!Platinum");
    println!("{:?}", metals);

    let name = get_identity();
    println!("{name}");
}

fn make_money(s: &mut String) {
    s.push_str("$$$");
}

fn trim_and_capitalize(s: &str) -> String {
    let trimmed = s.trim();
    trimmed.to_uppercase()
}

fn elements(s: &str) -> Vec<&str> {
    s.split('!').collect()
}

fn get_identity() -> String {
    let mut first = String::new();
    let mut last = String::new();

    println!("Enter your first name");
    io::stdin().read_line(&mut first).expect("failed to collect first nname");


    println!("Enter your last name");
    io::stdin().read_line(&mut last).expect("failed to collect last nname");

    format!("{} {}", first.trim(), last.trim())

}