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
