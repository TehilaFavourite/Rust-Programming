mod inventory {
    const FLOOR_SPACE: i32 = 10000;
    pub const MANAGER: &str = "Ivan";

    enum ProductCategory {
        Ladder,
        Hammer,
    }

    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }

    // Public so `main` can call it.
    pub fn talk_to_manager() {
        println!("Hey {}", MANAGER);
    }
}

fn main() {
    println!("The manager is {}", inventory::MANAGER);
    
}

