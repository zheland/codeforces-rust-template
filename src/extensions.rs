pub use option_min_max::*;
mod option_min_max {
    pub trait OptionMinMax<T> {
        fn omin(self, other: T) -> T;
        fn omax(self, other: T) -> T;
    }

    impl<T: Ord> OptionMinMax<T> for Option<T> {
        fn omin(self, other: T) -> T {
            match self {
                Some(value) => value.min(other),
                None => other,
            }
        }

        fn omax(self, other: T) -> T {
            match self {
                Some(value) => value.max(other),
                None => other,
            }
        }
    }

    impl<T: Ord> OptionMinMax<Option<T>> for Option<T> {
        fn omin(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(value), Some(other)) => Some(value.min(other)),
                (Some(value), None) => Some(value),
                (None, Some(other)) => Some(other),
                (None, None) => None,
            }
        }

        fn omax(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(value), Some(other)) => Some(value.max(other)),
                (Some(value), None) => Some(value),
                (None, Some(other)) => Some(other),
                (None, None) => None,
            }
        }
    }
}

// ========

pub use bit_vec::*;
mod bit_vec {
    use core::iter::FromIterator;
    use core::ops::RangeBounds;

    use crate::ceil::Ceil;

    pub type BitVecChunk = usize;

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct BitVec {
        chunks: Vec<BitVecChunk>,
        free_bits: usize,
    }

    impl BitVec {
        pub fn new() -> Self {
            Self::default()
        }

        pub const fn chunk_bits() -> usize {
            std::mem::size_of::<BitVecChunk>() * 8
        }

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

        pub fn chunks(&self) -> &[BitVecChunk] {
            &self.chunks
        }

        pub fn free_bits(&self) -> usize {
            self.free_bits
        }

        pub fn next_bit_mask(&self) -> usize {
            Self::chunk_bits() - self.free_bits
        }

        pub fn len(&self) -> usize {
            self.chunks.len() * Self::chunk_bits() - self.free_bits
        }

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

        pub fn get(&self, offset: usize) -> bool {
            self.try_get(offset).unwrap()
        }

        pub fn set(&mut self, offset: usize, value: bool) {
            self.try_set(offset, value).unwrap()
        }

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
                let rmask = if rb != 0 {
                    Some(std::usize::MAX >> (Self::chunk_bits() - rb))
                } else {
                    None
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
                let rmask = if rb != 0 {
                    Some(std::usize::MAX >> (Self::chunk_bits() - rb))
                } else {
                    None
                };
                (self.chunks[lj] & lmask).count_ones()
                    + self.chunks[lj + 1..rj]
                        .iter()
                        .map(|chunk| chunk.count_ones())
                        .sum::<u32>()
                    + rmask.map_or(0, |rmask| (self.chunks[rj] & rmask).count_ones())
            }
        }

        pub fn iter(&self) -> BitVecIter<'_> {
            BitVecIter::new(&self)
        }
    }

    #[derive(Clone, Debug)]
    pub struct BitVecIter<'a>(&'a BitVec, usize);

    impl<'a> BitVecIter<'a> {
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

// ========

pub use each::*;
mod each {
    #[allow(single_use_lifetimes)]
    pub trait Each {
        type Iter;
        fn each(self) -> Self::Iter;
    }

    impl<'a, I> Each for &'a [I]
    where
        I: 'a + Clone + IntoIterator,
    {
        type Iter = EachIter<&'a [I], I>;
        fn each(self) -> Self::Iter {
            EachIter::new(self)
        }
    }

    #[derive(Clone, Debug)]
    pub struct EachIter<T, I>(Option<(T, Vec<I::IntoIter>, Vec<I::Item>)>)
    where
        T: AsRef<[I]>,
        I: Clone + IntoIterator;

    impl<T, I> EachIter<T, I>
    where
        T: AsRef<[I]>,
        I: Clone + IntoIterator,
    {
        pub fn new(iter: T) -> Self {
            Self(Some((iter, Vec::new(), Vec::new())))
        }
    }

    impl<T, I> Iterator for EachIter<T, I>
    where
        T: AsRef<[I]>,
        I: Clone + IntoIterator,
        I::Item: Clone,
    {
        type Item = Vec<I::Item>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.take().and_then(|mut data| loop {
                if data.2.len() < data.1.len() {
                    let mut last = data.1.pop().unwrap();
                    if let Some(value) = last.next() {
                        data.1.push(last);
                        data.2.push(value);
                    } else {
                        let _ = data.2.pop()?;
                    }
                } else if data.1.len() < data.0.as_ref().len() {
                    data.1
                        .push(data.0.as_ref()[data.1.len()].clone().into_iter());
                } else {
                    let item = data.2.clone();
                    let _ = data.2.pop();
                    self.0 = Some(data);
                    return Some(item);
                }
            })
        }
    }
}

