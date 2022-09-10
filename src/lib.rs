pub mod array2d {
    use core::iter::{DoubleEndedIterator, Iterator};
    use core::mem;
    use core::ops::{Index, IndexMut};

    /// trait that is used for indexing the 2d array
    pub trait GridIdx {
        fn no_row(&self) -> usize;
        fn no_column(&self) -> usize;
    }

    #[derive(Debug)]
    pub struct ColumMut<'a, T>
    {
        pub(super) v: &'a mut [T],
        pub(super) skip: usize,
    }

    impl<'a, T> Index<usize> for ColumMut<'a, T>
    {
        type Output = T;

        fn index(&self, idx: usize) -> &Self::Output {
            let pos = idx * (1 + self.skip);
            &self.v[pos]
        }
    }

    impl<'a, T> IndexMut<usize> for ColumMut<'a, T>
    {
        /// indexing the column, mutable
        fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
            let pos = idx * (1 + self.skip);
            &mut self.v[pos]
        }
    }

    impl<'a, T> Iterator for ColumMut<'a, T>
    {
        type Item = &'a mut T;

        /// next item in the column, mutable
        fn next(&mut self) -> Option<Self::Item> {
            let tmp = mem::take(&mut self.v);
            if let Some((fst, snd)) = tmp.split_first_mut() {
                if snd.is_empty() {
                    self.v = &mut [];
                } else {
                    self.v = snd.get_mut(self.skip..).unwrap();
                }
                Some(fst)
            } else {
                None
            }
        }
    }

    #[derive(Debug)]
    pub struct Column<'a, T>
    {
        pub(super) v: &'a [T],
        pub(super) skip: usize,
    }

    impl<'a, T> Index<usize> for Column<'a, T>
    {
        type Output = T;

        fn index(&self, idx: usize) -> &Self::Output {
            let pos = idx * (1 + self.skip);
            &self.v[pos]
        }
    }

    impl<'a, T> Iterator for Column<'a, T>
    {
        type Item = &'a T;

        /// next item in the column
        fn next(&mut self) -> Option<Self::Item> {
            if let Some((fst, snd)) = self.v.split_first() {
                if snd.is_empty() {
                    self.v = &[];
                } else {
                    self.v = snd.get(self.skip..).unwrap();
                }
                Some(fst)
            } else {
                None
            }
        }
    }

    /// wrapper struct for iterating over rows
    #[derive(Debug)]
    pub struct Rows<'a, T> {
        pub(super) v: &'a [T],
        pub(super) columns: usize,
        pub(super) skip_columns: usize,
    }

    impl<'a, T> DoubleEndedIterator for Rows<'a, T>
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.v.is_empty() {
                None
            } else {
                let (fst, snd) = self.v.split_at(self.v.len() - self.columns);
                if fst.is_empty() {
                    self.v = &[];
                } else {
                    self.v = fst.get(..fst.len() - self.skip_columns).unwrap();
                }
                Some(snd)
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            let (adj,overflow) = n.overflowing_mul(self.columns + self.skip_columns);
            if adj >= self.v.len() || overflow{
                self.v = &[];
            }else{
                self.v = self.v.get(..self.v.len() - adj).unwrap();
            }
            self.next_back()
        }


    }

    impl<'a, T> Iterator for Rows<'a, T>
    {
        type Item = &'a [T];

        fn next(&mut self) -> Option<Self::Item> {
            if self.v.is_empty() {
                None
            } else {
                let (fst, snd) = self.v.split_at(self.columns);
                if snd.is_empty() {
                    self.v = &[];
                } else {
                    self.v = snd.get(self.skip_columns..).unwrap();
                }

                Some(fst)
            }
        }
    }

    /// wrapper struct for iterating over mutable rows
    #[derive(Debug)]
    pub struct RowsMut<'a, T> {
        pub(super) v: &'a mut [T],
        pub(super) no_columns: usize,
        pub(super) skip_columns: usize,
    }

    impl<'a, T> DoubleEndedIterator for RowsMut<'a, T>
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.v.is_empty() {
                None
            } else {
                let tmp = mem::take(&mut self.v);
                let tmp_len = tmp.len();
                let (fst, snd) = tmp.split_at_mut(tmp_len - self.no_columns);
                if fst.is_empty() {
                    self.v = &mut [];
                } else {
                    self.v = fst.get_mut(..tmp_len - self.no_columns - self.skip_columns).unwrap();
                }
                Some(snd)
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            let (adj,overflow)  = n.overflowing_mul(self.no_columns + self.skip_columns);
            if adj >= self.v.len() || overflow{
                self.v = &mut [];
            }
            else {
                let tmp = mem::take(&mut self.v);
                self.v = tmp.get_mut(..self.v.len() - adj).unwrap();
            }

            self.next_back()
        }
    }

    impl<'a, T> Iterator for RowsMut<'a, T> {
        type Item = &'a mut [T];

        fn next(&mut self) -> Option<Self::Item> {
            if !self.v.is_empty() && self.skip_columns < self.no_columns {
                let tmp = mem::take(&mut self.v);
                let (head, tail) = tmp.split_at_mut(self.no_columns);
                if tail.is_empty() {
                    self.v = &mut [];
                } else {
                    self.v = tail.get_mut(self.skip_columns..).unwrap()
                }
                return Some(head);
            }
            None
        }
    }

    /// struct that expresses index in 2d array
    pub struct GridPos {
        pub row: usize,
        pub column: usize,
    }

    impl GridPos {
        pub fn new(r: usize, c: usize) -> Self {
            Self { row: r, column: c }
        }
    }

    impl GridIdx for GridPos {
        fn no_row(&self) -> usize {
            self.row
        }

        fn no_column(&self) -> usize {
            self.column
        }
    }

    impl GridIdx for (usize, usize) {
        fn no_row(&self) -> usize {
            self.0
        }

        fn no_column(&self) -> usize {
            self.1
        }
    }

    impl GridIdx for [usize; 2] {
        fn no_row(&self) -> usize {
            self[0]
        }

        fn no_column(&self) -> usize {
            self[1]
        }
    }

    #[derive(Debug)]
    /// the main struct for the 2d array
    pub struct Array2d<T> {
        vec_slice: Box<[T]>,
        no_rows: usize,
        no_columns: usize,
    }

    /// default implementation, creates an empty array
    impl<T> Default for Array2d<T> {
        fn default() -> Self {
            Array2d {
                vec_slice: Box::new([]),
                no_rows: 0,
                no_columns: 0,
            }
        }
    }

    impl<T> Array2d<T> {
        /// create a new 2d array each elem of type T where T is clonable
        pub fn filled_with(element: T, r: usize, c: usize) -> Self
            where
                T: Clone,
        {
            assert!(r >= 1 && c >= 1);
            let v = vec![element; r * c];
            let vb = v.into_boxed_slice();
            Array2d {
                vec_slice: vb,
                no_rows: r,
                no_columns: c,
            }
        }

        // get mutable column
        pub fn column_mut(&mut self, no_column: usize) -> ColumMut<'_, T>
        {
            assert!(no_column < self.column_count());
            let c = self.column_count();
            ColumMut {
                v: self.vec_slice.get_mut(no_column..self.vec_slice.len() - self.column_count() + no_column + 1).unwrap(),
                skip: c - 1,
            }
        }

        /// get column
        pub fn column(&self, no_column: usize) -> Column<'_, T>
        {
            assert!(no_column < self.column_count());
            Column {
                v: self.vec_slice.get(no_column..self.vec_slice.len() - self.column_count() + no_column + 1).unwrap(),
                skip: self.column_count() - 1,
            }
        }

        /// create a new 2d array each elem of type T where T is the default implementation
        pub fn filled_with_default(r: usize, c: usize) -> Self
            where
                T: Default,
        {
            assert!(r >= 1 && c >= 1);
            let mut v = Vec::with_capacity(r * c);
            for _ in 0..(r * c) {
                v.push(T::default());
            }
            let vb = v.into_boxed_slice();
            Array2d {
                vec_slice: vb,
                no_rows: r,
                no_columns: c,
            }
        }

        /// return the 2d array as 1d slice iterable
        pub fn iter(&self) -> impl Iterator<Item=&T> {
            self.vec_slice.iter()
        }

        /// return the row count
        pub fn row_count(&self) -> usize {
            self.no_rows
        }

        /// return the column count
        pub fn column_count(&self) -> usize {
            self.no_columns
        }

        /// convert 2d position to 1d position row_to * column_count + column_to, row_major
        pub fn d2_index_d1<F>(&self, pos: &F) -> usize
            where
                F: GridIdx,
        {
            pos.no_row() * self.column_count() + pos.no_column()
        }

        /// swap two position values
        pub fn swap<F, K>(&mut self, pos1: &F, pos2: &K)
            where
                F: GridIdx,
                K: GridIdx,
        {
            let converted_rc1 = self.d2_index_d1(pos1);
            let converted_rc2 = self.d2_index_d1(pos2);
            self.vec_slice.swap(converted_rc1, converted_rc2);
        }

        /// return the row between containing the index
        pub fn row_between(&self, row_index: usize) -> (usize, usize) {
            assert!(row_index < self.row_count());
            let start = row_index * self.column_count();
            let end = start + self.column_count();
            (start, end)
        }

        /// return row as iterable
        pub fn iter_row(&self, row_index: usize) -> impl Iterator<Item=&T> {
            let (start, end) = self.row_between(row_index);
            self.vec_slice[start..end].iter()
        }

        /// return row as mutable
        pub fn mut_row(&mut self, row_index: usize) -> &mut [T] {
            let (start, end) = self.row_between(row_index);
            &mut self.vec_slice[start..end]
        }

        /// return row as mutable iterable
        pub fn iter_mut_row(&mut self, row_index: usize) -> impl Iterator<Item=&mut T> {
            let (start, end) = self.row_between(row_index);
            self.vec_slice[start..end].iter_mut()
        }

        /// iterate over the rows as mutable
        pub fn iter_mut_rows(&mut self) -> RowsMut<'_, T> {
            let c = self.column_count();
            RowsMut {
                v: &mut self.vec_slice,
                no_columns: c,
                skip_columns: 0,
            }
        }

        /// iterate over the rows
        pub fn iter_rows(&self) -> Rows<'_, T> {
            //impl Iterator<Item=impl Iterator<Item=&T>> {
            //(0_usize..self.row_count()).map(move |row_index| self.iter_row(row_index))
            let c = self.column_count();
            Rows {
                v: &self.vec_slice,
                columns: c,
                skip_columns: 0,
            }
        }

        pub fn as_slice(&self) -> &[T] {
            &self.vec_slice
        }
    }

    impl<T, Idx: GridIdx> Index<Idx> for Array2d<T> {
        type Output = T;
        fn index(&self, index: Idx) -> &Self::Output {
            &self.vec_slice[self.d2_index_d1(&GridPos::new(index.no_row(), index.no_column()))]
        }
    }

    impl<T, Idx: GridIdx> IndexMut<Idx> for Array2d<T> {
        fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
            &mut self.vec_slice[self.d2_index_d1(&GridPos::new(index.no_row(), index.no_column()))]
        }
    }
}
