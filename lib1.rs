use std::convert::From;
use std::iter::{IntoIterator, Iterator};
use std::ops::{Index, IndexMut};
use std::cell::RefCell;

pub trait GridIdx {
    fn no_row(&self) -> usize;
    fn no_column(&self) -> usize;
}


struct RowIter<'a,T>
{
    next: Option<&'a Array2d<T>>,

}

impl<'a,T: 'a> Iterator for RowIter<'a,T>
{
    type Item = &'a [T];
   fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map
   } 
}

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
pub struct Array2d<T> {
    vec_slice: RefCell<Box<[T]>>,
    no_rows: usize,
}

pub struct Array2dIter<'a, T> {
    inner: &'a [T],
    index: usize,
}

impl<T> Array2d<T> {
    /// create a new 2d array each elem of type T no
    pub fn filled_with(element: T, r: usize, c: usize) -> Self
    where
        T: Clone,
    {
        assert!(r >= 1 && c >= 1);
        let v = vec![element; r * c];
        let vb = v.into_boxed_slice();
        Array2d {
            vec_slice: RefCell::new(vb),
            no_rows: r,
        }
    }

    pub fn row_count(&self) -> usize {
        self.no_rows
    }

    pub fn column_count(&self) -> usize {
        self.vec_slice.borrow().len() / self.row_count()
    }

    /// convert 2d position to 1d position row_len * row_index + col_index
    pub fn d2_index_d1<F>(&self, pos: &F) -> usize
    where
        F: GridIdx,
    {
        //println!("row: {} column: {}",pos.no_row(),pos.no_column());
        pos.no_row() * (self.row_count() - 1) + pos.no_column()
    }

    /// swap two position
    pub fn swap<F, K>(&mut self, pos1: &F, pos2: &K)
    where
        F: GridIdx,
        K: GridIdx,
    {
        let converted_rc1 = self.d2_index_d1(pos1);
        let converted_rc2 = self.d2_index_d1(pos2);
        self.vec_slice.borrow_mut().swap(converted_rc1, converted_rc2);
    }

    pub fn row_between(&self, row_index: usize) -> (usize, usize) {
        assert!(row_index < self.row_count());
        let start = row_index * self.column_count();
        let end = start + self.column_count();
        (start, end)
    }
//std::option::Iter<&[T]
    pub fn iter_row(&self, row_index: usize) -> &std::slice::Iter<T>{
        let (start, end) = self.row_between(row_index);
       &self.vec_slice.borrow().get(start..end).unwrap().iter()
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        let m = (0..self.row_count()).map(|r| self.iter_row(r));
    }


    pub fn iter_mut_row(&mut self, row_index: usize) -> std::option::IterMut<&mut [T]>{
        let (start, end) = self.row_between(row_index);
        self.vec_slice.get_mut().get_mut(start..end).iter_mut()
    }

    pub fn iter_mut_rows(&mut self) -> std::option::IterMut<&mut [T]>{
    {
        (0..self.row_count()).map(|ri| self.iter_mut_row(ri))
        
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
        &mut self.vec_slice[self.d2_index_d1(&GridPos::new(index.no_row(), index.no_column()
    }
}
