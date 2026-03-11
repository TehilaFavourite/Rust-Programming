    pub const FLOOR_SPACE: i32 = 10000;
    pub const MANAGER: &str = "Ivan";

    enum ProductCategory {
        pub Ladder,
        pub Hammer,
    }

    struct Item {
        pub name: String,
        pub category: ProductCategory,
        pub quantity: u32,
    }

    // Public so `main` can call it.
    pub fn talk_to_manager() {
        println!("Hey {}", MANAGER);
    }
