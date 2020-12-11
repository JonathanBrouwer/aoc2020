use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Vec2D<T> {
    pub vec: Vec<T>,
    pub dim: (usize, usize)
}

impl<T> Index<usize> for Vec2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.vec[index.0*self.dim.1+index.1]
    }
}

impl<T> IndexMut<usize> for Vec2D<T> {

    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {

    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.vec[index.0*self.dim.1+index.1]
    }
}