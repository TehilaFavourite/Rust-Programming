trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64;
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Taxable for Income {
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Bonus {
    amount: f64,
}

impl Taxable for Bonus {

    const TAX_RATE: f64 = 0.50;
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}

fn main() {
    let income = Income { amount: 1000.0 };
    println!("Tax bill: ${:.}", income.tax_bill());

    let bonus = Bonus { amount: 1500.0 };
    println!("Tax bill: ${:.}", bonus.tax_bill());
}
