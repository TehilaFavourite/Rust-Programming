#[derive(Debug)]

enum PaymentMethodTypes {
    CreditCard(String),
    DebitCard(String),
    Paypal(String, String),
}


fn main() {
    let mut my_payment_method = PaymentMethodTypes::DebitCard(String::from("0000-1111-2222-3333"));
    my_payment_method = PaymentMethodTypes::Paypal(String::from("myemail@gmail.com"), String::from("myPassword"));
    
    println!("this is my payment info: {:#?}", my_payment_method);
    
}