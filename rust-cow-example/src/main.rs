// Cow means clone-on-write. It holds either a borrow of someone else's data
// or its own owned copy, and it only clones when you ask to write.
use std::borrow::Cow;

// Tells us which side the Cow is on right now.
fn state(cow: &Cow<str>) -> &'static str {
    match cow {
        Cow::Borrowed(_) => "Borrowed, no clone",
        Cow::Owned(_) => "Owned, cloned",
    }
}

fn main() {
    let original = String::from("Rust");

    // Borrow: the Cow points at `original`. Nothing is cloned.
    let mut text: Cow<str> = Cow::Borrowed(&original);
    println!("Borrow: {text} -> {}", state(&text));

    // Read: just look at it. Still no clone needed.
    println!("Read:   {} letters -> {}", text.len(), state(&text));

    // Mutate: to_mut() clones the borrowed data into an owned copy,
    // then hands us a `&mut String` to write to.
    text.to_mut().push_str(" is fun");
    println!("Mutate: {text} -> {}", state(&text));

    // Already owned? to_mut() uses the existing value. No second clone.
    text.to_mut().push('!');
    println!("Again:  {text} -> {}", state(&text));

    // The original was never touched.
    println!("Original is still: {original}");
}
