use rand::{rngs::SmallRng, SeedableRng};

use self::{
    card::{Card, Gem},
    chip::{Chip, ChipStack},
    game_board::{GameBoard, OpenCard},
    player_board::PlayerBoard,
};

pub mod card;
pub mod chip;
pub mod deck;
pub mod game_board;
pub mod noble;
pub mod player_board;

/// Plays the game and returns a position of the winner.
pub fn play_game<const N: usize>(mut players: [Box<dyn Player>; N], seed: u64) -> usize {
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut game_board = GameBoard::new(
        players
            .len()
            .try_into()
            .expect("Splendor is only for 2-4 players"),
        &mut rng,
    );
    let mut player_boards = [(); N].map(|_| PlayerBoard::new());
    const SCORE_THRESHOLD: u8 = 15;
    while player_boards
        .iter()
        .any(|player| SCORE_THRESHOLD <= player.score())
    {
        for (player_board, player) in player_boards.iter_mut().zip(players.iter_mut()) {
            player_turn(player.as_mut(), player_board, &mut game_board);
        }
    }
    player_boards
        .iter()
        .enumerate()
        .max_by_key(|(_, board)| board.score())
        .map(|(idx, _)| idx)
        .unwrap()
}

fn player_turn(
    player: &mut dyn Player,
    player_board: &mut PlayerBoard,
    game_board: &mut GameBoard,
) {
    match player.ask_choice(player_board, game_board) {
        Choice::ThreeDifferentChips(gems) => {
            game_board.pick_differ_three_chips(gems);
            player_board.bring_differ_three_chips(gems);
        }
        Choice::TwoSameChips(gem) => {
            game_board.pick_same_two_chips(gem);
            player_board.bring_same_two_chips(gem);
        }
        Choice::BuyOpen(target) => {
            for use_golden_chips in [false, true] {
                if !player_board.can_buy(target.as_ref(), use_golden_chips) {
                    continue;
                }
                let picked = game_board.pick_open_card(target);
                let paid = player_board.buy_open(picked, use_golden_chips);
                game_board.return_chips(paid);
            }
        }
        Choice::BuyHand(target) => {
            for use_golden_chips in [false, true] {
                if !player_board.can_buy(&target, use_golden_chips) {
                    continue;
                }
                let paid = player_board.buy_kept(target, use_golden_chips);
                game_board.return_chips(paid);
            }
        }
        Choice::KeepOpen(card) => {
            let picked = game_board.pick_open_card(card);
            player_board.keep(picked);
            if 0 < game_board.bank().chips(Chip::Golden) {
                game_board.pick_golden_chip();
                player_board.bring_golden_chip();
            }
        }
        Choice::KeepTop { keeping_deck_level } => {
            let picked = game_board.pick_deck_top(keeping_deck_level);
            player_board.keep(picked);
            if 0 < game_board.bank().chips(Chip::Golden) {
                game_board.pick_golden_chip();
                player_board.bring_golden_chip();
            }
        }
    }
    while player_board.chip_stack().all_chips_count() <= 10 {
        let to_drop = player.force_drop(player_board);
        player_board.drop_chips(to_drop);
    }
}

pub trait Player {
    fn ask_choice(&mut self, your: &PlayerBoard, game: &GameBoard) -> Choice;

    /// Proposes chips to drop until the number of chips is lesser than 11.
    fn force_drop(&self, your: &PlayerBoard) -> ChipStack;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Choice<'a> {
    ThreeDifferentChips([Gem; 3]),
    TwoSameChips(Gem),
    BuyOpen(OpenCard<'a>),
    BuyHand(Card),
    KeepOpen(OpenCard<'a>),
    KeepTop { keeping_deck_level: u8 },
}
