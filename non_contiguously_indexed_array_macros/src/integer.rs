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
                    .map_or(Self::NonNegative(1), |new_value| Self::Negative(new_value)),
            ),
            Self::NonNegative(value) => value
                .checked_add(1)
                .map(|new_value| Self::NonNegative(new_value)),
        }
    }

    fn distance(self, other: Self) -> Option<usize> {
        match (self, other) {
            (Integer::Negative(l0), Integer::Negative(r0))
            | (Integer::NonNegative(l0), Integer::NonNegative(r0)) => {
                l0.abs_diff(r0).try_into().ok()
            }
            (Integer::Negative(l0), Integer::NonNegative(r0))
            | (Integer::NonNegative(l0), Integer::Negative(r0)) => {
                l0.checked_add(r0)?.try_into().ok()
            }
        }
    }
}

macro_rules! impl_from_trait_for_unsigned_int {
    ($t:ty) => {
        impl From<$t> for Integer {
            fn from(value: $t) -> Self {
                Self::NonNegative(value as u128)
            }
        }
    };
}

macro_rules! impl_from_trait_for_signed_int {
    ($t:ty) => {
        impl From<$t> for Integer {
            fn from(value: $t) -> Self {
                if value >= 0 {
                    Self::NonNegative(value as u128)
                } else {
                    Self::Negative(-value as u128)
                }
            }
        }
    };
}

impl_from_trait_for_unsigned_int!(u8);
impl_from_trait_for_unsigned_int!(u16);
impl_from_trait_for_unsigned_int!(u32);
impl_from_trait_for_unsigned_int!(u64);
impl_from_trait_for_unsigned_int!(u128);
impl_from_trait_for_unsigned_int!(usize);

impl_from_trait_for_signed_int!(i8);
impl_from_trait_for_signed_int!(i16);
impl_from_trait_for_signed_int!(i32);
impl_from_trait_for_signed_int!(i64);
impl_from_trait_for_signed_int!(i128);
impl_from_trait_for_signed_int!(isize);
