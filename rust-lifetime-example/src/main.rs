// Bob doesn't own Alice's toy. He holds a magic hologram of it.
// `'a` says: the magic hologram can't outlive the toy it shows.
struct Borrower<'a> {
    toy: &'a str,
}

fn main() {
    // Works: Bob's magic hologram is only used while Alice's toy exists.
    {
        let alice_toy = String::from("Robot");
        let bob = Borrower { toy: &alice_toy };

        println!("Bob's magic hologram shows: {}", bob.toy);
    } // Alice's toy is dropped here.

    // Uncomment this block: Bob uses the magic hologram after the toy is gone.
    /*
    let bob;

    {
        let alice_toy = String::from("Robot");
        bob = Borrower { toy: &alice_toy };
    } // Alice's toy is dropped here.

    println!("Bob's magic hologram shows: {}", bob.toy);
    */
}
