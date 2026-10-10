# Lifetimes: Borrowing Toys

A reference can never outlive the thing it points to. Lifetimes are how Rust checks that.

## The idea

![Why lifetimes matter in Rust: Alice owns a toy robot. Bob holds a magic hologram of it. Once Alice's scope ends the robot is dropped, so Bob's magic hologram shows a toy that no longer exists and can no longer be used.](images/why-lifetimes-matter.png)

Alice owns a Robot. Bob doesn't take it. He holds a **magic hologram** of Alice's Robot. It's Bob's *reference*: it lets him see the toy without owning it.

A magic hologram is only real while the toy is still there. Once Alice leaves and the Robot is dropped, Bob's magic hologram would show a toy that no longer exists. Other languages call this a "dangling pointer" bug. Rust refuses to compile it.

## The three steps

1. **Functions.** Alice owns a Robot and Bob owns a Teddy Bear. `pick_longer` borrows both toy names and returns a reference to whichever name is longer:

   ```rust
   fn pick_longer<'a>(first: &'a str, second: &'a str) -> &'a str
   ```

   The answer could be Alice's toy or Bob's, so it's only safe to use while *both* toys still exist. That's what `'a` says: the result lives no longer than the shorter-lived input.

2. **Structs.** This is the picture above in code. `Borrower` is Bob holding his magic hologram, and `toy` is the magic hologram of Alice's Robot:

   ```rust
   struct Borrower<'a> {
       toy: &'a str,
   }
   ```

   `'a` ties Bob's magic hologram to Alice's toy. Bob can keep the magic hologram only while Alice still has the Robot.

3. **Elision.** Bob has a Teddy Bear and looks at it. `look_at(toy: &str) -> &str` has no `'a`: there's only one toy coming in, so the result must come from it, and Rust adds the lifetime for you. The function just returns `toy` on purpose, so the signature is the only thing to look at.

## Run it

```sh
cargo run
```

Output:

```
Longer toy name: Teddy Bear
Bob is holding: Robot
Bob looks at: Teddy Bear
```

## Try it yourself

1. Uncomment each `/* ... */` block in `main` one at a time and run `cargo run`. Each one fails because a toy is dropped while someone still needs it:

   ```
   `bob_toy` does not live long enough     (step 1)
   `alice_toy` does not live long enough   (step 2)
   ```

   In step 1, Bob's Teddy Bear is dropped before `longer` is used. In step 2, Alice's Robot is dropped while Bob still holds a magic hologram of it.

2. Remove `<'a>` and every `'a` from `pick_longer`. The compiler asks for a lifetime:

   ```
   missing lifetime specifier
   ```

3. Remove `<'a>` from `struct Borrower`. Same complaint: if Bob holds a magic hologram, Rust needs to know how long the toy it shows will be there.
