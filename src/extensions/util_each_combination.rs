pub use util_each_combination::*;
mod util_each_combination {
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
                data,
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
                            #[allow(clippy::never_loop)]
                            'outer: loop {
                                for bit in &mut comb.mask {
                                    *bit = !*bit;
                                    if *bit {
                                        self.num += 1;
                                        break 'outer;
                                    }
                                    self.num -= 1;
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
                }
                comb.j += 1;
            }
            None
        }
    }

    impl<T> FusedIterator for EachCombinationIterIter<'_, T> {}
}
