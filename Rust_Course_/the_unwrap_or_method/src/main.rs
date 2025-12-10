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
    let present_value = Some(13);
    let missing_value: Option<i32> = None;
    println!("{}", present_value.unwrap_or(0));
}

