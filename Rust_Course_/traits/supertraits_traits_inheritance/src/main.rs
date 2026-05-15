trait Investment {
    fn amount(&self) -> f64;

    fn set_amount(&mut self, new_amount: f64);

    fn double_amount(&mut self){
        self.set_amount(self.amount() * 2.0);
    }
}

trait Taxable: Investment {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    value: f64,
}

impl Investment for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;
}

struct QualityTime {
    minutes: f64,
}

impl Investment for QualityTime {
    fn amount(&self) -> f64 {
        self.minutes
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.minutes = new_amount;
    }
}

fn main() {
    let mut income = Income { amount: 1000.0 };
    println!("Tax bill: ${:.}", income.tax_bill());
    income.double_amount();
    println!("Tax bill after doubling: ${:.}", income.tax_bill());

    let mut bonus = Bonus { value: 1500.0 };
    println!("Tax bill: ${:.}", bonus.tax_bill());
    bonus.double_amount();
    println!("Tax bill after doubling: ${:.}", bonus.tax_bill());

    let weekend = QualityTime { minutes: 120.0 };
    println!("Weekend quality time: {} minutes", weekend.amount());
}
