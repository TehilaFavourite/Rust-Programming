#[derive(Debug)]
enum CheeseSteak<T> {
    Plain,
    Topping(T)
}

fn main() {
    let mushroom = CheeseSteak::Topping("Mushroom");
    println!("mushroom steak {:?}", mushroom);
    
    let onions = CheeseSteak::Topping("Onions".to_string());
    println!("onions steak {:?}", onions);
    
    let topping = "bacon".to_string();
    println!("bacon steak {:?}", topping);
    
    let bacon = CheeseSteak::Topping(&topping);
    println!("bacon steak {:?}", bacon);
    
    let mut plain:CheeseSteak<String> = CheeseSteak::Plain;
    println!("plain steak {:?}", plain);

    plain = CheeseSteak::Topping("Sausage".to_string());
    println!("plain steak {:?}", plain);
}