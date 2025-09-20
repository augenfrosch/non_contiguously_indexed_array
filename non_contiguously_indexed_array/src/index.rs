pub trait NciIndex: Ord + Copy {
    /// Return the next index after this one, or `None` if this is the maximum possible index.
    /// The implementation must guarantee that if `Some(value)` is returned `value > next`.
    /// If this is not the case, any `NciArray` using the implementation will function incorrectly.
    fn next(self) -> Option<Self>;

    /// Return the distance between `self` and `other`.
    /// If the distance is greater than `usize::MAX`, return `None`.
    /// If `other > self`, the implementation must guarantee that `next` can be called at least the returned distance times without returning `None`,
    /// and at least `usize::MAX` if the returned distance is `None`.
    /// If this is not the case, any `NciArray` using the implementation will function incorrectly.
    fn distance(self, other: Self) -> Option<usize>;
}

macro_rules! impl_index_trait_for_primitive_num {
    ($t:ty) => {
        impl NciIndex for $t {
            fn next(self) -> Option<Self> {
                self.checked_add(1)
            }
            fn distance(self, other: Self) -> Option<usize> {
                self.abs_diff(other).try_into().ok()
            }
        }
    };
}

impl_index_trait_for_primitive_num!(u8);
impl_index_trait_for_primitive_num!(u16);
impl_index_trait_for_primitive_num!(u32);
impl_index_trait_for_primitive_num!(u64);
impl_index_trait_for_primitive_num!(u128);

impl_index_trait_for_primitive_num!(i8);
impl_index_trait_for_primitive_num!(i16);
impl_index_trait_for_primitive_num!(i32);
impl_index_trait_for_primitive_num!(i64);
impl_index_trait_for_primitive_num!(i128);
