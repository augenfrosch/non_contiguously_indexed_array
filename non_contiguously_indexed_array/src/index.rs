pub trait NciIndex: Ord + Copy {
    /// Return the next index after this one, or `None` if there is no directly following index, for example, when `self` it the maximum possible index.
    /// The implementation should guarantee that if `Some(value)` is returned `value > self`.
    /// Additionally, it should also try to ensure that there exists no other value between `self` and the result if `Some(value)` is returned,
    /// this is to ensure the correctness and an efficient representation of any `NciArray` using the implementation, as it relies on the total order and binary search.
    /// If these ewquirements are not guaranteed by the implementation, it is possible that some `NciArray`s using the implementation cannot be constructed without violated invariants.
    fn next(self) -> Option<Self>;

    /// Return the distance between `self` and `other`.
    /// If the distance is greater than `usize::MAX`, return `None`.
    /// If `other > self` and the function returned `Some(distance)`, the implementation must guarantee that `next` can be called `distance` times to reach `other` without returning `None`.
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