// ========

pub use each_combination::*;
mod each_combination {
    use core::cell::RefCell;
    use core::iter::FusedIterator;
    use std::rc::Rc;

    #[allow(single_use_lifetimes)]
    pub trait EachCombination {
        type Iter;
        fn each_combination(self) -> Self::Iter;
        fn each_k_combination(self, k: usize) -> Self::Iter;
        fn each_ks_combination(self, min: usize, max: usize) -> Self::Iter;
    }

    impl<'a, T> EachCombination for &'a [T] {
        type Iter = EachCombinationIter<'a, T>;

        fn each_combination(self) -> Self::Iter {
            EachCombinationIter::new(self, 0, self.len())
        }

        fn each_k_combination(self, k: usize) -> Self::Iter {
            EachCombinationIter::new(self, k, k)
        }

        fn each_ks_combination(self, min: usize, max: usize) -> Self::Iter {
            EachCombinationIter::new(self, min, max)
        }
    }

    #[derive(Clone, Debug)]
    pub struct EachCombinationComb<'a, T> {
        data: &'a [T],
        mask: Vec<bool>,
        j: usize,
    }

    impl<'a, T> EachCombinationComb<'a, T> {
        pub fn new(data: &'a [T]) -> Self {
            Self {
                data: data,
                mask: vec![],
                j: 0,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct EachCombinationIter<'a, T> {
        comb: Option<Rc<RefCell<EachCombinationComb<'a, T>>>>,
        min: usize,
        max: usize,
        num: usize,
    }

    impl<'a, T> EachCombinationIter<'a, T> {
        pub fn new(data: &'a [T], min: usize, max: usize) -> Self {
            Self {
                comb: Some(Rc::new(RefCell::new(EachCombinationComb::new(data)))),
                min,
                max,
                num: 0,
            }
        }
    }

    impl<'a, T> Iterator for EachCombinationIter<'a, T> {
        type Item = EachCombinationIterIter<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(comb_rc) = self.comb.take() {
                {
                    let mut comb = comb_rc.borrow_mut();
                    comb.j = 0;
                    if comb.data.is_empty() {
                        return if self.min == 0 {
                            let iter = EachCombinationIterIter::new(&comb_rc);
                            Some(iter)
                        } else {
                            None
                        };
                    }
                    loop {
                        if comb.mask.is_empty() {
                            comb.mask = vec![false; comb.data.len()];
                        } else {
                            'outer: loop {
                                for bit in comb.mask.iter_mut() {
                                    *bit = !*bit;
                                    if *bit {
                                        self.num += 1;
                                        break 'outer;
                                    } else {
                                        self.num -= 1;
                                    }
                                }
                                return None;
                            }
                        }
                        if self.num >= self.min && self.num <= self.max {
                            break;
                        }
                    }
                }

                let iter = EachCombinationIterIter::new(&comb_rc);
                self.comb = Some(comb_rc);
                Some(iter)
            } else {
                None
            }
        }
    }

    impl<T> FusedIterator for EachCombinationIter<'_, T> {}

    #[derive(Clone, Debug)]
    pub struct EachCombinationIterIter<'a, T>(Rc<RefCell<EachCombinationComb<'a, T>>>);

    impl<'a, T> EachCombinationIterIter<'a, T> {
        pub fn new(rc: &Rc<RefCell<EachCombinationComb<'a, T>>>) -> Self {
            Self(Rc::clone(rc))
        }
    }

    impl<'a, T: 'a> Iterator for EachCombinationIterIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            let mut comb = self.0.borrow_mut();
            while comb.j < comb.mask.len() {
                if comb.mask[comb.j] {
                    let item = &comb.data[comb.j];
                    comb.j += 1;
                    return Some(item);
                } else {
                    comb.j += 1;
                }
            }
            None
        }
    }

    impl<T> FusedIterator for EachCombinationIterIter<'_, T> {}
}

// ========

pub use each_permutation::*;
mod each_permutation {
    use core::cell::{Ref, RefCell};
    use core::iter::FusedIterator;
    use std::rc::Rc;

    #[allow(single_use_lifetimes)]
    pub trait EachPermutation {
        type Iter;
        fn each_permutation(self) -> Self::Iter;
        fn each_k_permutation(self, k: usize) -> Self::Iter;
    }

    impl<'a, T> EachPermutation for &'a [T] {
        type Iter = EachPermutationIter<'a, T>;

        fn each_permutation(self) -> Self::Iter {
            EachPermutationIter::new(self, self.len())
        }

