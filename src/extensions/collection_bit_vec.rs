pub use collection_bit_vec::*;
mod collection_bit_vec {
    use core::iter::FromIterator;
    use core::ops::RangeBounds;

    use crate::Ceil;

    pub type BitVecChunk = usize;

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct BitVec {
        chunks: Vec<BitVecChunk>,
        free_bits: usize,
    }

    impl BitVec {
        #[must_use]
        pub fn new() -> Self {
            Self::default()
        }

        #[must_use]
        pub const fn chunk_bits() -> usize {
            std::mem::size_of::<BitVecChunk>() * 8
        }

        #[must_use]
        pub fn offset_of(j: usize) -> (usize, usize) {
            (j / Self::chunk_bits(), j & (Self::chunk_bits() - 1))
        }

        pub fn range_of<R: RangeBounds<usize>>(
            &self,
            range: R,
        ) -> Option<((usize, usize), (usize, usize))> {
            use core::ops::Bound;
            let len = self.len();
            let start = match range.start_bound() {
                Bound::Included(&offset) => offset,
                Bound::Excluded(&offset) => offset + 1,
                Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                Bound::Included(&offset) => offset + 1,
                Bound::Excluded(&offset) => offset,
                Bound::Unbounded => len,
            };

            assert!(start <= len);
            assert!(end <= len);
            if start < end {
                Some((Self::offset_of(start), Self::offset_of(end)))
            } else {
                None
            }
        }

        #[must_use]
        pub fn chunks(&self) -> &[BitVecChunk] {
            &self.chunks
        }

        #[must_use]
        pub fn free_bits(&self) -> usize {
            self.free_bits
        }

        #[must_use]
        pub fn next_bit_mask(&self) -> usize {
            Self::chunk_bits() - self.free_bits
        }

        #[must_use]
        pub fn len(&self) -> usize {
            self.chunks.len() * Self::chunk_bits() - self.free_bits
        }

        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.chunks.is_empty()
        }

        pub fn truncate(&mut self, new_len: usize) {
            if new_len == 0 {
                self.chunks.truncate(0);
                self.free_bits = 0;
            } else if new_len < self.len() {
                let new_num_chunks = Ceil(new_len) / Self::chunk_bits();
                self.chunks.truncate(new_num_chunks);
                self.free_bits = new_num_chunks * Self::chunk_bits() - new_len;
            }
        }

        pub fn resize(&mut self, new_len: usize, value: bool) {
            let old_len = self.len();
            if new_len == 0 {
                self.chunks.truncate(0);
                self.free_bits = 0;
            } else {
                let old_num_chunks = self.chunks.len();
                let new_num_chunks = Ceil(new_len) / Self::chunk_bits();
                let template = if value {
                    BitVecChunk::MAX
                } else {
                    BitVecChunk::MIN
                };
                self.chunks.resize(new_num_chunks, template);
                self.free_bits = new_num_chunks * Self::chunk_bits() - new_len;
                if old_num_chunks < new_num_chunks {
                    let range = old_len..old_num_chunks * Self::chunk_bits();
                    self.set_range(range, value);
                } else if old_len < new_len {
                    self.set_range(old_len..new_len, value);
                }
            }
        }

        pub fn push(&mut self, value: bool) {
            if self.free_bits == 0 {
                self.chunks.push(0);
                self.free_bits = Self::chunk_bits();
            }
            let next_bit_mask = self.next_bit_mask();
            let last_chunk = self.chunks.last_mut().unwrap();
            if value {
                *last_chunk |= 1 << next_bit_mask;
            } else {
                *last_chunk &= !(1 << next_bit_mask);
            }
            self.free_bits -= 1;
        }

        pub fn pop(&mut self) -> Option<bool> {
            if self.is_empty() {
                return None;
            }
            self.free_bits += 1;
            let last_chunk = self.chunks.last_mut().unwrap();
            let result = (*last_chunk >> self.next_bit_mask()) & 1 == 1;
            if self.free_bits == Self::chunk_bits() {
                let _ = self.chunks.pop();
                self.free_bits = 0;
            }
            Some(result)
        }

        #[must_use]
        pub fn try_get(&self, offset: usize) -> Option<bool> {
            if offset < self.len() {
                let (j, b) = Self::offset_of(offset);
                Some((self.chunks[j] & (1 << b)) != 0)
            } else {
                None
            }
        }

