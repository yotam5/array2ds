use std::iter::{IntoIterator, Iterator};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Array2d<T> {
    vec_slice: Box<[T]>,
    no_rows: usize,
}

pub struct IterArr2d<'a,T>
{
    inner: &'a [T],
    index: usize,
}

impl<T> Array2d<T> {
    /// create a new 2d array each elem of type T no
    pub fn filled_with(element: T, r: usize, c: usize) -> Self
    where
        T: Clone,
    {
        let v = vec![element; r * c];
        let vb = v.into_boxed_slice();
        Array2d {
            vec_slice: vb,
            no_rows: r,
        }
    }

    pub fn row_count(&self) -> usize {
        self.no_rows
    }

    pub fn column_count(&self) -> usize {
        self.vec_slice.len() % self.row_count()
    }

    pub fn d2_index_d1(&self, (r, c): (usize, usize)) -> usize {
        self.row_count() * r + c
    }
}

impl<T> Index<(usize, usize)> for Array2d<T> {
    type Output = T;
    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.vec_slice[self.d2_index_d1((r, c))]
    }
}

impl<T> IndexMut<(usize, usize)> for Array2d<T> {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        &mut self.vec_slice[self.d2_index_d1((r, c))]
    }
}