        fn each_k_permutation(self, k: usize) -> Self::Iter {
            EachPermutationIter::new(self, k)
        }
    }

    #[derive(Clone, Debug)]
    pub struct EachPermutationIter<'a, T> {
        data: Option<&'a [T]>,
        mask: Vec<bool>,
        idxs: Vec<usize>,
        perm: Rc<RefCell<Vec<&'a T>>>,
        k: usize,
    }

    impl<'a, T> EachPermutationIter<'a, T> {
        pub fn new(data: &'a [T], k: usize) -> Self {
            Self {
                data: if k <= data.len() { Some(data) } else { None },
                mask: vec![false; data.len()],
                idxs: Vec::new(),
                perm: Rc::new(RefCell::new(Vec::new())),
                k,
            }
        }
    }

    impl<'a, T> Iterator for EachPermutationIter<'a, T> {
        type Item = EachPermutationIterItem<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(data) = self.data {
                if self.k == 0 {
                    self.data = None;
                    return Some(EachPermutationIterItem::new(&self.perm));
                }

                if self.idxs.len() == self.k {
                    while let Some(mut j) = self.idxs.pop() {
                        let _ = self.perm.borrow_mut().pop();
                        self.mask[j] = false;
                        j += 1;
                        while j < data.len() && self.mask[j] {
                            j += 1;
                        }
                        if j < data.len() {
                            self.idxs.push(j);
                            self.perm.borrow_mut().push(&data[j]);
                            self.mask[j] = true;
                            break;
                        }
                    }
                    if self.idxs.len() == 0 {
                        self.data = None;
                        return None;
                    }
                }
                let mut j = 0;
                while j < data.len() && self.idxs.len() < self.k {
                    if self.mask[j] {
                        j += 1;
                    } else {
                        self.idxs.push(j);
                        self.perm.borrow_mut().push(&data[j]);
                        self.mask[j] = true;
                    }
                }
                if self.idxs.len() == self.k {
                    Some(EachPermutationIterItem::new(&self.perm))
                } else {
                    self.data = None;
                    None
                }
            } else {
                None
            }
        }
    }

    impl<T> FusedIterator for EachPermutationIter<'_, T> {}

    #[derive(Clone, Debug)]
    pub struct EachPermutationIterItem<'a, T>(Rc<RefCell<Vec<&'a T>>>);

    impl<'a, T> EachPermutationIterItem<'a, T> {
        pub fn new(rc: &Rc<RefCell<Vec<&'a T>>>) -> Self {
            Self(Rc::clone(rc))
        }

        pub fn get(&self) -> Ref<'_, [&'a T]> {
            Ref::map(self.0.borrow(), |value| value.as_slice())
        }
    }
}

// ========

pub use lfmt::*;
mod lfmt {
    #[macro_export]
    macro_rules! lfmt {
        () => {{ "" }};
        ( @impl ($($pattern:literal),*) $(, ($($args:tt)*))? ) => {{
            use ::core::fmt::{Debug, Display, Formatter, Result};

            #[derive(Clone, Copy)]
            struct LazyFormat<F: Fn(&mut Formatter<'_>) -> Result>(F);

            impl<F: Fn(&mut Formatter<'_>) -> Result> Debug for LazyFormat<F> {
                #[inline]
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    f.write_str(concat!("lazy_format(", stringify!(
                        ::core::concat!($($pattern)*),
                        $(, $($args)*)?), ")"))?;
                    (self.0)(f)
                }
            }

            impl<F: Fn(&mut Formatter<'_>) -> Result> Display for LazyFormat<F> {
                #[inline]
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    (self.0)(f)
                }
            }

            LazyFormat(#[inline] move |fmt: &mut Formatter<'_>| -> Result {
                ::core::write!(fmt, ::core::concat!($($pattern),*) $(, $($args)*)?)
            })
        }};
        ( @impl $($pattern:literal: $args:expr),* ) => {{
            $crate::lfmt!( @impl ($($pattern),*), ($($args),*) )
        }};
        ( $pattern:literal $(, $($args:tt)*)? ) => {{
            $crate::lfmt!( @impl ($pattern) $(, ($($args)*))? )
        }};
        ( , $arg:expr $(, $args:expr)* ) => {{
            $crate::lfmt!( @impl "{}": $arg $(, " {}": $args)* )
        }};
    }
}

// ========

pub use dijkstra::*;
mod dijkstra {
    use core::fmt::{self, Debug, Formatter};
    use std::collections::BTreeSet;

    use crate::Max;

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct NonMax<T>(T);

    impl<T: Debug + Eq + Max> NonMax<T> {
        #[track_caller]
        pub fn new(value: T) -> Self {
            assert_ne!(value, T::max());
            Self(value)
        }

