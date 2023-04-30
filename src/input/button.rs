#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Left,
    Middle,
    Right,
}

impl Button {
    #[must_use]
    pub const fn from_code(code: i16) -> Option<Self> {
        Some(match code {
            0 => Self::Left,
            1 => Self::Middle,
            2 => Self::Right,
            _ => return None,
        })
    }
}
