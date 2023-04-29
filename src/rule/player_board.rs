use std::collections::HashMap;

use super::{
    card::{Card, Gem},
    chip::{Chip, ChipStack},
    noble::Noble,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerBoard {
    bought_cards: Vec<Card>,
    kept_cards: Vec<Card>,
    chip_stack: ChipStack,
    visited_nobles: u8,
}

impl PlayerBoard {
    #[inline]
    pub fn new() -> Self {
        Self {
            bought_cards: Vec::with_capacity(15),
            kept_cards: Vec::with_capacity(3),
            chip_stack: ChipStack::new(),
            visited_nobles: 0,
        }
    }

    #[inline]
    pub fn bought_cards(&self) -> &[Card] {
        &self.bought_cards
    }

    #[inline]
    pub fn kept_cards(&self) -> &[Card] {
        &self.kept_cards
    }

    #[inline]
    pub fn chip_stack(&self) -> ChipStack {
        self.chip_stack
    }

    pub fn score(&self) -> u8 {
        self.bought_cards.iter().map(|card| card.points).sum::<u8>() + self.visited_nobles * 3
    }

    pub fn bring_differ_three_chips(&mut self, gems: [Gem; 3]) {
        for gem in gems {
            self.chip_stack = self.chip_stack.add_chips_to(gem.into(), 1);
        }
    }

    pub fn bring_same_two_chips(&mut self, gem: Gem) {
        self.chip_stack = self.chip_stack.add_chips_to(gem.into(), 2);
    }

    pub fn bring_golden_chip(&mut self) {
        self.chip_stack = self.chip_stack.add_chips_to(Chip::Golden, 1);
    }

    pub fn can_buy(&self, card: &Card, use_golden_chips: bool) -> bool {
        let eliminated_costs = card.cost.eliminated(self.chip_stack);
        eliminated_costs == 0
            || (use_golden_chips && eliminated_costs <= self.chip_stack.chips(Chip::Golden))
    }

    /// Buys the card and returns paid chips.
    pub fn buy(&mut self, card: Card, use_golden_chips: bool) -> ChipStack {
        debug_assert!(self.can_buy(&card, use_golden_chips));

        let eliminated_costs = card.cost.eliminated(self.chip_stack);
        let mut paid = ChipStack::new();
        for gem in Gem::all_gems() {
            let chip = gem.into();
            self.chip_stack = self.chip_stack.sub_chips_to(chip, card.cost.cost_by(gem));
            paid = paid.add_chips_to(chip, card.cost.cost_by(gem));
        }
        if use_golden_chips {
            self.chip_stack.sub_chips_to(Chip::Golden, eliminated_costs);
            paid = paid.add_chips_to(Chip::Golden, eliminated_costs);
        }
        self.bought_cards.push(card);
        paid
    }

    pub fn keep(&mut self, card: Card) {
        self.kept_cards.push(card);
    }

    pub fn can_welcome_noble(&self, noble: &Noble) -> bool {
        let mut bought_by_gem: HashMap<Gem, u8> = HashMap::new();
        for bought in self.bought_cards() {
            bought_by_gem
                .entry(bought.kind)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        noble
            .conditions
            .iter()
            .all(|(gem, required)| required <= bought_by_gem.get(gem).unwrap_or(&0))
    }

    pub fn welcome_noble(&mut self, noble: Noble) {
        debug_assert!(self.can_welcome_noble(&noble));
        self.visited_nobles += 1;
    }
}