        pub fn none() -> Self {
            Self(T::max())
        }

        pub fn get_ref(&self) -> Option<&T> {
            if self.0 != T::max() {
                Some(&self.0)
            } else {
                None
            }
        }

        pub fn get_mut(&mut self) -> Option<&mut T> {
            if self.0 != T::max() {
                Some(&mut self.0)
            } else {
                None
            }
        }

        pub fn into_inner(self) -> Option<T> {
            if self.0 != T::max() {
                Some(self.0)
            } else {
                None
            }
        }

        pub fn set_none(&mut self) {
            self.0 = T::max();
        }

        #[track_caller]
        pub fn set(&mut self, value: T) {
            assert_ne!(value, T::max());
            self.0 = value;
        }

        pub fn take(&mut self) -> Option<T> {
            let value = core::mem::replace(&mut self.0, T::max());
            Self(value).into_inner()
        }

        #[track_caller]
        pub fn replace(&mut self, value: T) -> Option<T> {
            assert_ne!(value, T::max());
            let old = core::mem::replace(&mut self.0, value);
            Self(old).into_inner()
        }
    }

    impl<T: Debug + Eq + Max> Default for NonMax<T> {
        fn default() -> Self {
            Self::none()
        }
    }

    impl<T: Debug + Eq + Max> From<NonMax<T>> for Option<T> {
        fn from(value: NonMax<T>) -> Option<T> {
            value.into_inner()
        }
    }

    impl<T: Debug + Eq + Max> From<T> for NonMax<T> {
        fn from(value: T) -> NonMax<T> {
            Self::new(value)
        }
    }

    impl<T: Debug + Eq + Max> From<Option<T>> for NonMax<T> {
        fn from(value: Option<T>) -> NonMax<T> {
            match value {
                Some(value) => Self::new(value),
                None => Self::none(),
            }
        }
    }

    impl<T: Debug + Eq + Max> Debug for NonMax<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            Debug::fmt(&self.get_ref(), f)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Dijkstra<T> {
        pub nodes: Vec<usize>,
        pub dists: Vec<Option<T>>,
        pub prevs: Vec<NonMax<usize>>,
    }

    impl<T> Dijkstra<T> {
        pub fn new<I, F, E>(num_nodes: usize, initial: I, mut edges: F) -> Self
        where
            T: Clone + PartialOrd + Ord,
            I: IntoIterator<Item = (usize, T)>,
            F: FnMut(usize, T) -> Option<E>,
            E: IntoIterator<Item = (usize, T)>,
        {
            let mut queue = BTreeSet::new();
            let mut nodes = Vec::new();
            let mut dists = vec![None; num_nodes];
            let mut prevs = vec![NonMax::none(); num_nodes];
            for (node_id, dist) in initial.into_iter() {
                let _ = queue.insert((dist, node_id, None));
            }
            while let Some(item) = queue.iter().next().cloned() {
                let _ = queue.remove(&item);
                let (source_dist, source_node_id, parent_node_id) = item;
                if dists[source_node_id].is_some() {
                    continue;
                }
                nodes.push(source_node_id);
                dists[source_node_id] = Some(source_dist.clone());
                prevs[source_node_id] = NonMax::from(parent_node_id);
                let edges = if let Some(edges) = edges(source_node_id, source_dist.clone()) {
                    edges
                } else {
                    break;
                };
                for edge in edges.into_iter() {
                    let target_node_id = edge.0;
                    if dists[target_node_id].is_some() {
                        continue;
                    }
                    let target_dist = edge.1;
                    assert!(
                        source_dist <= target_dist,
                        "Negative distance is not supported by algorithm"
                    );
                    let _ = queue.insert((target_dist, target_node_id, Some(source_node_id)));
                }
            }
            Dijkstra {
                nodes,
                dists,
                prevs,
            }
        }
    }
}

// ========

pub use shuffle::*;
mod shuffle {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    pub trait Shuffle {
        fn shuffle(self, seed: u64) -> Self;
    }

    impl<T> Shuffle for Vec<T> {
        fn shuffle(mut self, seed: u64) -> Self {
            let mut rand = ChaCha8Rng::seed_from_u64(seed);
            let mut result = Vec::new();
            while !self.is_empty() {
                let j = rand.gen_range(0..self.len());
                result.push(self.swap_remove(j))
            }
            result
        }
    }
}

// ========

pub use nd_array::*;
mod nd_array {
    use core::borrow::{Borrow, BorrowMut};
    use core::fmt::Debug;
    use core::marker::PhantomData;
    use core::ops::{Index, IndexMut};

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct NdArray<T, U, J, M> {
        data: U,
        dims: J,
        mults: M,
        _item: PhantomData<T>,
    }

