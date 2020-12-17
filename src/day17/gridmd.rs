use crate::day17::main::{MAX_POS, MAX_NEG};
use std::ops::{Index, IndexMut, RangeFrom, Range};
use std::slice::SliceIndex;

pub struct GridMD<T: Clone, const DIM: usize> {
    pub vec: Vec<T>
}

impl<T: Clone, const DIM: usize> GridMD<T, DIM> {
    pub(crate) fn new(default: T) -> Self {
        let size = (MAX_POS + MAX_NEG + 1).pow(DIM as u32);
        let size_rounded_up = (size + 64 - 1) / 64 * 64;
        assert_eq!(size_rounded_up%64, 0);
        GridMD { vec: vec![default; 64 + size_rounded_up + 64]}
    }

    fn index_to_final(index: [isize; DIM]) -> usize {
        index.iter()
            .map(|&i| (i + MAX_NEG as isize) as usize)
            .enumerate()
            .map(|(i, index)| (MAX_POS + MAX_NEG).pow((DIM-i-1) as u32) * index)
            .sum::<usize>() + 64
    }
}

//Index operations

impl<T: Clone, const DIM: usize> Index<[isize; DIM]> for GridMD<T, DIM> {
    type Output = T;

    #[inline]
    fn index(&self, index: [isize; DIM]) -> &T {
        &self.vec[GridMD::<T, DIM>::index_to_final(index)]
    }
}

impl<T: Clone, const DIM: usize> IndexMut<[isize; DIM]> for GridMD<T, DIM> {
    #[inline]
    fn index_mut(&mut self, index: [isize; DIM]) -> &mut T {
        &mut self.vec[GridMD::<T, DIM>::index_to_final(index)]
    }
}

//Slice operations

impl<T: Clone, const DIM: usize> Index<RangeFrom<[isize; DIM]>> for GridMD<T, DIM> {
    type Output = [T];

    fn index(&self, index: RangeFrom<[isize; DIM]>) -> &Self::Output {
        &self.vec[GridMD::<T, DIM>::index_to_final(index.start)..]
    }
}