fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}

fn main() {
    let availability = is_item_in_stock(true, true);
    match availability {
        Some(true) => println!("yes, in stock"),
        Some(false) => println!("No, not in stock"),
        None => println!("item is not in the system"),
    }
}

// Matching on Option
fn process_value(opt: Option<i32>) {
    match opt {
        Some(value) => println!("Got value: {}", value),
        None => println!("No value found"),
    }
}

// Function returning Option
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

// Using Some/None directly (no Option:: prefix needed)
let result = Some(42);
let empty = None;