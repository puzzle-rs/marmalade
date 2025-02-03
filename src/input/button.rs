#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Button {
    Left,
    Middle,
    Right,
}

impl Button {
    /// Create a mouse button for the given code, return None if the code is unknown
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
