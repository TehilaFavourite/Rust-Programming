#[derive(Debug)]

struct Credentials {
    Username: String,
    Password: String,
}

#[derive(Debug)]

enum PaymentMethodTypes {
    CreditCard(String),
    DebitCard(String),
    Paypal(Credentials),
    Crypto{Wallet: String, Address: String}
}


fn main() {
    let paypal_credentials = Credentials{
    Username: String::from("myemail@gmail.com"), 
    Password: String::from("myPassword")
    };
    let paypal = PaymentMethodTypes::Paypal(paypal_credentials);
    
    println!("this is my payment info: {:#?}", paypal);
    
    let crypto_payment = PaymentMethodTypes::Crypto{
        Wallet: String::from("Metamask"),
        Address: String::from("0X00000"),
    };
    println!("this is my crypto payment: {:#?}", crypto_payment);
    
    
}