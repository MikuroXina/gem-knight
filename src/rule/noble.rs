use super::card::Gem;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Noble {
    pub conditions: Vec<(Gem, u8)>,
}

impl Noble {
    pub fn new(conditions: impl IntoIterator<Item = (Gem, u8)>) -> Self {
        Self {
            conditions: conditions.into_iter().collect(),
        }
    }
}
