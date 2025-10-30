#[derive(Debug)]

enum PaymentMethodTypes {
    CreditCard(String),
    DebitCard(String),
    Paypal(String),
}


fn main() {
    let visa = PaymentMethodTypes::CreditCard(String::from("0000-2222-3333-4444"));
    let mastercard = PaymentMethodTypes::DebitCard(String::from("0000-5555-6666-7777"));
    println!("This is the Vis: {:#?} and this is my mastercard: {:#?}", visa, mastercard);
    
    
}