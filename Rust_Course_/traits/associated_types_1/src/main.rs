use std::ops::Add;

#[derive(Debug)]

struct lunch {
    cost: f64,
}

impl Add for lunch {
    type Output = f64;

    fn add(self, rhs: Self) -> Self::Output {
        self.cost + rhs.cost
    }
}

fn main() {
    let lunch1 = lunch { cost: 10.5 };
    let lunch2 = lunch { cost: 15.0 };
    let total_cost = lunch1 + lunch2;
    println!("Total cost: {}", total_cost);
}