        pub fn try_set(&mut self, offset: usize, value: bool) -> Result<(), ()> {
            if offset < self.len() {
                assert!(offset < self.len());
                let (j, b) = Self::offset_of(offset);
                if value {
                    self.chunks[j] |= 1 << b;
                } else {
                    self.chunks[j] &= !(1 << b);
                }
                Ok(())
            } else {
                Err(())
            }
        }

        #[must_use]
        pub fn get(&self, offset: usize) -> bool {
            self.try_get(offset).unwrap()
        }

        pub fn set(&mut self, offset: usize, value: bool) {
            self.try_set(offset, value).unwrap();
        }

        #[allow(clippy::similar_names)]
        pub fn set_range<R: RangeBounds<usize>>(&mut self, range: R, value: bool) {
            let ((lj, lb), (rj, rb)) = if let Some(range) = self.range_of(range) {
                range
            } else {
                return;
            };

            if lj == rj {
                let mask = std::usize::MAX >> (Self::chunk_bits() + lb - rb) << lb;
                if value {
                    self.chunks[lj] |= mask;
                } else {
                    self.chunks[lj] &= !mask;
                }
            } else {
                let lmask = std::usize::MAX << lb;
                let rmask = if rb == 0 {
                    None
                } else {
                    Some(std::usize::MAX >> (Self::chunk_bits() - rb))
                };
                if value {
                    self.chunks[lj] |= lmask;
                    for j in lj + 1..rj {
                        self.chunks[j] = std::usize::MAX;
                    }
                    if let Some(rmask) = rmask {
                        self.chunks[rj] |= rmask;
                    }
                } else {
                    self.chunks[lj] &= !lmask;
                    for j in lj + 1..rj {
                        self.chunks[j] = 0;
                    }
                    if let Some(rmask) = rmask {
                        self.chunks[rj] &= !rmask;
                    }
                }
            }
        }

        #[allow(clippy::similar_names)]
        pub fn count_ones<R: RangeBounds<usize>>(&mut self, range: R) -> u32 {
            let ((lj, lb), (rj, rb)) = if let Some(range) = self.range_of(range) {
                range
            } else {
                return 0;
            };

            if lj == rj {
                let mask = std::usize::MAX >> (Self::chunk_bits() + lb - rb) << lb;
                (self.chunks[lj] & mask).count_ones()
            } else {
                let lmask = std::usize::MAX << lb;
                let rmask = if rb == 0 {
                    None
                } else {
                    Some(std::usize::MAX >> (Self::chunk_bits() - rb))
                };
                (self.chunks[lj] & lmask).count_ones()
                    + self.chunks[lj + 1..rj]
                        .iter()
                        .map(|chunk| chunk.count_ones())
                        .sum::<u32>()
                    + rmask.map_or(0, |rmask| (self.chunks[rj] & rmask).count_ones())
            }
        }

        #[must_use]
        pub fn iter(&self) -> BitVecIter<'_> {
            BitVecIter::new(self)
        }
    }

    #[derive(Clone, Debug)]
    pub struct BitVecIter<'a>(&'a BitVec, usize);

    impl<'a> BitVecIter<'a> {
        #[must_use]
        pub fn new(bitvec: &'a BitVec) -> Self {
            Self(bitvec, 0)
        }
    }

    impl Iterator for BitVecIter<'_> {
        type Item = bool;

        fn next(&mut self) -> Option<Self::Item> {
            if self.1 < self.0.len() {
                let value = self.0.get(self.1);
                self.1 += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct BitVecIntoIter(BitVec, usize);

    impl BitVecIntoIter {
        #[must_use]
        pub fn new(bitvec: BitVec) -> Self {
            Self(bitvec, 0)
        }
    }

    impl IntoIterator for BitVec {
        type Item = bool;
        type IntoIter = BitVecIntoIter;

        fn into_iter(self) -> Self::IntoIter {
            BitVecIntoIter::new(self)
        }
    }

    impl Iterator for BitVecIntoIter {
        type Item = bool;

        fn next(&mut self) -> Option<Self::Item> {
            if self.1 < self.0.len() {
                let value = self.0.get(self.1);
                self.1 += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    impl FromIterator<bool> for BitVec {
        fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
            let mut vec = Self::new();
            for value in iter {
                vec.push(value);
            }
            vec
        }
    }

    impl Extend<bool> for BitVec {
        fn extend<I: IntoIterator<Item = bool>>(&mut self, iter: I) {
            for value in iter {
                self.push(value);
            }
        }
    }
}
