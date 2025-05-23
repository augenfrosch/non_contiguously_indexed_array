#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound = "T: serde::Serialize + serde::de::DeserializeOwned")
)]
pub struct NciBaseArray<T, const R: usize, const N: usize> {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    index_ranges: [(usize, usize); R],
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    data: [T; N],
}

#[derive(Debug, PartialEq)]
pub struct NciArray<'a, T> {
    index_ranges: &'a [(usize, usize)],
    data: &'a [T],
}

impl<'a, T> NciArray<'a, T> {
    pub fn new(index_ranges: &'a [(usize, usize)], data: &'a [T]) -> Self {
        Self { index_ranges, data }
    }
}

impl<'a, T> std::ops::Index<usize> for NciArray<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

pub struct NciIndexIter<'a> {
    index_ranges: &'a [(usize, usize)],
    next_index_range: usize,
    true_index: usize,
    data_len: usize,
    index: usize,
}

impl<'a> NciIndexIter<'a> {
    fn new(index_ranges: &'a [(usize, usize)], data_len: usize) -> Self {
        let (initial_index, initial_next_index_range) = index_ranges
            .get(0)
            .map(|(range_start, skipped)| {
                if range_start - skipped == 0 {
                    (*range_start, 1)
                } else {
                    (0, 0)
                }
            })
            .unwrap_or_default();
        Self {
            index_ranges,
            next_index_range: initial_next_index_range,
            true_index: 0,
            data_len,
            index: initial_index,
        }
    }
}

