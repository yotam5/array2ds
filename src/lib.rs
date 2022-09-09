pub mod array2d
{
    use core::iter::Iterator;
    use core::mem;
    use core::ops::{Index, IndexMut};

    /// trait that is used for indexing the 2d array
    pub trait GridIdx {
        fn no_row(&self) -> usize;
        fn no_column(&self) -> usize;
    }


    /// wrapper struct for iterating over rows
    pub struct Rows<'a, T>
    {
        pub(super) v: &'a [T],
        pub(super) skip_cols: usize,

    }

    impl<'a, T> Iterator for Rows<'a, T>
    {
        type Item = &'a [T];

        fn next(&mut self) -> Option<Self::Item> {
            if self.v.is_empty() {
                return None;
            }
            let (fst, snd) = self.v.split_at(self.skip_cols);
            if snd.is_empty() {
                self.v = &[];
            } else {
                self.v = snd.get(self.skip_cols..).unwrap();
            }
            Some(fst)
        }
    }


    /// wrapper struct for iterating over mutable rows
    pub struct RowsMut<'a, T>
    {
        pub(super) v: &'a mut [T],
        pub(super) no_cols: usize,
        pub(super) skip_cols: usize,
    }

    impl<'a, T> Iterator for RowsMut<'a, T>
    {
        type Item = &'a mut [T];

        fn next(&mut self) -> Option<Self::Item> {
            if !self.v.is_empty() && self.skip_cols < self.no_cols
            {
                let tmp = mem::take(&mut self.v);
                let (head, tail) = tmp.split_at_mut(self.no_cols);
                if tail.is_empty() {
                    self.v = &mut [];
                } else {
                    self.v = tail.get_mut(self.skip_cols..).unwrap()
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
    impl<T> Default for Array2d<T>
    {
        fn default() -> Self {
            Array2d {
                vec_slice: Box::new([]),
                no_rows: 0,
                no_columns: 0,
            }
        }
    }

    impl<T> Array2d<T> {
        /// create a new 2d array each elem of type T
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

        /// return the 2d array as 1d slice iterable
        pub fn iter(&self) -> impl Iterator<Item=&T>
        {
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
        pub fn mut_row(&mut self, row_index: usize) -> &mut [T]
        {
            let (start, end) = self.row_between(row_index);
            &mut self.vec_slice[start..end]
        }


        /// return row as mutable iterable
        pub fn iter_mut_row(&mut self, row_index: usize) -> impl Iterator<Item=&mut T> {
            let (start, end) = self.row_between(row_index);
            self.vec_slice[start..end].iter_mut()
        }

        /// iterate over the rows as mutable
        pub fn iter_mut_rows(&mut self) -> RowsMut<'_, T>
        {
            let c = self.column_count();
            RowsMut {
                v: &mut self.vec_slice,
                no_cols: c,
                skip_cols: 0,
            }
        }

        /// iterate over the rows
        pub fn iter_rows(&self) -> impl Iterator<Item=impl Iterator<Item=&T>> {
            (0_usize..self.row_count()).map(move |row_index| self.iter_row(row_index))
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