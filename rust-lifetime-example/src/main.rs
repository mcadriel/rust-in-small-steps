// Bob doesn't own Alice's toy. He holds a magic hologram of it.
// `'a` says: the magic hologram can't outlive the toy it shows.
struct Borrower<'a> {
    toy: &'a str,
}

// The result is one of the two inputs, so it can only live
// as long as the shorter-lived of the two.
fn pick_longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

// One reference in, one reference out: Rust fills in the lifetime for you.
fn look_at(toy: &str) -> &str {
    toy
}

fn main() {
    // Step 1. Functions: the returned reference can't outlive either input.
    {
        let alice_toy = String::from("Robot");
        let bob_toy = String::from("Teddy Bear");

        let longer = pick_longer(&alice_toy, &bob_toy);
        println!("Longer toy name: {longer}");
    }

    // Uncomment this block: Bob's toy is gone before we use `longer`.
    /*
    let alice_toy = String::from("Robot");
    let longer;

    {
        let bob_toy = String::from("Teddy Bear");
        longer = pick_longer(&alice_toy, &bob_toy);
    } // Bob's toy is dropped here.

    println!("Longer toy name: {longer}");
    */

    // Step 2. Structs: Bob's magic hologram is only valid while Alice's toy exists.
    {
        let alice_toy = String::from("Robot");
        let bob = Borrower { toy: &alice_toy };

        println!("Bob is holding: {}", bob.toy);
    }

    // Uncomment this block: Bob still holds the magic hologram after the toy is gone.
    /*
    let bob;

    {
        let alice_toy = String::from("Robot");
        bob = Borrower { toy: &alice_toy };
    } // Alice's toy is dropped here.

    println!("Bob is holding: {}", bob.toy);
    */

    // Step 3. Elision: no `'a` needed when the rule is obvious.
    {
        let bob_toy = String::from("Teddy Bear");
        let seen = look_at(&bob_toy);

        println!("Bob looks at: {seen}");
    }
}