impl<'a> Iterator for NciIndexIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.true_index < self.data_len {
            let value = self.index;

            self.index += 1;
            if let Some((range_start, skipped)) = self.index_ranges.get(self.next_index_range) {
                if range_start - skipped <= self.index {
                    self.index = *range_start;
                    self.next_index_range += 1;
                }
            }

            self.true_index += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<'a, T> NciArray<'a, T> {
    pub fn data(&self) -> core::slice::Iter<'a, T> {
        self.data.iter()
    }

    pub fn indices(&self) -> NciIndexIter {
        NciIndexIter::new(self.index_ranges, self.data.len())
    }

    pub fn entries(&self) -> std::iter::Zip<NciIndexIter, core::slice::Iter<'a, T>> {
        self.indices().zip(self.data())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut index_offset = 0;
        let mut index_range_start = 0;
        let mut index_range_end = None;
        for (range_start, skipped) in self.index_ranges {
            if range_start > &index {
                index_range_end = Some(range_start - skipped);
                break;
            } else {
                index_range_start = *range_start;
                index_offset += skipped;
            }
        }
        let slice_start = index_range_start - index_offset;
        let slice_end = index_range_end
            .map(|index_range_end| index_range_end - index_offset)
            .unwrap_or(self.data.len());
        let slice: &[T] = &self.data[slice_start..slice_end];
        slice.get(index - index_range_start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_ARRAY_1: NciBaseArray<i32, 2, 6> = NciBaseArray {
        index_ranges: [(10, 7), (100, 88)],
        data: [0, 1, 2, 10, 11, 100],
    };
    const BASE_ARRAY_2: NciBaseArray<i32, 3, 6> = NciBaseArray {
        index_ranges: [(100, 100), (200, 98), (500, 299)],
        data: [100, 101, 200, 500, 501, 502],
    };

    #[test]
    fn basic_array_test_1() {
        let arr = NciArray::new(&BASE_ARRAY_1.index_ranges, &BASE_ARRAY_1.data);
        let data_as_slice = arr.data().as_slice();
        assert_eq!(data_as_slice, BASE_ARRAY_1.data);

        assert_eq!(arr.get(0), Some(&0));
        assert_eq!(arr.get(1), Some(&1));
        assert_eq!(*arr.get(1).unwrap(), arr[1]);
        assert_eq!(arr.get(2), Some(&2));
        assert_eq!(arr.get(5), None);
        assert_eq!(arr.get(10), Some(&10));
        assert_eq!(arr.get(11), Some(&11));
        assert_eq!(*arr.get(11).unwrap(), arr[11]);
        assert_eq!(arr.get(55), None);
        assert_eq!(arr.get(99), None);
        assert_eq!(arr.get(100), Some(&100));
        assert_eq!(*arr.get(100).unwrap(), arr[100]);
        assert_eq!(arr.get(101), None);
    }
    #[test]
    fn basic_array_test_2() {
        let arr = NciArray::new(&BASE_ARRAY_2.index_ranges, &BASE_ARRAY_2.data);
        let data_as_slice = arr.data().as_slice();
        assert_eq!(data_as_slice, BASE_ARRAY_2.data);

        assert_eq!(arr.get(0), None);
        assert_eq!(arr.get(1), None);
        assert_eq!(arr.get(50), None);
        assert_eq!(arr.get(99), None);
        assert_eq!(arr.get(100), Some(&100));
        assert_eq!(arr.get(101), Some(&101));
        assert_eq!(*arr.get(101).unwrap(), arr[101]);
        assert_eq!(arr.get(102), None);
        assert_eq!(arr.get(150), None);
        assert_eq!(arr.get(199), None);
        assert_eq!(arr.get(200), Some(&200));
        assert_eq!(*arr.get(200).unwrap(), arr[200]);
        assert_eq!(arr.get(201), None);
        assert_eq!(arr.get(350), None);
        assert_eq!(arr.get(499), None);
        assert_eq!(arr.get(500), Some(&500));
        assert_eq!(arr.get(501), Some(&501));
        assert_eq!(arr.get(502), Some(&502));
        assert_eq!(*arr.get(502).unwrap(), arr[502]);
        assert_eq!(arr.get(503), None);
        assert_eq!(arr.get(750), None);
        assert_eq!(arr.get(999), None);
    }

    #[test]
    fn basic_array_iterator_test_1() {
        let arr = NciArray::new(&BASE_ARRAY_1.index_ranges, &BASE_ARRAY_1.data);

        let mut entries = arr.entries();
        let mut indices = arr.indices();
        let mut data = arr.data();

        while let (Some(entry), Some(index), Some(value)) =
            (entries.next(), indices.next(), data.next())
        {
            assert_eq!(entry.0, index);
            assert_eq!(entry.1, value);
            assert_eq!(entry.0, *entry.1 as usize); // not generally true
        }

        assert_eq!(entries.next(), None);
        assert_eq!(indices.next(), None);
        assert_eq!(data.next(), None);
    }

    #[test]
    fn basic_array_iterator_test_2() {
        let arr = NciArray::new(&BASE_ARRAY_2.index_ranges, &BASE_ARRAY_2.data);

        let mut entries = arr.entries();
        let mut indices = arr.indices();
        let mut data = arr.data();

        while let (Some(entry), Some(index), Some(value)) =
            (entries.next(), indices.next(), data.next())
        {
            assert_eq!(entry.0, index);
            assert_eq!(entry.1, value);
            assert_eq!(entry.0, *entry.1 as usize); // not generally true
        }

        assert_eq!(entries.next(), None);
        assert_eq!(indices.next(), None);
        assert_eq!(data.next(), None);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_test_1() {
        let serialized = ron::to_string(&BASE_ARRAY_1).unwrap();
        let deserialized: Result<NciBaseArray<i32, 2, 6>, ron::de::SpannedError> =
            ron::from_str(&serialized);
        assert_eq!(BASE_ARRAY_1, deserialized.unwrap())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_test_2() {
        let serialized = ron::to_string(&BASE_ARRAY_2).unwrap();
        let deserialized: Result<NciBaseArray<i32, 3, 6>, ron::de::SpannedError> =
            ron::from_str(&serialized);
        assert_eq!(BASE_ARRAY_2, deserialized.unwrap())
    }
}
