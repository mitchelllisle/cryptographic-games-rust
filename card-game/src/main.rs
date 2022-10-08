mod utility;

fn main() {
    let alice = utility::Alice {interested: true};
    let bob = utility::Bob {interested: true};

    let alice_shift = 3;
    let bob_shift = 2;

    let alice_cards = alice.encode();
    let bob_cards = bob.encode();

    let mut deck= utility::CardDeck::join(alice_cards, bob_cards);

    deck.cyclic_shift(bob_shift);
    deck.cyclic_shift(alice_shift);

    let response = deck.decode();

    match response {
        true => println!("MATCH"),
        false => println!("NO MATCH")
    };

}
