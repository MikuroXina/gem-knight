use super::card::Gem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Chip {
    Blue,
    Green,
    Red,
    Black,
    White,
    Golden,
}

impl From<Gem> for Chip {
    fn from(value: Gem) -> Self {
        match value {
            Gem::Blue => Chip::Blue,
            Gem::Green => Chip::Green,
            Gem::Red => Chip::Red,
            Gem::Black => Chip::Black,
            Gem::White => Chip::White,
        }
    }
}

impl Chip {
    pub const fn all_chips() -> [Chip; 6] {
        [
            Chip::Blue,
            Chip::Green,
            Chip::Red,
            Chip::Black,
            Chip::White,
            Chip::Golden,
        ]
    }
}

/// The state representation of stack of chips.
///
/// - 1-4 bits: blue chips
/// - 5-8 bits: green chips
/// - 9-12 bits: red chips
/// - 13-16 bits: black chips
/// - 17-20 bits: white chips
/// - 21-24 bits: golden chips
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChipStack(u32);

impl ChipStack {
    pub const fn new() -> Self {
        Self(0)
    }

    const fn shifts(variant: Chip) -> i32 {
        match variant {
            Chip::Blue => 0,
            Chip::Green => 4,
            Chip::Red => 8,
            Chip::Black => 12,
            Chip::White => 16,
            Chip::Golden => 20,
        }
    }

    pub const fn chips(self, variant: Chip) -> u8 {
        ((self.0 >> Self::shifts(variant)) & 0xf) as u8
    }

    pub const fn with_chips_to(self, variant: Chip, to_set: u8) -> Self {
        let write_bits = ((to_set & 0xf) as u32) << Self::shifts(variant);
        Self(self.0 & write_bits)
    }

    pub const fn add_chips_to(self, variant: Chip, diff: u8) -> Self {
        let saturated_add = self.chips(variant).saturating_add(diff);
        self.with_chips_to(variant, saturated_add)
    }

    pub const fn sub_chips_to(self, variant: Chip, diff: u8) -> Self {
        let saturated_sub = self.chips(variant).saturating_sub(diff);
        self.with_chips_to(variant, saturated_sub)
    }
}
