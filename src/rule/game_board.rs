use std::marker::PhantomData;

use heapless::Vec;
use rand::Rng;

use super::{
    card::{Card, Gem},
    chip::{Chip, ChipStack},
    deck::{standard_deck, Deck},
    noble::Noble,
};

#[derive(Debug, Clone)]
pub struct GameBoard {
    card_rows: [CardRow; 3],
    nobles: Vec<Noble, 5>,
    bank: ChipStack,
}

impl GameBoard {
    pub fn new(players: u8, rng: &mut impl Rng) -> Self {
        let Deck {
            mut level1,
            mut level2,
            mut level3,
            mut nobles,
        } = standard_deck();
        use rand::seq::SliceRandom;
        level1.shuffle(rng);
        level2.shuffle(rng);
        level3.shuffle(rng);
        nobles.shuffle(rng);

        let card_rows = [level1, level2, level3].map(CardRow::new);

        let picked_nobles: Vec<_, 5> = nobles[..(players as usize) + 1].try_into().unwrap();

        let using_chips = match players {
            2 => 4,
            3 => 5,
            4 => 7,
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

    pub fn card_row(&self, index: usize) -> &CardRow {
        &self.card_rows[index]
    }

    pub fn open_cards(&'_ self) -> impl Iterator<Item = OpenCard<'_>> {
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
    }

    pub fn pick_open_card(&'_ mut self, open: OpenCard<'_>) -> Card {
        let picked = self.card_rows[open.source_row_index]
            .open
            .swap_remove(open.source_row_index);
        self.card_rows[open.source_row_index].update_open();
        picked
    }

    pub fn pick_deck_top(&mut self, keeping_deck_level: u8) -> Card {
        self.card_rows[keeping_deck_level as usize]
            .stock
            .pop()
            .expect("deck was empty")
    }

    pub fn bank(&self) -> &ChipStack {
        &self.bank
    }

    pub fn pick_differ_three_chips(&mut self, gems: [Gem; 3]) {
        debug_assert_ne!(gems[0], gems[1]);
        debug_assert_ne!(gems[1], gems[2]);
        for gem in gems {
            let chip = gem.into();
            debug_assert!(0 < self.bank.chips(chip));
            self.bank = self.bank.sub_chips_to(chip, 1);
        }
    }

    pub fn pick_same_two_chips(&mut self, gem: Gem) {
        let chip = gem.into();
        debug_assert!(4 <= self.bank.chips(chip));
        self.bank = self.bank.sub_chips_to(chip, 2);
    }

    pub fn pick_golden_chip(&mut self) {
        debug_assert!(0 < self.bank.chips(Chip::Golden));
        self.bank = self.bank.sub_chips_to(Chip::Golden, 1);
    }

    pub fn return_chips(&mut self, chips: ChipStack) {
        self.bank = self.bank.merge(chips);
    }
}

#[derive(Debug, Clone)]
pub struct CardRow {
    open: Vec<Card, 4>,
    stock: Vec<Card, 35>,
}

impl CardRow {
    fn new(mut shuffled_deck: std::vec::Vec<Card>) -> Self {
        let mut open = Vec::new();
        for _ in 0..4 {
            open.push(shuffled_deck.pop().expect("amount of deck was too low"))
                .unwrap();
        }
        Self {
            open,
            stock: shuffled_deck[..].try_into().unwrap(),
        }
    }

    fn update_open(&mut self) {
        while self.open.len() < 4 {
            let Some(top) = self.stock.pop() else { return; };
            self.open.push(top).unwrap();
        }
    }

    pub fn open_cards(&self) -> &[Card] {
        &self.open
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
