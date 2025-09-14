pub trait NciIndex: Ord + PartialOrd + Sized + Clone + Copy {
    /// Return the next index after this one, or `None` if this is the maximum possible index.
    fn next(self) -> Option<Self>;

    /// Return the distance between `self` and `other` in case `self <= other`.
    /// If `self > other` or the distance is greater than `usize::MAX`, return `None`.
    fn distance(self, other: Self) -> Option<usize>;
}

impl NciIndex for u32 {
    fn next(self) -> Option<Self> {
        self.checked_add(1)
    }

    fn distance(self, other: Self) -> Option<usize> {
        other.checked_sub(self)?.try_into().ok()
    }
}
