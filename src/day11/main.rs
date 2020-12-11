extern crate test;
extern crate strum;
extern crate strum_macros;
extern crate bitvec;

use itertools::iproduct;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::mem::swap;

fn part1(inp: &str) -> usize {
    solve_generic(inp, calc_neighbours_p1, 4)
}

fn part2(inp: &str) -> usize {
    solve_generic(inp, calc_neighbours_p2, 5)
}

#[inline]
fn solve_generic(inp: &str, calc_neighbours: fn((usize, usize), &(usize, usize), &Vec<bool>) -> Vec<usize>, seatfull_swap_min: usize) -> usize {
    let dim = (inp.lines().count(), inp.lines().next().unwrap().len());

    //Keep track of bitmaps for each state
    let mut seat_bitmap: Vec<_> = inp.chars().filter(|&c| c != '\n').map(|c| c == 'L').collect();
    let mut old_seat_bitmap = seat_bitmap.clone();

    //Calc seat positions
    let seat_positions: Vec<_> = seat_bitmap.iter().enumerate().filter(|&(_, v)| *v).map(|(i, _)| i).collect();

    //Calc neighbours for each state
    let neighbours: Vec<_> = seat_bitmap.iter().enumerate().map(|(i, &p)| {
        if !p { return Vec::new(); }
        let pos = (i / dim.1, i % dim.1);
        calc_neighbours(pos, &dim, &seat_bitmap)
    }).collect();

    loop {
        //Keep track of old state and changed
        let mut changed = false;

        //For each position
        for &pos in &seat_positions {
            //Match on old state and the amount of neighbours, return new state
            let nb_count = neighbours[pos].iter().filter(|&&p| old_seat_bitmap[p]).count();
            seat_bitmap[pos] = match (old_seat_bitmap[pos], nb_count) {
                (false, 0) => true,
                (false, _) => false,
                (true, v) if (seatfull_swap_min..).contains(&v) => false,
                (true, _) => true,
            };
            //Store if state changed
            changed |= seat_bitmap[pos] ^ old_seat_bitmap[pos];
        }

        //If nothing changed, break
        if !changed { break; }

        //Swap
        swap(&mut seat_bitmap, &mut old_seat_bitmap)
    }

    //Count amount of full seats
    return seat_positions.iter().filter(|&&pos| seat_bitmap[pos]).count();
}

#[inline]
fn calc_neighbours_p1(pos: (usize, usize), dim: &(usize, usize), seatlocs: &Vec<bool>) -> Vec<usize> {
    //Count amount of directions for which there is a seat with a person on it
    Direction::iter().map(|dir: Direction| {
        //Find the position in this direction
        dir.apply_to(pos.0, pos.1, dim.0, dim.1, 1)
            //Take only positions that are a seat
            .filter(|&(ni, nj)| seatlocs[ni * dim.1 + nj])
            //Map to 1d
            .map(|pos| pos.0 * dim.1 + pos.1)
    }).flatten().collect()
}

#[inline]
fn calc_neighbours_p2(pos: (usize, usize), dim: &(usize, usize), seatlocs: &Vec<bool>) -> Vec<usize> {
    //Count amount of directions for which there is a seat with a person on it
    Direction::iter().map(|dir: Direction| {
        //Iterate over each factor
        (1..)
            //Map each factor to a position (i, j)
            .map(|f| dir.apply_to(pos.0, pos.1, dim.0, dim.1, f))
            //Take all the positions that are valid
            .take_while(|opt| opt.is_some()).map(|opt| opt.unwrap())
            //Take only positions that are a seat
            .filter(|&(ni, nj)| seatlocs[ni * dim.1 + nj])
            //Map to 1d
            .map(|pos| pos.0 * dim.1 + pos.1)
            //Take the first seat that is found
            .next()
    }).flatten().collect()
}

#[derive(EnumIter, Eq, PartialEq, Copy, Clone)]
enum Direction {
    MinMin,
    MinCen,
    MinMax,
    CenMin,
    CenMax,
    MaxMin,
    MaxCen,
    MaxMax,
}

impl Direction {
    /// Map each direction to (int, int) vector
    #[inline]
    fn get(&self) -> (isize, isize) {
        match self {
            Direction::MinMin => (-1, -1),
            Direction::MinCen => (-1, 0),
            Direction::MinMax => (-1, 1),
            Direction::CenMin => (0, -1),
            Direction::CenMax => (0, 1),
            Direction::MaxMin => (1, -1),
            Direction::MaxCen => (1, 0),
            Direction::MaxMax => (1, 1)
        }
    }

    /// Move (i, j) to this direction count times.
    #[inline]
    fn apply_to(&self, i: usize, j: usize, leni: usize, lenj: usize, count: usize) -> Option<(usize, usize)> {
        let mut dir = self.get();
        dir.0 *= count as isize;
        dir.1 *= count as isize;

        let (ni, nj) = (i as isize + dir.0, j as isize + dir.1);
        if !(0..leni as isize).contains(&ni) { return None; }
        if !(0..lenj as isize).contains(&nj) { return None; }
        Some((ni as usize, nj as usize))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(37, result);
    }

    #[test]
    pub(crate) fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(2275, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(26, result);
    }

    #[test]
    pub(crate) fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(2121, result);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            part1(input)
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            part2(input)
        });
    }
}
