mod utility;

fn main() {
    let alice = utility::Alice {interested: true};
    let bob = utility::Bob {interested: true};

    let alice_cards = alice.encode();
    let bob_cards = bob.encode();

    let mut card_deck = utility::CardDeck { cards: vec![
        utility::Card::Queen,
        utility::Card::King,
        utility::Card::King,
        utility::Card::King,
        utility::Card::Queen
    ]
    };

    let _ = utility::CardDeck::join(alice_cards, bob_cards);
    let _ = card_deck.cyclic_shift(2);
}
