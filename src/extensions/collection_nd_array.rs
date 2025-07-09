pub use collection_nd_array::*;
mod collection_nd_array {
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
            Self {
                data,
                dims,
                mults,
                _item: PhantomData,
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
            Self {
                data,
                dims,
                mults,
                _item: PhantomData,
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
            Self {
                data,
                dims,
                mults,
                _item: PhantomData,
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

        pub const fn dims(&self) -> &J {
            &self.dims
        }

        fn try_offset_of_slice(&self, index: &[usize]) -> Option<usize> {
            let mut offset = 0;
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
            self.try_offset_of_slice(index).unwrap_or_else(|| {
                let dims = self.dims.borrow();
                panic!("index out of bounds: the len is {dims:?} but the index is {index:?}");
            })
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
            "invalid slice length: the len is {len} but the product of dimensions is {prod}"
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
        #[track_caller]
        pub fn at(&self, index: usize) -> NdArray<T, &[T], &[usize], &[usize]> {
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
        pub fn at_mut(&mut self, index: usize) -> NdArray<T, &mut [T], &[usize], &[usize]> {
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
