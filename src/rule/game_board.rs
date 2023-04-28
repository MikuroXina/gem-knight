use std::marker::PhantomData;

use rand::Rng;

use super::{
    card::{Card, Gem},
    chip::{Chip, ChipStack},
    deck::standard_deck,
    noble::Noble,
};

#[derive(Debug, Clone)]
pub struct GameBoard {
    card_rows: [CardRow; 3],
    nobles: Vec<Noble>,
    bank: ChipStack,
}

impl GameBoard {
    pub fn new(players: u8, rng: &mut impl Rng) -> Self {
        let ([mut level1, mut level2, mut level3], mut nobles) = standard_deck();
        use rand::seq::SliceRandom;
        level1.shuffle(rng);
        level2.shuffle(rng);
        level3.shuffle(rng);
        nobles.shuffle(rng);

        let card_rows = [level1, level2, level3].map(CardRow::new);

        let picked_nobles: Vec<_> = nobles[..(players as usize) + 1].to_vec();

        let using_chips = match players {
            2 => 7,
            3 => 5,
            4 => 4,
            _ => panic!("Splendor is only for 2-4 players"),
        };
        let mut bank = ChipStack::new().with_chips_to(Chip::Golden, 5);
        for gem in Gem::all_gems() {
            bank = bank.with_chips_to(gem.into(), using_chips);
        }

        Self {
            card_rows,
            nobles: picked_nobles,
            bank,
        }
    }

    pub fn open_cards(&'_ self) -> Vec<OpenCard<'_>> {
        self.card_rows
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.open
                    .iter()
                    .enumerate()
                    .map(move |(open_idx, &card)| OpenCard {
                        source_row_index: row_idx,
                        open_index: open_idx,
                        card,
                        _phantom: PhantomData,
                    })
            })
            .collect()
    }

    pub fn pick_open_card(&'_ mut self, open: OpenCard<'_>) -> Card {
        let picked = self.card_rows[open.source_row_index]
            .open
            .remove(open.source_row_index);
        self.card_rows[open.source_row_index].update_open();
        picked
    }
}

#[derive(Debug, Clone)]
pub struct CardRow {
    open: Vec<Card>,
    stock: Vec<Card>,
}

impl CardRow {
    fn new(mut shuffled_deck: Vec<Card>) -> Self {
        let mut open = Vec::with_capacity(4);
        for _ in 0..4 {
            open.push(shuffled_deck.pop().expect("amount of deck was too low"));
        }
        Self {
            open,
            stock: shuffled_deck,
        }
    }

    fn update_open(&mut self) {
        while self.open.len() < 4 {
            let Some(top) = self.stock.pop() else { return; };
            self.open.push(top);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpenCard<'a> {
    source_row_index: usize,
    open_index: usize,
    card: Card,
    _phantom: PhantomData<&'a ()>,
}

impl AsRef<Card> for OpenCard<'_> {
    fn as_ref(&self) -> &Card {
        &self.card
    }
}