    impl<T, J> NdArray<T, Vec<T>, J, Vec<usize>>
    where
        J: Borrow<[usize]>,
    {
        #[track_caller]
        pub fn new(value: T, dims: J) -> Self
        where
            T: Clone,
        {
            let len = dims.borrow().iter().product();
            let data = vec![value; len];
            let mults = build_mults(dims.borrow());
            let _item = PhantomData;
            Self {
                data,
                dims,
                mults,
                _item,
            }
        }

        #[track_caller]
        pub fn with<F>(f: F, dims: J) -> Self
        where
            F: FnMut() -> T,
        {
            let dims_ref = dims.borrow();
            let len = dims_ref.iter().product();
            let mut data = Vec::with_capacity(len);
            data.resize_with(len, f);
            let mults = build_mults(dims.borrow());
            let _item = PhantomData;
            Self {
                data,
                dims,
                mults,
                _item,
            }
        }
    }

    impl<T, U, J> NdArray<T, U, J, Vec<usize>>
    where
        U: Borrow<[T]>,
        J: Borrow<[usize]> + Debug + Clone,
    {
        #[track_caller]
        pub fn from(data: U, dims: J) -> Self {
            validate_data_dims(data.borrow(), dims.borrow());
            let mults = build_mults(dims.borrow());
            let _item = PhantomData;
            Self {
                data,
                dims,
                mults,
                _item,
            }
        }
    }

    impl<T, U, J, M> NdArray<T, U, J, M>
    where
        U: Borrow<[T]>,
        J: Borrow<[usize]> + Debug,
        M: Borrow<[usize]>,
    {
        pub fn into_inner(self) -> U {
            self.data
        }

        pub fn dims(&self) -> &J {
            &self.dims
        }

        fn try_offset_of_slice(&self, index: &[usize]) -> Option<usize> {
            let mut offset = 0;
            let index = index.borrow();
            let dims = self.dims.borrow();
            let mults: &[usize] = self.mults.borrow();
            for ((&i, &m), &d) in index.iter().zip(mults.iter()).zip(dims.iter()) {
                if i >= d {
                    return None;
                }
                offset += i * m;
            }
            Some(offset)
        }

        pub fn try_offset_of(&self, index: J) -> Option<usize> {
            self.try_offset_of_slice(index.borrow())
        }

        #[track_caller]
        pub fn offset_of(&self, index: J) -> usize {
            let index = index.borrow();
            if let Some(offset) = self.try_offset_of_slice(index) {
                offset
            } else {
                let dims = self.dims.borrow();
                panic!(
                    "index out of bounds: the len is {:?} but the index is {:?}",
                    dims, index
                );
            }
        }

        pub fn get(&self, index: J) -> Option<&T> {
            self.try_offset_of(index)
                .map(move |offset| &self.data.borrow()[offset])
        }
    }

    impl<T, U, J, M> NdArray<T, U, J, M>
    where
        U: BorrowMut<[T]>,
        J: Borrow<[usize]> + Debug,
        M: Borrow<[usize]>,
    {
        pub fn get_mut(&mut self, index: J) -> Option<&mut T> {
            self.try_offset_of(index)
                .map(move |offset| &mut self.data.borrow_mut()[offset])
        }
    }

    impl<T, U, J, M> NdArray<T, U, J, M>
    where
        U: BorrowMut<Vec<T>>,
        J: Borrow<[usize]>,
        M: Borrow<[usize]>,
    {
        pub fn truncate(&mut self, len: usize) {
            let vec: &mut Vec<_> = self.data.borrow_mut();
            let mults: &[usize] = self.mults.borrow();
            vec.truncate(len * mults[0]);
        }

        pub fn resize(&mut self, new_len: usize, value: T)
        where
            T: Clone,
        {
            let vec: &mut Vec<_> = self.data.borrow_mut();
            let mults: &[usize] = self.mults.borrow();
            vec.resize(new_len * mults[0], value);
        }

        pub fn resize_with<F>(&mut self, new_len: usize, f: F)
        where
            F: FnMut() -> T,
        {
            let vec: &mut Vec<_> = self.data.borrow_mut();
            vec.resize_with(new_len, f);
        }
    }

    #[track_caller]
    fn validate_data_dims<T>(data: &[T], dims: &[usize]) {
        let len = data.len();
        let prod = dims.iter().product();
        assert_eq!(
            len, prod,
            "invalid slice length: the len is {} but the product of dimensions is {}",
            len, prod
        );
    }

    fn build_mults(dims: &[usize]) -> Vec<usize> {
        let mut prod = dims.iter().product();
        dims.iter()
            .map(|dim| {
                prod /= dim;
                prod
            })
            .collect()
    }

