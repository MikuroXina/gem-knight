use super::{
    card::{Card, Gem::*},
    noble::Noble,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Deck {
    pub level1: Vec<Card>,
    pub level2: Vec<Card>,
    pub level3: Vec<Card>,
    pub nobles: Vec<Noble>,
}

pub fn standard_deck() -> Deck {
    let level1 = vec![
        Card::new(Blue, 0, [0, 0, 0, 2, 1]),
        Card::new(Blue, 0, [2, 0, 2, 0, 1]),
        Card::new(Blue, 0, [0, 2, 0, 2, 0]),
        Card::new(Blue, 0, [0, 1, 1, 1, 1]),
        Card::new(Blue, 0, [0, 1, 2, 1, 1]),
        Card::new(Blue, 0, [1, 3, 1, 0, 0]),
        Card::new(Blue, 0, [0, 0, 0, 3, 0]),
        Card::new(Blue, 1, [0, 0, 4, 0, 0]),
        Card::new(Red, 0, [0, 1, 0, 2, 2]),
        Card::new(Red, 0, [2, 1, 0, 0, 0]),
        Card::new(Red, 0, [0, 0, 0, 0, 3]),
        Card::new(Red, 0, [0, 0, 1, 3, 1]),
        Card::new(Red, 0, [0, 0, 2, 0, 2]),
        Card::new(Red, 0, [1, 1, 0, 1, 1]),
        Card::new(Red, 1, [0, 0, 0, 0, 4]),
        Card::new(Black, 0, [2, 0, 1, 0, 2]),
        Card::new(Black, 0, [1, 1, 1, 0, 1]),
        Card::new(Black, 0, [2, 1, 1, 0, 1]),
        Card::new(Black, 0, [0, 3, 0, 0, 0]),
        Card::new(Black, 0, [0, 1, 3, 1, 0]),
        Card::new(Black, 0, [0, 2, 1, 0, 0]),
        Card::new(Black, 1, [4, 0, 0, 0, 0]),
        Card::new(White, 0, [1, 1, 1, 1, 0]),
        Card::new(White, 0, [3, 0, 0, 0, 0]),
        Card::new(White, 0, [2, 0, 0, 2, 0]),
        Card::new(White, 0, [0, 0, 2, 1, 0]),
        Card::new(White, 0, [1, 0, 0, 1, 3]),
        Card::new(White, 0, [1, 2, 1, 1, 0]),
        Card::new(White, 0, [2, 2, 0, 1, 0]),
        Card::new(White, 1, [0, 4, 0, 0, 0]),
        Card::new(Green, 0, [3, 1, 0, 0, 1]),
        Card::new(Green, 0, [0, 0, 3, 0, 0]),
        Card::new(Green, 0, [1, 0, 1, 2, 1]),
        Card::new(Green, 0, [1, 0, 1, 1, 1]),
        Card::new(Green, 0, [2, 0, 2, 0, 0]),
        Card::new(Green, 0, [1, 0, 0, 0, 2]),
        Card::new(Green, 1, [0, 0, 0, 4, 0]),
    ];
    let level2 = vec![
        Card::new(Blue, 1, [2, 3, 0, 3, 0]),
        Card::new(Blue, 1, [2, 2, 3, 0, 0]),
        Card::new(Blue, 2, [0, 0, 1, 4, 2]),
        Card::new(Blue, 2, [5, 0, 0, 0, 0]),
        Card::new(Blue, 2, [3, 0, 0, 0, 5]),
        Card::new(Blue, 3, [6, 0, 0, 0, 0]),
        Card::new(Red, 1, [3, 0, 2, 3, 0]),
        Card::new(Red, 1, [0, 0, 2, 3, 2]),
        Card::new(Red, 2, [4, 2, 0, 0, 1]),
        Card::new(Red, 2, [0, 0, 0, 5, 3]),
        Card::new(Red, 2, [0, 0, 0, 5, 0]),
        Card::new(Red, 3, [0, 0, 6, 0, 0]),
        Card::new(Black, 1, [2, 2, 0, 0, 3]),
        Card::new(Black, 1, [0, 3, 0, 2, 3]),
        Card::new(Black, 2, [1, 4, 2, 0, 0]),
        Card::new(Black, 2, [0, 5, 3, 0, 0]),
        Card::new(Black, 2, [0, 0, 0, 0, 5]),
        Card::new(Black, 3, [0, 0, 0, 6, 0]),
        Card::new(White, 1, [3, 0, 3, 0, 2]),
        Card::new(White, 1, [0, 3, 2, 2, 0]),
        Card::new(White, 2, [0, 1, 4, 2, 0]),
        Card::new(White, 2, [0, 0, 5, 0, 0]),
        Card::new(White, 2, [0, 0, 5, 3, 0]),
        Card::new(White, 3, [0, 0, 0, 0, 6]),
        Card::new(Green, 1, [3, 0, 0, 2, 2]),
        Card::new(Green, 1, [0, 2, 3, 0, 3]),
        Card::new(Green, 2, [0, 5, 0, 0, 0]),
        Card::new(Green, 2, [2, 0, 0, 1, 4]),
        Card::new(Green, 2, [5, 3, 0, 0, 0]),
        Card::new(Green, 3, [0, 6, 0, 0, 0]),
    ];
    let level3 = vec![
        Card::new(Blue, 3, [0, 3, 3, 5, 3]),
        Card::new(Blue, 4, [0, 0, 0, 0, 7]),
        Card::new(Blue, 4, [3, 0, 0, 3, 6]),
        Card::new(Blue, 5, [3, 0, 0, 0, 7]),
        Card::new(Red, 3, [5, 3, 0, 3, 3]),
        Card::new(Red, 4, [3, 6, 3, 0, 0]),
        Card::new(Red, 4, [0, 7, 0, 0, 0]),
        Card::new(Red, 5, [0, 7, 3, 0, 0]),
        Card::new(Black, 3, [3, 5, 3, 0, 3]),
        Card::new(Black, 4, [0, 0, 7, 0, 0]),
        Card::new(Black, 4, [0, 3, 6, 3, 0]),
        Card::new(Black, 5, [0, 0, 7, 3, 0]),
        Card::new(White, 3, [3, 3, 5, 3, 0]),
        Card::new(White, 4, [0, 0, 0, 7, 0]),
        Card::new(White, 4, [0, 0, 3, 6, 3]),
        Card::new(White, 5, [0, 0, 0, 7, 3]),
        Card::new(Green, 3, [3, 0, 3, 3, 5]),
        Card::new(Green, 4, [6, 3, 0, 0, 3]),
        Card::new(Green, 4, [7, 0, 0, 0, 0]),
        Card::new(Green, 5, [7, 3, 0, 0, 0]),
    ];
    let nobles = vec![
        Noble::new([(Green, 3), (Red, 3), (Black, 3)]),
        Noble::new([(Red, 3), (Black, 3), (White, 3)]),
        Noble::new([(Blue, 4), (White, 4)]),
        Noble::new([(Blue, 3), (Green, 3), (White, 3)]),
        Noble::new([(Red, 4), (Black, 4)]),
        Noble::new([(Blue, 3), (Green, 3), (Red, 3)]),
        Noble::new([(Green, 4), (Red, 4)]),
        Noble::new([(Blue, 3), (Black, 3), (White, 3)]),
        Noble::new([(Black, 4), (White, 4)]),
        Noble::new([(Blue, 4), (Green, 4)]),
    ];
    Deck {
        level1,
        level2,
        level3,
        nobles,
    }
}
