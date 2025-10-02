#[derive(Debug, Clone, Copy, Eq)]
pub enum Integer {
    Negative(u128), // Both variants can currently encode ±0
    NonNegative(u128),
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Negative(l0), Self::Negative(r0)) => l0 == r0,
            (Self::NonNegative(l0), Self::Negative(r0))
            | (Self::Negative(l0), Self::NonNegative(r0)) => *l0 == 0 && *r0 == 0,
            (Self::NonNegative(l0), Self::NonNegative(r0)) => l0 == r0,
        }
    }
}

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match (self, other) {
            (Self::Negative(l0), Self::Negative(r0)) => r0.cmp(l0),
            (Self::Negative(l0), Self::NonNegative(r0)) => match (l0, r0) {
                (0, 0) => core::cmp::Ordering::Equal,
                (_, _) => core::cmp::Ordering::Less,
            },
            (Self::NonNegative(l0), Self::Negative(r0)) => match (l0, r0) {
                (0, 0) => core::cmp::Ordering::Equal,
                (_, _) => core::cmp::Ordering::Greater,
            },
            (Self::NonNegative(l0), Self::NonNegative(r0)) => l0.cmp(r0),
        }
    }
}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl non_contiguously_indexed_array_shared::NciIndex for Integer {
    fn next(self) -> Option<Self> {
        match self {
            Self::Negative(value) => Some(
                value
                    .checked_sub(1)
                    .map_or(Self::NonNegative(1), Self::Negative),
            ),
            Self::NonNegative(value) => value.checked_add(1).map(Self::NonNegative),
        }
    }

    fn distance(self, _other: Self) -> Option<usize> {
        unimplemented!(
            "`NciIndex::next` is used to detect new segments. Size of host architecture's `usize` might differ from target."
        )
    }
}
