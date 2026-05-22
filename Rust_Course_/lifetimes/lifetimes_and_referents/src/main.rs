fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2] // selected_items is a reference to the first two elements of the items vector
}

fn main() {
    let items = vec!["New York".into(), "Los Angeles".into(), "Chicago".into()];
    let selected_items = {
        let cities_refernce = &items; // cities_reference is a reference to the items vector
        select_first_two_elements(cities_refernce)
    };

    println!("Selected items: {:?}", selected_items);

    {
        let coffees = vec!["Espresso".into(), "Latte".into(), "Cappuccino".into()];
        let selected_coffees = select_first_two_elements(&coffees);
        println!("Selected coffees: {:?}", selected_coffees); // This is fine because selected_coffees is still valid
    }
}
