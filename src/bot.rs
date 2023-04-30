//! # Strategy
//!
//! Some cards are cheaper than others. So GemKnight will buy cards below prior to others:
//!
//! - 4 points, 7 costs: The 1st efficiency.
//! - 3 points, 6 costs: The 2nd efficiency.
//! - 5 points, 10 costs: The 3rd efficiency.
//! - 2 points, 5 costs: The 4th efficiency.
//!
//! GemKnight aims to purchase them greedy. If there is no such card on the field, then it will take white gems or white cards as possible. Because white gems will lead to purchase additional cards because most of all require white gems as the cost.
//!
//! As a result, GemKnight will act the first possible tactic of below:
//!
//! 1. Reserve the 1st-4th efficient card on the field when there is no card in the hand.
//! 2. Aim to purchase the card whose kind of gem is able to pay the cost of reserved card. Select the most efficient way of below:
//!     1. Reserve the card when it will be bought by another player in this round.
//!     2. Take required two chips.
//!     3. Take three different chips including required and white ones.
//! 3. Aim to purchase the cheapest white card on the field. Tactics choices are same as before.
//! 4. Reserve the top of level 3 deck.
//! 5. Reserve the top of level 2 deck.
//! 6. Reserve the top of level 1 deck.
//! 7. Panic.

use crate::rule::{
    card::{Card, Gem},
    chip::ChipStack,
    game_board::GameBoard,
    player_board::PlayerBoard,
    Choice, Player,
};

#[derive(Debug, Default, Clone)]
pub struct GemKnight {
    target_gem: Option<Gem>,
    target_card: Option<Card>,
}

impl Player for GemKnight {
    fn ask_choice(&mut self, your: &PlayerBoard, game: &GameBoard) -> Choice {
        todo!()
    }

    fn force_drop(&self, _your: &PlayerBoard) -> ChipStack {
        todo!()
    }
}
