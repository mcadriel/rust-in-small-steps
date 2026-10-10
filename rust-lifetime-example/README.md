# Lifetimes: Borrowing Toys

A reference can never outlive the thing it points to. Lifetimes are how Rust checks that.

## The idea

![Why lifetimes matter in Rust: Alice owns a toy robot. Bob holds a map that points to it. Once Alice's scope ends the robot is dropped, so Bob's map points to nothing and can no longer be used.](images/why-lifetimes-matter.png)

Alice owns a Robot. Bob doesn't take it. He holds a **map** that shows where Alice's Robot is. The map is Bob's *reference*: it lets him find the toy without owning it.

A map is only useful while the toy is still there. Once Alice leaves and the Robot is dropped, Bob's map points to nothing. Rust spots this and refuses to compile the code.

You've seen this rule before in other languages as a "dangling pointer" bug. Rust catches it at compile time.

## The three steps

1. **Functions.** Alice has a Robot and Bob has a Map. Here the Map is just Bob's own toy (in step 2, the map becomes his reference). `pick_longer` looks at both and hands back whichever name is longer:

   ```rust
   fn pick_longer<'a>(first: &'a str, second: &'a str) -> &'a str
   ```

   The answer could be Alice's toy or Bob's, so it's only safe to use while *both* toys still exist. That's what `'a` says: the result lives no longer than the shorter-lived input.

2. **Structs.** This is the picture above in code. `Borrower` is Bob holding his map, and `toy` is the map pointing at Alice's Robot:

   ```rust
   struct Borrower<'a> {
       toy: &'a str,
   }
   ```

   `'a` ties Bob's map to Alice's toy. Bob can keep the map only while Alice still has the Robot.

3. **Elision.** Bob has a Teddy Bear and looks at it. `look_at(toy: &str) -> &str` has no `'a`: there's only one toy coming in, so the result must come from it, and Rust adds the lifetime for you. The function just returns `toy` on purpose, so the signature is the only thing to look at.

## Run it

```sh
cargo run
```

Output:

```
Longer toy name: Robot
Bob is holding: Robot
Bob looks at: Teddy Bear
```

## Try it yourself

1. Uncomment each `/* ... */` block in `main` one at a time and run `cargo run`. Each one fails because a toy is dropped while someone still needs it:

   ```
   `bob_toy` does not live long enough
   ```

   In step 1, Bob's Map is dropped before `longer` is used. In step 2, Alice's Robot is dropped while Bob still holds a map to it, so the error names `alice_toy`.

2. Remove `<'a>` and every `'a` from `pick_longer`. The compiler asks for a lifetime:

   ```
   missing lifetime specifier
   ```

3. Remove `<'a>` from `struct Borrower`. Same complaint: if Bob holds a map, Rust needs to know how long the toy it points to will be there.