    impl<T, U, J, M> AsRef<U> for NdArray<T, U, J, M> {
        fn as_ref(&self) -> &U {
            &self.data
        }
    }

    impl<T, U, J, M> AsMut<U> for NdArray<T, U, J, M> {
        fn as_mut(&mut self) -> &mut U {
            &mut self.data
        }
    }

    impl<T, U, J, M> Index<J> for NdArray<T, U, J, M>
    where
        U: Borrow<[T]>,
        J: Borrow<[usize]> + Debug,
        M: Borrow<[usize]>,
    {
        type Output = T;

        fn index(&self, index: J) -> &Self::Output {
            &self.data.borrow()[self.offset_of(index)]
        }
    }

    impl<T, U, J, M> IndexMut<J> for NdArray<T, U, J, M>
    where
        U: Borrow<[T]> + BorrowMut<[T]>,
        J: Borrow<[usize]> + Debug,
        M: Borrow<[usize]>,
    {
        fn index_mut(&mut self, index: J) -> &mut Self::Output {
            let offset = self.offset_of(index);
            &mut self.data.borrow_mut()[offset]
        }
    }

    impl<T, U, J, M> NdArray<T, U, J, M>
    where
        U: Borrow<[T]>,
        J: Borrow<[usize]>,
        M: Borrow<[usize]>,
    {
        pub fn at<'a>(&'a self, index: usize) -> NdArray<T, &'a [T], &'a [usize], &'a [usize]> {
            let dims = self.dims.borrow();
            let mult: usize = dims.iter().skip(1).product();
            let offset = index * mult;
            NdArray {
                data: &self.data.borrow()[offset..offset + mult],
                dims: &self.dims.borrow()[1..],
                mults: &self.mults.borrow()[1..],
                _item: PhantomData,
            }
        }
    }

    impl<T, U, J, M> NdArray<T, U, J, M>
    where
        U: BorrowMut<[T]>,
        J: Borrow<[usize]>,
        M: Borrow<[usize]>,
    {
        pub fn at_mut<'a>(
            &'a mut self,
            index: usize,
        ) -> NdArray<T, &'a mut [T], &'a [usize], &'a [usize]> {
            let dims = self.dims.borrow();
            let mult: usize = dims.iter().skip(1).product();
            let offset = index * mult;
            NdArray {
                data: &mut self.data.borrow_mut()[offset..offset + mult],
                dims: &self.dims.borrow()[1..],
                mults: &self.mults.borrow()[1..],
                _item: PhantomData,
            }
        }
    }
}

// ========

pub use modular::*;
mod modular {
    use core::cmp::{Ord, Ordering};
    use core::ops::{
        Add, AddAssign, BitAnd, Div, Mul, MulAssign, Neg, Rem, ShrAssign, Sub, SubAssign,
    };
    use std::ops::DivAssign;

    use crate::{
        gcd, Abs, Five, ModularMul, ModularPow, MulDiv, One, RemEuclid, Ten, Three, Two, Unsigned,
        Zero,
    };

    pub trait ConstValue: Default {
        type Output;
        fn get() -> Self::Output;
    }

    pub trait Value: Copy {
        type Output;
        fn get(self) -> Self::Output;
    }

    pub trait ValueEulersPhi: Value {
        fn eulers_phi(self) -> Self::Output;
    }

    impl<T: Copy> Value for (T,) {
        type Output = T;
        fn get(self) -> Self::Output {
            self.0
        }
    }

