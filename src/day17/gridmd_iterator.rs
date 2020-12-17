pub struct GridMDIterator<const DIM: usize, const MIN: isize, const MAX: isize> {
    pub(crate) last: [isize; DIM]
}

impl<const DIM: usize, const MIN: isize, const MAX: isize> GridMDIterator<DIM, MIN, MAX> {
    pub(crate) fn new() -> Self {
        let mut res = GridMDIterator { last: [MIN; DIM] };
        res.last[DIM-1] = MIN-1;
        res
    }
}

impl<const DIM: usize, const MIN: isize, const MAX: isize> Iterator for GridMDIterator<DIM, MIN, MAX> {
    type Item = [isize; DIM];

    fn next(&mut self) -> Option<Self::Item> {
        for i in (0..DIM).rev() {
            if self.last[i] == MAX {
                self.last[i] = MIN;
            } else {
                self.last[i] += 1;
                return Some(self.last);
            }
        }
        None
    }
}