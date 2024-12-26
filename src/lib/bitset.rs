use std::fmt::Debug;

/**
* Stores a set of u64 values efficiently
*/
#[derive(Clone, Debug)]
pub struct BitSet {
    vec: Vec<u64>,
    count: usize,
}

impl BitSet {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            count: 0,
        }
    }

    pub fn with_capacity(max_capacity: usize) -> Self {
        let n_words = (max_capacity + 63) / 64; // ceil(max_capacity / 64)
        Self {
            vec: vec![0; n_words],
            count: 0,
        }
    }

    fn extend_capacity(&mut self, new_n_words: usize) {
        if new_n_words > self.vec.len() {
            self.vec.resize(new_n_words, 0);
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /** Insert a value into the set */
    pub fn insert(&mut self, value: u64) {
        let (word, idx) = (value / 64, value % 64);
        self.extend_capacity((word + 1) as usize);

        let target = self
            .vec
            .get_mut(word as usize)
            .expect("Should have extended");
        if *target & (1 << idx) == 0 {
            self.count += 1;
            *target |= 1 << idx;
        }
    }

    pub fn remove(&mut self, value: u64) {
        let (word, idx) = (value / 64, value % 64);
        let word = word as usize;
        if word >= self.vec.len() {
            return;
        }
        let target = self.vec.get_mut(word).unwrap();
        let mask = 1u64 << idx;
        if *target & mask > 0 {
            *target &= !mask;
            self.count -= 1;
        }
    }

    pub fn contains(&self, value: u64) -> bool {
        let (word, idx) = (value / 64, value % 64);
        self.vec
            .get(word as usize)
            .is_some_and(|v| (*v & (1 << idx)) > 0)
    }

    pub fn capacity(&self) -> usize {
        self.vec.len() * 64
    }

    pub fn intersection_inplace(&mut self, other: &BitSet) {
        for (target, val) in self.vec.iter_mut().zip(other.vec.iter()) {
            let num_to_remove = (*target & !*val).count_ones();
            *target &= val;
            self.count -= num_to_remove as usize;
        }
        for target in self.vec.iter_mut().skip(other.vec.len()) {
            let num_to_remove = target.count_ones();
            *target = 0;
            self.count -= num_to_remove as usize;
        }
    }

    pub fn intersection(&self, other: &BitSet) -> BitSet {
        let mut c = self.clone();
        c.intersection_inplace(other);
        c
    }

    pub fn union_inplace(&mut self, other: &BitSet) {
        self.extend_capacity(other.vec.len());

        for (target, val) in self.vec.iter_mut().zip(other.vec.iter()) {
            let num_to_add = (!*target & *val).count_ones();
            *target |= val;
            self.count += num_to_add as usize;
        }
    }

    pub fn union(&self, other: &BitSet) -> BitSet {
        let mut c = self.clone();
        c.union_inplace(other);
        c
    }

    pub fn difference_inplace(&mut self, other: &BitSet) {
        for (target, val) in self.vec.iter_mut().zip(other.vec.iter()) {
            let num_to_remove = (*target & *val).count_ones();
            *target &= !*val;
            self.count -= num_to_remove as usize;
        }
    }

    pub fn difference(&self, other: &BitSet) -> BitSet {
        let mut cl = self.clone();
        cl.difference_inplace(other);
        cl
    }

    pub fn iter(&self) -> BitSetIter<'_> {
        BitSetIter {
            current_idx: 0,
            current_word: 0,
            data: &self.vec,
        }
    }
}

impl std::default::Default for BitSet {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for BitSet {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.vec.iter().zip(other.vec.iter()).all(|(a, b)| *a == *b)
    }
}

impl Eq for BitSet {}

pub struct BitSetIter<'a> {
    data: &'a [u64],
    current_word: usize,
    current_idx: u64,
}

impl Iterator for BitSetIter<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_word < self.data.len() {
            let chunk = self.data[self.current_word];

            if chunk == 0 {
                self.current_word += 1;
                self.current_idx = 0;
                continue;
            }

            while self.current_idx < 64 {
                if (chunk & (1 << self.current_idx)) > 0 {
                    let result = 64 * (self.current_word as u64) + self.current_idx;
                    self.current_idx += 1;
                    return Some(result);
                }
                self.current_idx += 1;
            }

            self.current_word += 1;
            self.current_idx = 0;
        }

        None
    }
}

impl FromIterator<u64> for BitSet {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut bitset = BitSet::new();
        for value in iter {
            bitset.insert(value);
        }
        bitset
    }
}

#[macro_export]
macro_rules! bit_set {
    ( $($x:expr), * ) => {
        {
            let mut temp_bitset = BitSet::new();
            $(
                temp_bitset.insert($x);
            )*
            temp_bitset
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::bitset::*;
    #[test]
    fn test_contains() {
        let v = bit_set!(1, 2);
        assert!(v.contains(1));
    }

    #[test]
    fn test_eq() {
        let a = BitSet {
            vec: vec![0b10101, 0],
            count: 3,
        };
        let b = BitSet {
            vec: vec![0b10101],
            count: 3,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_insert() {
        let mut v = BitSet::new();
        v.insert(1);
        v.insert(4);
        v.insert(5);
        assert_eq!(v.len(), 3);
        assert_eq!(v, bit_set!(1, 4, 5));
        v.insert(1);
        assert_eq!(v.len(), 3);
        assert_eq!(v, bit_set!(1, 4, 5));
    }

    #[test]
    fn test_remove() {
        let mut v = bit_set!(1, 4, 5, 202);
        v.remove(4);
        assert_eq!(v, bit_set!(1, 5, 202));
        let mut v2 = v.clone();
        v2.remove(6);
        assert_eq!(v, v2);

        let mut v3 = bit_set!(1, 4, 5, 202);
        v3.remove(202);
        assert_eq!(v3, bit_set!(1, 4, 5));
    }

    #[test]
    fn test_intersection_inplace() {
        let mut v = bit_set!(1, 4, 5, 2000);
        v.intersection_inplace(&bit_set!(2, 4, 6));
        assert_eq!(v.len(), 1);
        assert!(v.contains(4));
    }

    #[test]
    fn test_union_inplace() {
        let mut v = bit_set!(1, 4, 5);
        v.union_inplace(&bit_set!(2, 4, 6));
        assert_eq!(v.len(), 5);
        assert_eq!(v, bit_set!(1, 2, 4, 5, 6));
    }

    #[test]
    fn test_difference_inplace() {
        let mut v = bit_set!(1, 4, 5);
        v.difference_inplace(&bit_set!(2, 4, 6));
        assert_eq!(v.len(), 2);
        assert_eq!(v, bit_set!(1, 5));
    }

    #[test]
    fn test_iter() {
        let v = bit_set!(1, 4, 5, 100);
        let x: Vec<_> = v.iter().collect();
        assert_eq!(x, vec![1, 4, 5, 100]);

        let v: BitSet = (0..520).collect();
        let x: Vec<u64> = (0..520).collect();
        let act: Vec<u64> = v.iter().collect();
        assert_eq!(x, act);
    }
}
