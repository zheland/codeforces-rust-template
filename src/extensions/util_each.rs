pub use util_each::*;
mod util_each {
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
