use crate::day17::main::{MAX_POS, MAX_NEG, MIN, MAX};
use std::ops::{Index, IndexMut, RangeFrom, Range};
use std::slice::SliceIndex;
use crate::day17::gridmd_iterator::GridMDIterator;
use std::fmt::Debug;
use itertools::__std_iter::Skip;

pub struct GridMD<T: Clone, const DIM: usize> {
    pub vec: Vec<T>
}

impl<T: Clone, const DIM: usize> GridMD<T, DIM> {
    pub(crate) fn new(default: T) -> Self {
        let size = (MAX_NEG + 1 + MAX_POS).pow(DIM as u32);
        let size_rounded_up = (size + 64 - 1) / 64 * 64;
        assert_eq!(size_rounded_up%64, 0);
        GridMD { vec: vec![default; 64 + size_rounded_up + 100000]}
    }

    #[inline]
    pub fn index_to_final(index: [isize; DIM]) -> usize {
        index.iter()
            .map(|&i| i + MAX_NEG as isize)
            .enumerate()
            .map(|(i, index)| (MAX_POS + 1 + MAX_NEG).pow((DIM-i-1) as u32) as isize * index)
            .sum::<isize>() as usize + 64
    }

    #[inline]
    pub fn iter_all(&self) -> Skip<GridMDIterator<DIM, MIN, MAX>> {
        let to_skip: usize = (0..DIM).map(|p| (MAX_NEG + 1 + MAX_POS).pow(p as u32)).sum();
        GridMDIterator::<DIM, MIN, MAX>::new().skip(to_skip)
    }
}

impl<T: Clone + Debug, const DIM: usize> GridMD<T, DIM> {
    pub fn print(&self) {
        if DIM == 2 {
            for y in -(MAX_NEG as isize)..=MAX_POS as isize {
                for x in -(MAX_NEG as isize)..=MAX_POS as isize {
                    let mut index = [0; DIM];
                    index[0] = x; index[1] = y;
                    print!("{:?}", self[index]);
                }
                println!();
            }
        }
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

    #[inline]
    fn index(&self, index: RangeFrom<[isize; DIM]>) -> &Self::Output {
        &self.vec[GridMD::<T, DIM>::index_to_final(index.start)..]
    }
}

impl<T: Clone, const DIM: usize> IndexMut<RangeFrom<[isize; DIM]>> for GridMD<T, DIM> {
    #[inline]
    fn index_mut(&mut self, index: RangeFrom<[isize; DIM]>) -> &mut [T] {
        &mut self.vec[GridMD::<T, DIM>::index_to_final(index.start)..]
    }
}