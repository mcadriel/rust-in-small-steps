// Marker types: they are never created, they only label an `Id`.
use std::marker::PhantomData;

struct User;
struct Order;

struct Id<T> {
    number: u64,
    marker: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(number: u64) -> Self {
        Self {
            number,
            // `T` isn't stored anywhere, so PhantomData tells the compiler
    // "this Id belongs to T" without taking up any space.
            marker: PhantomData,
        }
    }
}

fn find_user(id: Id<User>) {
    println!("Finding user {}", id.number);
}

fn main() {
    // Same number, different types.
    let user_id = Id::<User>::new(42);
    let order_id = Id::<Order>::new(42);

    find_user(user_id);

    // Try it: uncomment the line below.
    // The compiler stops you: expected `Id<User>`, found `Id<Order>`.
    // find_user(order_id);

}
