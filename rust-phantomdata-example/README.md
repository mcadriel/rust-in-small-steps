# PhantomData: Type-Safe IDs

Use the type system to stop you from mixing up IDs that look the same.

## The idea

A user ID and an order ID are both just numbers. If both are plain `u64`, nothing stops you from passing an order ID where a user ID belongs.

This example gives each ID a label:

```rust
struct Id<T> {
    number: u64,
    marker: PhantomData<T>,
}
```

`Id<User>` and `Id<Order>` hold the same kind of number, but Rust treats them as different types. A function that asks for `Id<User>` won't accept `Id<Order>`.

## Why `PhantomData`?

`T` is never stored in `Id`. Rust doesn't allow a type parameter that isn't used, so `PhantomData<T>` marks it as used. It takes up no space at runtime.

## Run it

```sh
cargo run
```

Output:

```
Finding user 42
```

## Try it yourself

1. In `main`, uncomment `find_user(order_id);` and run `cargo run` again. The compiler stops you:

   ```
   expected `Id<User>`, found `Id<Order>`
   ```

2. Delete the `marker` field (from the struct and from `new`) and see what the compiler says about `T`.

3. Add your own type, like `struct Product;`, and write a `find_product` function that only accepts `Id<Product>`.
