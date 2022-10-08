#[derive(Clone, PartialEq)]
pub enum Card {
    Queen,
    King,
}

#[derive(Clone, PartialEq)]
pub struct CardDeck {
    pub cards: Vec<Card>
}

pub struct Bob {
    pub interested: bool
}

pub struct Alice {
    pub interested: bool
}


impl CardDeck {
    /*
    Implement Deck::join. This function will output a new deck that is comprised of
    three elements: two input decks and a separator king card.
    */
    pub fn join(a: CardDeck, b: CardDeck) -> CardDeck {
        let mut deck = a;
        deck.cards.extend(b.cards);
        deck.cards.extend(vec![Card::King]);
        deck
    }

    pub fn cyclic_shift(&mut self, shift: usize) {
        /*
        Implement Deck::cyclic_shift. This function will take a deck of size five and a required
        shift and will return a deck with the cards rotated to the left by shift.
         */
        assert_eq!(self.cards.len(), 5);

        self.cards.rotate_left(shift % 5);
    }

    pub fn decode(self) -> bool {
        /*
        Implement a method named decode as part of the deck struct. The method takes a deck of size
        five and outputs a Boolean value as follows: if there are 3 kings in a set of 5, output
        true, else output false. Note that the position of the kings does not matter. If we have
        kings in positions 1, 2, and 5, the function will output true.
         */
        let mut king_count = 0;

        if self.cards.len() != 5 {
            return false
        }

        for card in self.cards.iter() {
            match card {
                Card::King => {
                    king_count += 1
                },
                _ => {}
            }
        }
        return if king_count == 3 { true } else { false }
    }
}


impl Alice {
    pub fn encode(self) -> CardDeck {
        match self.interested {
            true => CardDeck {
                cards: vec![Card::Queen, Card::King]
            },
            false => CardDeck {
                cards: vec![Card::King, Card::Queen]
            }
        }
    }
}


impl Bob {
    pub fn encode(self) -> CardDeck {
        match self.interested {
            true => CardDeck {
                cards: vec![Card::King, Card::Queen]
            },
            false => CardDeck {
                cards: vec![Card::Queen, Card::King]
            }
        }
    }
}