    impl<T: Copy + ConstValue> Value for T {
        type Output = <Self as ConstValue>::Output;
        fn get(self) -> Self::Output {
            Self::get()
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct ValueWithEulersPhi<T>(T, T);

    impl<T> ValueWithEulersPhi<T> {
        pub fn new(value: T, eulers_phi: T) -> Self {
            Self(value, eulers_phi)
        }
    }

    impl<T: Copy> Value for ValueWithEulersPhi<T> {
        type Output = T;
        fn get(self) -> Self::Output {
            self.0
        }
    }

    impl<T: Copy> ValueEulersPhi for ValueWithEulersPhi<T> {
        fn eulers_phi(self) -> Self::Output {
            self.1
        }
    }

    pub trait PrimeValue: Copy + ConstValue {}
    impl PrimeValue for P1_000_000_007 {}
    impl PrimeValue for P998_244_353 {}

    impl<T: PrimeValue> ValueEulersPhi for T
    where
        T::Output: One + Sub<Output = T::Output>,
    {
        fn eulers_phi(self) -> Self::Output {
            Self::get() - Self::Output::one()
        }
    }

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct P998_244_353;

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct P1_000_000_007;

    impl ConstValue for P998_244_353 {
        type Output = u32;
        fn get() -> Self::Output {
            998_244_353
        }
    }

    impl ConstValue for P1_000_000_007 {
        type Output = u32;
        fn get() -> Self::Output {
            1_000_000_007
        }
    }

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct N998_244_353;

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct N1_000_000_007;

    impl ConstValue for N998_244_353 {
        type Output = i64;
        fn get() -> Self::Output {
            998_244_353
        }
    }

    impl ConstValue for N1_000_000_007 {
        type Output = i64;
        fn get() -> Self::Output {
            1_000_000_007
        }
    }
    #[derive(Clone, Copy, Debug, Default, Hash)]
    pub struct Modular<T, M>(pub T, pub M);

    impl<T, M> Modular<T, M> {
        pub fn new(value: T, modulus: M) -> Self
        where
            T: RemEuclid + Unsigned,
            M: Clone + Value<Output = T>,
        {
            let value = value.rem_euclid(modulus.get());
            Self(value, modulus)
        }

        pub fn from_raw(value: T, modulus: M) -> Self
        where
            T: Unsigned,
            M: Value<Output = T>,
        {
            Self(value, modulus)
        }

        pub fn from_raw_value(value: T) -> Self
        where
            T: Unsigned,
            M: ConstValue + Default + Value<Output = T>,
        {
            Self(value, M::default())
        }

        pub fn value(self) -> T {
            self.0
        }

        pub fn modulus(self) -> M {
            self.1
        }
    }

    impl<T, M> From<T> for Modular<T, M>
    where
        T: RemEuclid + Unsigned,
        M: Copy + ConstValue<Output = T>,
    {
        fn from(value: T) -> Self {
            Self::new(value, M::default())
        }
    }

    impl<T, M> Neg for Modular<T, M>
    where
        T: Add<Output = T> + PartialOrd + RemEuclid + Sub<Output = T> + Zero,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn neg(self) -> Self {
            if self.0.is_zero() {
                self
            } else {
                Self(self.1.get() - self.0, self.1)
            }
        }
    }

