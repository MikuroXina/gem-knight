use super::chip::ChipStack;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gem {
    Blue,
    Green,
    Red,
    Black,
    White,
}

impl Gem {
    pub const fn all_gems() -> [Gem; 5] {
        [Gem::Blue, Gem::Green, Gem::Red, Gem::Black, Gem::White]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub kind: Gem,
    pub points: u8,
    pub cost: CardCost,
}

impl Card {
    pub fn new(kind: Gem, points: u8, cost: impl Into<CardCost>) -> Self {
        Self {
            kind,
            points,
            cost: cost.into(),
        }
    }

    pub fn points_per_cost(&self) -> f64 {
        self.points as f64 / self.cost.total_required_gems() as f64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardCost {
    blues: u8,
    greens: u8,
    reds: u8,
    blacks: u8,
    whites: u8,
}

impl From<[u8; 5]> for CardCost {
    fn from([blues, greens, reds, blacks, whites]: [u8; 5]) -> Self {
        Self {
            blues,
            greens,
            reds,
            blacks,
            whites,
        }
    }
}

impl CardCost {
    pub fn new(blues: u8, greens: u8, reds: u8, blacks: u8, whites: u8) -> Self {
        Self {
            blues,
            greens,
            reds,
            blacks,
            whites,
        }
    }

    pub fn total_required_gems(&self) -> u8 {
        self.blues + self.greens + self.reds + self.blacks + self.whites
    }

    pub fn cost_by(&self, gem: Gem) -> u8 {
        match gem {
            Gem::Blue => self.blues,
            Gem::Green => self.greens,
            Gem::Red => self.reds,
            Gem::Black => self.blacks,
            Gem::White => self.whites,
        }
    }

    pub fn color_kinds(&self) -> u8 {
        Gem::all_gems()
            .into_iter()
            .filter(|&gem| 0 < self.cost_by(gem))
            .count()
            .try_into()
            .unwrap()
    }

    pub fn eliminated(&self, chip_stack: ChipStack) -> u8 {
        Gem::all_gems()
            .into_iter()
            .map(|gem| {
                self.cost_by(gem)
                    .saturating_sub(chip_stack.chips(gem.into()))
            })
            .sum()
    }
}
