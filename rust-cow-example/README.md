# Cow: Clone-on-Write

Clone only when borrowed data needs mutation.

## The idea

![Cow in Rust: clone only when borrowed data needs mutation. Borrow: Cow::Borrowed points at the original. Read: no clone needed. Mutate: to_mut() clones the borrowed data into an owned copy. Already owned? to_mut() uses the existing value. Cow means clone-on-write.](images/cow-in-rust.png)

Most of the time you only need to *look* at a piece of data. Cloning it just in case you might change it later is wasted work. But if you only borrow it, you can't change it at all.

`Cow` gives you both. It's an enum with two sides:

- `Cow::Borrowed(&data)` points at someone else's data. Reading is free, nothing is cloned.
- `Cow::Owned(data)` holds its own copy that you can change.

You start on the **Borrow** side. You **Read** as much as you like. The moment you need to **Mutate**, you call `to_mut()`, and Cow clones the borrowed data into an owned copy and hands you a `&mut` to it. That's the "clone-on-write" in the name.

If the Cow is already owned, `to_mut()` just gives you the existing value. It never clones twice.

## The code

The code is in [`src/main.rs`](src/main.rs). It walks through the three panels of the picture, in order, and prints which side the Cow is on after each step.

```rust
let original = String::from("Rust");

// Borrow: the Cow points at `original`. Nothing is cloned.
let mut text: Cow<str> = Cow::Borrowed(&original);

// Read: just look at it. Still no clone needed.
println!("{} letters", text.len());

// Mutate: to_mut() clones the borrowed data, then lets us write.
text.to_mut().push_str(" is fun");
```

`Cow<str>` is short for `Cow<'_, str>`: a Cow that either borrows a `&str` or owns a `String`. The `state` helper on [lines 6–11](src/main.rs#L6-L11) matches on the two sides so you can watch the switch happen.

## Run it

```sh
cargo run
```

Output:

```
Borrow: Rust -> Borrowed, no clone
Read:   4 letters -> Borrowed, no clone
Mutate: Rust is fun -> Owned, cloned
Again:  Rust is fun! -> Owned, cloned
Original is still: Rust
```

The first two lines never clone. The third line is where `to_mut()` clones. The fourth line calls `to_mut()` again, but the Cow is already owned, so it reuses the same `String`. The last line shows the original was never touched: the Cow wrote to its *copy*.

## Try it yourself

1. A `Cow::Borrowed` is a real borrow, so the usual borrowing rules apply. Change `let original` on [line 14](src/main.rs#L14) to `let mut original`, then add this line right after the `Borrow` `println!` on [line 18](src/main.rs#L18):

   ```rust
   original.push_str("?");
   ```

   Run `cargo run`. `text` still borrows `original` and is used again on the next line, so Rust refuses to compile:

   ```
   cannot borrow `original` as mutable because it is also borrowed as immutable
   ```

   Now move that same line to just above `// The original was never touched.` on [line 32](src/main.rs#L32). It compiles, because `text` is never used after that point, so its borrow of `original` has already ended. Undo both changes before step 2.

2. On [line 17](src/main.rs#L17), start on the owned side instead. Change this:

   ```rust
   let mut text: Cow<str> = Cow::Borrowed(&original);
   ```

   to this:

   ```rust
   let mut text: Cow<str> = Cow::Owned(original.clone());
   ```

   Run `cargo run`. Every line now says `Owned, cloned`, and `to_mut()` never clones. The Cow had its own value from the start, so there was nothing to copy. Put the `Borrowed` line back before step 3.

3. The most common use of `Cow` is a function that *usually* doesn't need to change its input. Add this below `state` on [line 11](src/main.rs#L11):

   ```rust
   // Adds a `!` only if the text doesn't already end with one.
   fn add_bang(text: &str) -> Cow<'_, str> {
       if text.ends_with('!') {
           Cow::Borrowed(text)
       } else {
           Cow::Owned(format!("{text}!"))
       }
   }
   ```

   Then call it in `main`, just above `// The original was never touched.`:

   ```rust
   println!("add_bang(\"Hi!\") -> {}", state(&add_bang("Hi!")));
   println!("add_bang(\"Hi\")  -> {}", state(&add_bang("Hi")));
   ```

   Run `cargo run`. You'll see two new lines:

   ```
   add_bang("Hi!") -> Borrowed, no clone
   add_bang("Hi")  -> Owned, cloned
   ```

   When the input is already fine, the function hands it straight back without allocating. Only the input that needs a change pays for a new `String`. The caller doesn't care which side it got: both print and read the same way.
