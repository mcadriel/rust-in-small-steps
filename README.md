<p align="left">
  <img src="https://www.rust-lang.org/logos/rust-logo-128x128.png" alt="Rust logo" width="96">
   &nbsp;&nbsp;
  <img src="https://rustacean.net/assets/rustacean-flat-happy.svg" alt="Ferris, the Rust crab mascot" width="140">
</p>

<h1 align="left">Rust in Small Steps</h1>

<p align="left">Learn Rust one small, clear example at a time.</p>

A collection of simple, readable Rust examples designed to make sense just by looking at the code. Each example focuses on one idea, with clear names and straightforward logic.

Start small, run the code, and experiment at your own pace.

## Examples

| Example | Idea | You'll learn |
|---|---|---|
| [PhantomData](rust-phantomdata-example/) | Type-safe IDs | Use `PhantomData<T>` so the compiler stops you from mixing up a user ID and an order ID |
| [Lifetimes](rust-lifetime-example/) | Borrowing toys | Lifetimes in functions and structs, and when Rust fills them in for you (elision) |

Each folder has its own README that explains the idea, shows the expected output, and suggests things to try.

## How to use

You need Rust installed. If you don't have it yet, get it from [rustup.rs](https://rustup.rs).

Pick an example and run it:

```sh
cd rust-lifetime-example
cargo run
```

Then open `src/main.rs` and read it top to bottom. The examples are short on purpose.

## Experiment

Every example has code you can switch on to see what the compiler says:

- **Commented-out lines or `/* ... */` blocks.** Uncomment one and run `cargo run` again. Most of them are meant to fail, so you can read the compiler's error and see the rule it enforces.
- **"Try it yourself" in each README.** Small changes to make, like removing a lifetime or adding your own type.

Breaking the code on purpose is the fastest way to learn what Rust protects you from.