    impl<T, M> Add for Modular<T, M>
    where
        T: Add<Output = T> + PartialOrd + RemEuclid + Sub<Output = T>,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            let value = self.0 + other.0;
            if value >= self.1.get() {
                Self(value - self.1.get(), self.1)
            } else {
                Self(value, self.1)
            }
        }
    }

    impl<T, M> Sub for Modular<T, M>
    where
        T: Add<Output = T> + PartialOrd + Sub<Output = T> + Zero,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            if self.0 >= other.0 {
                Self(self.0 - other.0, self.1)
            } else {
                Self(self.0 + self.1.get() - other.0, self.1)
            }
        }
    }

    impl<T, M> Mul for Modular<T, M>
    where
        T: ModularMul,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            Self(self.0.modular_mul(other.0, self.1.get()), self.1)
        }
    }

    impl<T, M> Div for Modular<T, M>
    where
        T: ModularMul + ModularPow<T> + Sub<Output = T> + Two + Zero,
        M: PrimeValue<Output = T>,
    {
        type Output = Self;

        fn div(self, other: Self) -> Self {
            self.mul(other.inv().unwrap())
        }
    }

    impl<T, M> AddAssign for Modular<T, M>
    where
        T: Add<Output = T> + Copy + PartialOrd + RemEuclid + Sub<Output = T>,
        M: Value<Output = T>,
    {
        fn add_assign(&mut self, other: Self) {
            self.0 = (*self + other).0
        }
    }

    impl<T, M> SubAssign for Modular<T, M>
    where
        T: Add<Output = T> + Copy + PartialOrd + Sub<Output = T> + Zero,
        M: Value<Output = T>,
    {
        fn sub_assign(&mut self, other: Self) {
            self.0 = (*self - other).0
        }
    }

    impl<T, M> MulAssign for Modular<T, M>
    where
        T: Copy + ModularMul,
        M: Value<Output = T>,
    {
        fn mul_assign(&mut self, other: Self) {
            self.0 = (*self * other).0
        }
    }

    impl<T, M> DivAssign for Modular<T, M>
    where
        T: Copy + ModularMul + ModularPow<T> + Sub<Output = T> + Two + Zero,
        M: PrimeValue<Output = T>,
    {
        fn div_assign(&mut self, other: Self) {
            self.0 = (*self / other).0
        }
    }

    impl<T, M> Add<T> for Modular<T, M>
    where
        Self: Add<Self, Output = Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn add(self, other: T) -> Self {
            let modulus = self.1;
            self + Self::new(other, modulus)
        }
    }

    impl<T, M> Sub<T> for Modular<T, M>
    where
        Self: Sub<Self, Output = Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn sub(self, other: T) -> Self {
            let modulus = self.1;
            self - Self::new(other, modulus)
        }
    }

    impl<T, M> Mul<T> for Modular<T, M>
    where
        Self: Mul<Self, Output = Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn mul(self, other: T) -> Self {
            let modulus = self.1;
            self * Self::new(other, modulus)
        }
    }

    impl<T, M> Div<T> for Modular<T, M>
    where
        Self: Div<Self, Output = Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn div(self, other: T) -> Self {
            let modulus = self.1;
            self / Self::new(other, modulus)
        }
    }

    impl<T, M> AddAssign<T> for Modular<T, M>
    where
        Self: AddAssign<Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        fn add_assign(&mut self, other: T) {
            let modulus = self.1;
            *self += Self::new(other, modulus);
        }
    }

    impl<T, M> SubAssign<T> for Modular<T, M>
    where
        Self: SubAssign<Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        fn sub_assign(&mut self, other: T) {
            let modulus = self.1;
            *self -= Self::new(other, modulus);
        }
    }

    impl<T, M> MulAssign<T> for Modular<T, M>
    where
        Self: MulAssign<Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        fn mul_assign(&mut self, other: T) {
            let modulus = self.1;
            *self *= Self::new(other, modulus);
        }
    }

    impl<T, M> DivAssign<T> for Modular<T, M>
    where
        Self: DivAssign<Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        fn div_assign(&mut self, other: T) {
            let modulus = self.1;
            *self /= Self::new(other, modulus);
        }
    }

    impl<T, M> Modular<T, M> {
        pub fn pow<U>(self, exp: U) -> Self
        where
            T: ModularPow<U>,
            M: Value<Output = T>,
        {
            Self(self.0.modular_pow(exp, self.1.get()), self.1)
        }
    }

    impl<T, M> Modular<T, M>
    where
        T: ModularPow<T> + Sub<Output = T> + Two + Zero,
        M: PrimeValue<Output = T>,
    {
        // https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
        // p: prime && a % p > 0
        // => a ** (p - 1) % p = 1
        // => a ** (p - 2) % p = a ** -1 % p
        pub fn inv(self) -> Option<Self> {
            if self.0.is_zero() {
                None
            } else {
                Some(self.pow(M::get() - T::two()))
            }
        }
    }

    impl<T, M> Modular<T, M>
    where
        T: Abs + Copy + ModularPow<T> + One + PartialOrd + Rem<Output = T> + Sub<Output = T> + Zero,
        M: Value<Output = T> + ValueEulersPhi,
    {
        pub fn inv_with_eulers_phi(self) -> Option<Self> {
            if self.0.is_zero() || gcd(self.0, self.1.get()) > T::one() {
                None
            } else {
                Some(self.pow(self.1.eulers_phi() - T::one()))
            }
        }
    }

    impl<T: Ord, M> Eq for Modular<T, M> {}

    impl<T: Ord, M> Ord for Modular<T, M> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl<T: Ord, M> PartialEq for Modular<T, M> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl<T: Ord, M> PartialOrd for Modular<T, M> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl<T: Ord, M> PartialEq<T> for Modular<T, M> {
        fn eq(&self, other: &T) -> bool {
            self.0.eq(&other)
        }
    }

    impl<T: Ord, M> PartialOrd<T> for Modular<T, M> {
        fn partial_cmp(&self, other: &T) -> Option<Ordering> {
            self.0.partial_cmp(&other)
        }
    }

    macro_rules! def {
        ( $name:ident, $fn:ident, $is:ident ) => {
            impl<T, M> $name for Modular<T, M>
            where
                T: $name + PartialOrd + Unsigned,
                M: ConstValue + Default + Value<Output = T>,
            {
                fn $fn() -> Self {
                    assert!(T::$fn() < M::default().get());
                    Self::from_raw_value(T::$fn())
                }

                fn $is(&self) -> bool {
                    self.0 == T::$fn()
                }
            }
        };
    }

    def!(Zero, zero, is_zero);
    def!(One, one, is_one);
    def!(Two, two, is_two);
    def!(Three, three, is_three);
    def!(Five, five, is_five);
    def!(Ten, ten, is_ten);

    impl<T, M> Unsigned for Modular<T, M> {}

    impl<T, M> MulDiv for Modular<T, M>
    where
        Self: Div<Output = Self> + Mul<Output = Self>,
    {
        fn mul_div(self, mul: Self, div: Self) -> Self {
            self.mul(mul).div(div)
        }
    }

    pub type M1_000_000_007 = Modular<u32, P1_000_000_007>;
    pub type M998_244_353 = Modular<u32, P998_244_353>;

    pub type M07 = M1_000_000_007;
    pub type M53 = M998_244_353;
}
