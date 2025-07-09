pub use util_each_permutation::*;
mod util_each_permutation {
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
                data: (k <= data.len()).then_some(data),
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
                    if self.idxs.is_empty() {
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

        #[must_use]
        pub fn get(&self) -> Ref<'_, [&'a T]> {
            Ref::map(self.0.borrow(), Vec::as_slice)
        }
    }
}
