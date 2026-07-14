/*
Let's imagine we're running an e-commerce store that
sells home appliances. Another developer has left
some starter code to work with.

The Product enum has 4 variants for the products
we sell: blender, microwave, toaster, and fridge.

The CustomerOrder struct represents an online order.
It stores the ordered Product, its quantity, and
whether we've shipped it to the customer.

The Customer struct represents a customer. Each
customer has a unique numeric ID and a vector of
their orders.
*/

#![allow(unused, dead_code)]

use std::collections::HashMap;
use std::env;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    // In `main`, we have an `orders` vector with the 6
    // orders in our system.
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    // We also have a `customer_ids_by_order` array that
    // lists the customer ID of each customer who placed
    // each of the 6 orders.
    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    // ---------------------------------------------------------
    // "Extract all the customer orders where the customer
    // ordered a Blender. Our goal is a vector of &CustomerOrder
    // values. Print out the vector. It should have 2 total orders."
    // ---------------------------------------------------------
    println!("=== Blender orders ===");
    let blender_orders: Vec<&CustomerOrder> = orders
        .iter()
        .filter(|order| order.product == Product::Blender)
        .collect();
    println!("{:#?}\n", blender_orders);

    // ---------------------------------------------------------
    // "The boss would like to know the total quantity of
    // microwaves ordered across all customer orders. Filter
    // for the customer orders where the Product is a
    // Microwave, extract the 'quantity' field for each
    // customer order, then calculate the sum of those
    // values. The answer should be 6."
    //
    // BONUS: solve with both (a) filter + map and
    // (b) filter_map
    // ---------------------------------------------------------
    println!("=== Total microwaves ordered ===");

    // (a) filter + map
    let total_microwaves_a: u32 = orders
        .iter()
        .filter(|order| order.product == Product::Microwave)
        .map(|order| order.quantity)
        .sum();
    println!("filter+map: {}", total_microwaves_a);

    // (b) filter_map
    let total_microwaves_b: u32 = orders
        .iter()
        .filter_map(|order| {
            if order.product == Product::Microwave {
                Some(order.quantity)
            } else {
                None
            }
        })
        .sum();
    println!("filter_map: {}\n", total_microwaves_b);

    // ---------------------------------------------------------
    // "The boss would like to pass in a quantity from the
    // command line. They want to see a printed vector of
    // all orders where the quantity is greater than or
    // equal to their input. e.g. 'cargo run -- 5' should
    // print the Microwave(5, true) and Fridge(10, false)
    // orders.
    //
    // If the boss does not provide a command-line argument
    // OR provides an invalid numeric value, fallback to
    // printing customer orders with quantity >= 2."
    // ---------------------------------------------------------
    println!("=== Orders above quantity threshold ===");
    let args: Vec<String> = env::args().collect();
    let min_quantity: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(2);

    let above_threshold: Vec<&CustomerOrder> = orders
        .iter()
        .filter(|order| order.quantity >= min_quantity)
        .collect();
    println!("threshold used: {}", min_quantity);
    println!("{:#?}\n", above_threshold);

    // ---------------------------------------------------------
    // "The boss would like to know how much inventory of
    // each product we need for unshipped orders. Create a
    // HashMap where each key is a &Product and each value
    // is the sum of that product's quantities across
    // unshipped orders. Should print:
    // {Fridge: 10, Toaster: 2, Blender: 4}"
    // ---------------------------------------------------------
    println!("=== Unshipped inventory needed ===");
    let mut inventory_needed: HashMap<&Product, u32> = HashMap::new();
    for order in &orders {
        if !order.shipped {
            *inventory_needed.entry(&order.product).or_insert(0) += order.quantity;
        }
    }
    println!("{:?}\n", inventory_needed);

    // ---------------------------------------------------------
    // "Our warehouse worker informs us they've shipped the
    // next unshipped order. Find the first unshipped order
    // among the customer orders and change its `shipped`
    // field to `true`. Print out the customer orders to
    // confirm."
    // ---------------------------------------------------------
    println!("=== Ship the first unshipped order ===");
    if let Some(order) = orders.iter_mut().find(|order| !order.shipped) {
        order.shipped = true;
    }
    println!("{:#?}\n", orders);

    // ---------------------------------------------------------
    // "THIS IS A TOUGH ONE. The boss would like to see a
    // vector of Customer structs. Each Customer struct will
    // hold the user's id and a vector of their orders. Find
    // a way to merge the customer orders with the customers
    // who made them, then aggregate the data into Customer
    // structs, then collect the Customers in a vector, then
    // sort the collection by customer id."
    //
    // NOTE: orders.into_iter() below CONSUMES `orders`, so this
    // has to be the last thing we do with it. Also worth flagging:
    // the block above already flipped the first unshipped order
    // (the Blender, qty 3) to shipped: true, and that order belongs
    // to customer 2. So this run's output shows that Blender as
    // shipped: true, not false like the original sample output —
    // same Vec, same program, the earlier mutation carries forward.
    // ---------------------------------------------------------
    println!("=== Customers with their orders ===");
    let mut by_customer: HashMap<u32, Vec<CustomerOrder>> = HashMap::new();

    for (order, customer_id) in orders.into_iter().zip(customer_ids_by_order.iter()) {
        by_customer
            .entry(*customer_id)
            .or_insert_with(Vec::new)
            .push(order);
    }

    let mut customers: Vec<Customer> = by_customer
        .into_iter()
        .map(|(id, orders)| Customer { id, orders })
        .collect();

    customers.sort_by_key(|c| c.id);

    println!("{:#?}", customers);
}
