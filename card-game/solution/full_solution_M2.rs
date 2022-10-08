
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Card {
    Queen,
    King,
}

pub struct Alice {
    pub secret_input: bool,
}

pub struct Bob {
    pub secret_input: bool,
}

#[derive(Clone, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Alice {
    // "queen" = false , "king" = true
    pub fn encode_alice_input(self) -> Deck {
        match self.secret_input {
            true => Deck {
                cards: vec![Card::Queen, Card::King],
            },
            false => Deck {
                cards: vec![Card::King, Card::Queen],
            },
        }
    }
}

impl Bob {
    pub fn encode_bob_input(self) -> Deck {
        match self.secret_input {
            true => Deck {
                cards: vec![Card::King, Card::Queen],
            },
            false => Deck {
                cards: vec![Card::Queen, Card::King],
            },
        }
    }
}

impl Deck {
    pub fn cyclic_shift(&mut self, shift: usize) {
        assert_eq!(self.cards.len(), 5);

        self.cards.rotate_left(shift % 5);
    }

    pub fn join_decks(deck_alice: Deck, deck_bob: Deck) -> Self {
        // we will inject a separator "public card" = ("king" = true)
        let mut new_dec = deck_alice;
        new_dec.cards.extend(vec![Card::King]);
        new_dec.cards.extend(deck_bob.cards);
        new_dec
    }
    // open deck. If there are 3 "kings" in a row (= 3 straight  true values) return "match" (true),
    // else return false ("no match")
    pub fn decode(self) -> bool {
        let mut opened_deck = self;
        assert_eq!(opened_deck.cards.len(), 5);
        let first = opened_deck
            .cards
            .iter()
            .position(|&x| x == Card::Queen)
            .unwrap();
        opened_deck.cyclic_shift(first);
        return if opened_deck.cards[1] == Card::Queen || opened_deck.cards[4] == Card::Queen {
            true
        } else {
            false
        }
    }
}
