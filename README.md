<p align="center">“Learning Rust starts with understanding how the code works—and why it behaves that way.”</p>

<h1 align="left">
  <img src="images/rust-in-small-steps-banner.png" alt="Rust in Small Steps" width="100%">
</h1>

<p align="left">Learn Rust one small, clear example at a time.</p>

## Introduction

AI can generate code quickly, but we still need to understand what we build. In Rust, that means understanding how ownership, borrowing, lifetimes, and error handling affect the way our programs behave. Code that compiles is a starting point; we also need to know whether it does what we intend.

**Rust in Small Steps** is for experienced engineers who are new to Rust. Each example focuses on one idea, using clear names and straightforward logic to help you connect the code with the rules behind it.

Use AI as a learning partner. Ask it to explain a compiler error, suggest an alternative, or challenge your understanding. Then run the code, test its suggestions, and make sure you can explain the result yourself.

Start small, experiment at your own pace, and build understanding one step at a time.

## Examples

| Example | Idea | You'll learn |
|---|---|---|
| [PhantomData](rust-phantomdata-example/) | Type-safe IDs | Use `PhantomData<T>` so the compiler stops you from mixing up a user ID and an order ID |
| [Lifetimes](rust-lifetime-example/) | Borrowing toys | Why a struct holding a reference needs a lifetime, and why Rust rejects using it after its data is gone |
| [Cow](rust-cow-example/) | Clone-on-write | Borrow data without cloning, and let `to_mut()` clone borrowed data when you need to change it |

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

- **Commented-out lines or `/* ... */` blocks.** Uncomment one and run `cargo run` again. Most are meant to fail, so you can read the compiler's error and see the rule it enforces.
- **"Try it yourself" in each README.** Make small changes, like removing a lifetime or adding your own type.

Breaking the code on purpose helps you learn what Rust protects you from. Before running a change, predict what will happen. Then compare your prediction with the result.

## Support Rust in Small Steps ⭐

If this repository helps you learn Rust, consider giving it a star! It shows your support and may help more developers discover these resources.
