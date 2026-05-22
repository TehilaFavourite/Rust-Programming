// returning_an_option_enum_from_a_function
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
        Option::Some(true) => println!("yes, in stock"),
        Option::Some(false) => println!("No, not in stock"),
        Option::None => println!("item is not in the system"),
    }
}