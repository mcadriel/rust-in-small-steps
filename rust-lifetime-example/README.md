# Lifetimes: Borrowing Toys

A reference can never outlive the thing it points to. Lifetimes are how Rust checks that.

## The idea

![Why lifetimes matter in Rust: Alice owns a toy robot. Bob holds a magic hologram of it. Once Alice's scope ends the robot is dropped, so Bob's magic hologram shows a toy that no longer exists and can no longer be used.](images/why-lifetimes-matter.png)

Alice owns a Robot. Bob doesn't take it. He holds a **magic hologram** of Alice's Robot. It's Bob's *reference*: it lets him see the toy without owning it.

A magic hologram is only real while the toy is still there. Once Alice leaves and the Robot is dropped, Bob's magic hologram would show a toy that no longer exists. Other languages call this a "dangling pointer" bug. Rust refuses to compile it.

## The code

The code is in [`src/main.rs`](src/main.rs). It's the picture above in code: `Borrower` is Bob holding his magic hologram, and `toy` is the magic hologram of Alice's Robot.

```rust
struct Borrower<'a> {
    toy: &'a str,
}
```

`'a` is a *lifetime parameter*: a name for "how long Bob's magic hologram can be used". Alice's toy must live at least that long. Read it as "tick a".

`'a` ties Bob's magic hologram to Alice's toy. It doesn't keep the Robot alive. It only lets Rust check that Bob never uses the hologram after the Robot is gone.

In `main` ([lines 9–14](src/main.rs#L9-L14)), Bob uses his magic hologram inside the same `{ }` where Alice's Robot lives, so it compiles.

## Run it

```sh
cargo run
```

Output:

```
Bob's magic hologram shows: Robot
```

## Try it yourself

1. In [`src/main.rs`](src/main.rs#L17-L26), uncomment the block on lines 17–26 by deleting the `/*` on line 17 and the `*/` on line 26:

   ```rust
   let bob;

   {
       let alice_toy = String::from("Robot");
       bob = Borrower { toy: &alice_toy };
   } // Alice's toy is dropped here.

   println!("Bob's magic hologram shows: {}", bob.toy);
   ```

   Run `cargo run`. Alice's Robot is dropped, then Bob uses his magic hologram of it, so Rust refuses to compile:

   ```
   `alice_toy` does not live long enough
   ```

2. In the same block, delete the `println!` line at the end ([line 25](src/main.rs#L25) in the original file):

   ```rust
   println!("Bob's magic hologram shows: {}", bob.toy);
   ```

   Run `cargo run` again. It compiles, with only warnings about `bob` being unused. Holding a magic hologram of a toy that's gone is fine. *Using* it is what Rust stops. In Rust's terms, a reference's lifetime ends at its last use, not at the closing `}`.

3. In [`src/main.rs`](src/main.rs#L3-L5), remove both `'a`s from `struct Borrower` on lines 3–5. Change this:

   ```rust
   struct Borrower<'a> {
       toy: &'a str,
   }
   ```

   to this:

   ```rust
   struct Borrower {
       toy: &str,
   }
   ```

   Run `cargo run`. The compiler asks for a lifetime:

   ```
   missing lifetime specifier
   ```

   If Bob holds a magic hologram, Rust needs to know how long the toy it shows will be there.
