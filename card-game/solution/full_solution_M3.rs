

#[cfg(test)]
mod tests {
    use crate::Card::{King, Queen};
    use crate::{Alice, Bob, Deck};

    #[test]
    fn test_cyclic_shift() {
        let mut deck = Deck {
            cards: vec![King, King, King, King, Queen],
        };
        deck.cyclic_shift(2);
        assert_eq!(deck.cards, vec![King, King, Queen, King, King]);
        deck.cyclic_shift(3);
        assert_eq!(deck.cards, vec![King, King, King, King, Queen]);
        assert_ne!(deck.cards, vec![King, Queen, King, King, King]);
    }

    #[test]
    fn test_full_game_var_shifts() {
        //true true
        let alice = Alice { secret_input: true };
        let bob = Bob { secret_input: true };

        let alice_deck = alice.encode_alice_input();
        let bob_deck = bob.encode_bob_input();
        let mut deck1 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck2 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck3 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck4 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck5 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());

        deck1.cyclic_shift(1);
        deck2.cyclic_shift(2);
        deck3.cyclic_shift(3);
        deck4.cyclic_shift(4);
        deck5.cyclic_shift(5);

        assert_eq!(deck1.decode(), true);
        assert_eq!(deck2.decode(), true);
        assert_eq!(deck3.decode(), true);
        assert_eq!(deck4.decode(), true);
        assert_eq!(deck5.decode(), true);

        //true false
        let alice = Alice { secret_input: true };
        let bob = Bob {
            secret_input: false,
        };

        let alice_deck = alice.encode_alice_input();
        let bob_deck = bob.encode_bob_input();
        let mut deck1 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck2 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck3 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck4 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck5 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());

        deck1.cyclic_shift(1);
        deck2.cyclic_shift(2);
        deck3.cyclic_shift(3);
        deck4.cyclic_shift(4);
        deck5.cyclic_shift(5);

        assert_eq!(deck1.decode(), false);
        assert_eq!(deck2.decode(), false);
        assert_eq!(deck3.decode(), false);
        assert_eq!(deck4.decode(), false);
        assert_eq!(deck5.decode(), false);

        //false true
        let alice = Alice {
            secret_input: false,
        };
        let bob = Bob { secret_input: true };

        let alice_deck = alice.encode_alice_input();
        let bob_deck = bob.encode_bob_input();
        let mut deck1 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck2 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck3 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck4 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck5 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());

        deck1.cyclic_shift(1);
        deck2.cyclic_shift(2);
        deck3.cyclic_shift(3);
        deck4.cyclic_shift(4);
        deck5.cyclic_shift(5);

        assert_eq!(deck1.decode(), false);
        assert_eq!(deck2.decode(), false);
        assert_eq!(deck3.decode(), false);
        assert_eq!(deck4.decode(), false);
        assert_eq!(deck5.decode(), false);

        //false false
        let alice = Alice {
            secret_input: false,
        };
        let bob = Bob {
            secret_input: false,
        };

        let alice_deck = alice.encode_alice_input();
        let bob_deck = bob.encode_bob_input();
        let mut deck1 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck2 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck3 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck4 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());
        let mut deck5 = Deck::join_decks(alice_deck.clone(), bob_deck.clone());

        deck1.cyclic_shift(1);
        deck2.cyclic_shift(2);
        deck3.cyclic_shift(3);
        deck4.cyclic_shift(4);
        deck5.cyclic_shift(5);

        assert_eq!(deck1.decode(), false);
        assert_eq!(deck2.decode(), false);
        assert_eq!(deck3.decode(), false);
        assert_eq!(deck4.decode(), false);
        assert_eq!(deck5.decode(), false);
    }
}
