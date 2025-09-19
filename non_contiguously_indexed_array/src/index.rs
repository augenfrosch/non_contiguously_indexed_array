pub const trait NciIndex: const Ord + Copy {
    /// Return the next index after this one, or `None` if this is the maximum possible index.
    fn next(self) -> Option<Self>;

    /// Return the distance between `self` and `other`.
    /// If the distance is greater than `usize::MAX`, return `None`.
    fn distance(self, other: Self) -> Option<usize>;
}

macro_rules! impl_index_trait_for_primitive_num {
    ($t:ty) => {
        impl const NciIndex for $t {
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
