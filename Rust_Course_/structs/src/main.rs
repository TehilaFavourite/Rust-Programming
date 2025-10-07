#[derive(Debug)]
struct Tehila {
    age: u8,
    height: f64,
    is_happy: bool
}

struct Coffee {
    price: f64,
    name: String,
    is_hot: bool
}
fn main() {
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true
    };
    println!("The price of {} is ${}, and it is hot: {}", mocha.name, mocha.price, mocha.is_hot);
    let favorite_coffee = mocha.name;
    println!("My favorite coffee is {}", favorite_coffee);

    // overwriting a struct
    let latte = Coffee {
        name: String::from("Latte"),
        ..mocha
    };
    println!("The price of {} is ${}, and it is hot: {}", latte.name, latte.price, latte.is_hot);

    // creating a struct in a function
    let name = String::from("Cappuccino");
    let cappuccino = make_coffee(name, 3.99, true);
    println!("The price of {} is ${}, and it is hot: {}", cappuccino.name, cappuccino.price, cappuccino.is_hot);

    // struct update syntax
    let mocha = make_coffee(String::from("Mocha"), 4.99, true);
    let caramel = Coffee {
        name: String::from("Caramel"),
        ..mocha
    };
    println!("The price of {} is ${}, and it is hot: {}", caramel.name, caramel.price, caramel.is_hot);
    let iced_coffee = make_coffee(String::from("Iced Coffee"), 2.99, false);
    let caramel = Coffee {..iced_coffee};
    println!("The price of {} is ${}, and it is hot: {}", caramel.name, caramel.price, caramel.is_hot);
    // passing structs into a function
    drink_coffee(caramel);
    my_coffee(mocha);
    your_coffee(&latte);
    let mut espresso = make_coffee(String::from("Espresso"), 2.49, true);
    our_coffee(&mut espresso);

    let tehila = Tehila {
        age: 30,
        height: 1.65,
        is_happy: true
    };
    // println!("{:?}", tehila);
    tehila.new();

    let tehila2 = Tehila {
        age: 25,
        height: 1.70,
        is_happy: false
    };
    tehila2.double_age();

    let tehila3 = Tehila {
        age: 25,
        height: 1.70,
        is_happy: false
    };
    tehila3.double_height();

    let mut tehila4 = Tehila {
        age: 25,
        height: 1.70,
        is_happy: false
    };
    tehila4.mood();

    let new_traits = Tehila {
        age: 10,
        height: 1.2,
        is_happy: true
    };

    let custom_traits = Tehila {
        age: 20,
        height: 2.0,
        is_happy: true
    };

    if new_traits.is_taller(&custom_traits) {
        println!("{:?} is longer than {:?}", new_traits, custom_traits);
    } else {
        println!("this is false information");
    }

}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot
    }
}

// passing structs into a function
fn drink_coffee(coffee: Coffee) {
    println!("Drinking a {} which costs ${}", coffee.name, coffee.price);
    // this is defining the parameter that receives the struct as an immutable value.
}

fn my_coffee(mut coffee: Coffee) {
    coffee.price += 1.0;
    println!("The price of {} is now ${}", coffee.name, coffee.price);
    // this is defining the parameter that receives the struct as a mutable value.
}

fn your_coffee(coffee: &Coffee) {
    println!("The price of {} is ${}", coffee.name, coffee.price);
    // this is defining the parameter that receives the struct as a reference.
}

fn our_coffee(coffee: &mut Coffee) {
    coffee.price += 1.0;
    println!("The price of {} is now ${}", coffee.name, coffee.price);
    // this is defining the parameter that receives the struct as a mutable reference.
}

impl Tehila {
    fn new(self) {
        println!("Tehila is {} years old, {} meters tall, and is happy: {}", self.age, self.height, self.is_happy);
    } // here, self is the immutable struct instance

    fn double_age(mut self) {
        self.age = self.age * 2;
        println!("{:#?}", self);
    } // here is a mutable struct instance

    fn double_height(&self) {
        let doubled = self.height * 2.0;
        println!("Tehila's doubled height would be: {}", doubled);
        println!("Current Tehila: {:#?}", self);
    } // here is an immutable reference - can only read, not modify
    //
    fn mood(&mut self) {
        self.is_happy = true;
        println!("{:#?}", self);
    } // here is a mutable reference.

    // methods with multiple parameters
    fn is_taller(&self, other: &Self) -> bool {
        self.height > other.height
    }
}
