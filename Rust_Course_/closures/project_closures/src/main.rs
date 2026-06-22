/*
Define a `SupermarketItem` struct with a `name` field
set to a String and a `price` field set to a f64.
Derive a Debug trait implementation.

Define a `ShoppingCart` struct with an `items` field
set to a vector of `SupermarketItem` structs.
Derive a Debug trait implementation.

---

Define a `traverse_items` method on the `ShoppingCart`.
It should accept a mutable reference to the instance
as well as an `operation` parameter that will be a
closure. Use a trait bound to mandate the closure implement
the `FnMut` trait. In addition, the closure should accept
a mutable reference to a `SupermarketItem`.

In the body of the `traverse_items` method, perform an
iteration over all `SupermarketItem`s in the `items` vector
using a `while` loop. For each vector element, invoke the
`operation` closure and pass in a mutable reference to the
`SupermarketItem` to the closure.

---

Define a `checkout` method on the `ShoppingCart`. It should
take ownership of the instance. It should also define an
`operation` parameter that will be a closure. Use a trait
bound to mandate the closure implement the `FnOnce` trait.
In addition, the closure should accept a single parameter of
a ShoppingCart.

In the body of the `checkout` method, invoke the `operation`
parameter and pass in the `ShoppingCart` instance.

---

In the body of `main`, create a `ShoppingCart` instance.
Populate its `items` vector with two `SupermarketItem`
instances. For the names of the items, provide capitalized
strings. Here's some sample data:
- { name: "APPLE", price: 3.99 }
- { name: "BANANA", price: 2.99 }

Invoke the `traverse_items` method and pass in a closure.
The closure will accept a mutable reference to each
`SupermarketItem` struct. Decrease the item's price by 15%
(you can multiply the price by 0.85 to accomplish this).

Invoke the `traverse_items` method again and pass in another
closure. This time around, take the `SupermarketItem`'s name,
lowercase it, and overwrite the existing `name` field value.

---

Declare a `total_price` variable initialized to 0.0.
Invoke the `checkout` method and pass in a closure.
The closure should receive mutable ownership of the
`ShoppingCart` struct. In the body of the closure, output
the cart in Debug format.

Then, call the `traverse_items` closure IN THIS closure.
Pass in a closure to `traverse_items`. The nested closure
should increment the `total_price` value so that it gathers
the cumulative sum of all of the `SupermarketItem`'s prices.

Print out the final value of the `total_price` variable
with a precision of 2 decimal points and a dollar sign
in front.
*/

#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SupermarketItem>,
}

impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F)
    where
        F: FnMut(&mut SupermarketItem),
    {
        let mut index = 0;
        while index < self.items.len() {
            operation(&mut self.items[index]);
            index += 1;
        }
    }

    fn checkout<F>(self, operation: F)
    where
        F: FnOnce(ShoppingCart),
    {
        operation(self);
    }
}

fn main() {
    let mut cart = ShoppingCart {
        items: vec![
            SupermarketItem {
                name: "APPLE".to_string(),
                price: 3.99,
            },
            SupermarketItem {
                name: "BANANA".to_string(),
                price: 2.99,
            },
        ],
    };

    cart.traverse_items(|item| {
        item.price *= 0.85;
    });

    cart.traverse_items(|item| {
        item.name = item.name.to_lowercase();
    });

    let mut total_price = 0.0;
    cart.checkout(|mut cart| {
        println!("{:?}", cart);
        cart.traverse_items(|item| {
            total_price += item.price;
        });
    });

    println!("Total Price: ${:.2}", total_price);
}
