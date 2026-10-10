# Lifetimes: Borrowing Toys

A reference can never outlive the thing it points to. Lifetimes are how Rust checks that.

## The idea

Alice and Bob each own toys, and sometimes one borrows from the other. A borrow is only allowed while the toy still exists. Once a toy is gone, a reference to it would point at nothing, so Rust refuses to compile that code.

You've seen this rule before in other languages as a "dangling pointer" bug. Rust catches it at compile time.

## The three steps

1. **Functions.** Alice has a Robot and Bob has a Spaceship. `pick_longer` looks at both and hands back whichever name is longer:

   ```rust
   fn pick_longer<'a>(first: &'a str, second: &'a str) -> &'a str
   ```

   The answer could be Alice's toy or Bob's, so it's only safe to use while *both* toys still exist. That's what `'a` says: the result lives no longer than the shorter-lived input.

2. **Structs.** Bob borrows Alice's Robot and holds on to it in a `Borrower`:

   ```rust
   struct Borrower<'a> {
       toy: &'a str,
   }
   ```

   `'a` ties Bob to Alice's toy. Bob can hold the Robot only while Alice still has it.

3. **Elision.** Bob has a Teddy Bear and just calls it "Teddy". `first_word(text: &str) -> &str` has no `'a`: there's only one toy coming in, so the result must come from it, and Rust adds the lifetime for you.

## Run it

```sh
cargo run
```

Output:

```
Longer toy name: Spaceship
Bob is holding: Robot
Bob calls it: Teddy
```

## Try it yourself

1. Uncomment each `/* ... */` block in `main` one at a time and run `cargo run`. Each one fails because a toy is dropped while someone still needs it:

   ```
   `bob_toy` does not live long enough
   ```

   In step 1, Bob's Spaceship is dropped before `longer` is used. In step 2, Alice's Robot is dropped while Bob still holds it, so the error names `alice_toy`.

2. Remove `<'a>` and every `'a` from `pick_longer`. The compiler asks for a lifetime:

   ```
   missing lifetime specifier
   ```

3. Remove `<'a>` from `struct Borrower`. Same complaint: a struct holding a reference has to say how long that reference lives.
