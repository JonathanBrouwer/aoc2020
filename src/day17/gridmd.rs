use crate::day17::main::{MAX_POS, MAX_NEG};
use std::ops::{Index, IndexMut};

pub struct GridMD<const DIM: usize> {
    pub vec: Vec<bool>
}

impl<const DIM: usize> GridMD<DIM> {
    pub(crate) fn new() -> Self {
        GridMD { vec: vec![false; (MAX_POS + MAX_NEG + 1).pow(DIM as u32) ]}
    }
}

impl<const DIM: usize> Index<[isize; DIM]> for GridMD<DIM> {
    type Output = bool;

    #[inline]
    fn index(&self, index: [isize; DIM]) -> &bool {
        let final_index: usize = index.iter()
            .map(|&i| (i + MAX_NEG as isize) as usize)
            .enumerate()
            .map(|(i, index)| (MAX_POS + MAX_NEG).pow((DIM-i-1) as u32) * index)
            .sum();
        &self.vec[final_index]
    }
}

impl<const DIM: usize> IndexMut<[isize; DIM]> for GridMD<DIM> {
    #[inline]
    fn index_mut(&mut self, index: [isize; DIM]) -> &mut bool {
        let final_index: usize = index.iter()
            .map(|&i| (i + MAX_NEG as isize) as usize)
            .enumerate()
            .map(|(i, index)| (MAX_POS + MAX_NEG).pow((DIM-i-1) as u32) * index)
            .sum();
        &mut self.vec[final_index]
    }
}