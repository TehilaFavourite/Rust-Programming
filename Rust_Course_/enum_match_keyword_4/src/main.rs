#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed | OnlineOrderStatus::Shipped => {
                println!("Your order has been delivered");
            }
            other_status => {
                println!("Your order is not ready yet");
            }
        }
    }
}

fn main() {
    OnlineOrderStatus::Ordered.check();
}
