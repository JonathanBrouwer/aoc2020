extern crate test;
extern crate strum;
extern crate strum_macros;

use crate::day11::main::Seat::{Floor, SeatEmpty, SeatFull};
use itertools::iproduct;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::mem::swap;
use std::ops::{Index, IndexMut};

fn part1(inp: &str) -> usize {
    solve_generic(inp, Vec2D::calc_neighbours_p1, 4)
}

fn part2(inp: &str) -> usize {
    solve_generic(inp, Vec2D::calc_neighbours_p2, 5)
}

#[inline]
fn solve_generic(inp: &str, calc_neighbours: fn(&Vec2D<Seat>, usize, usize) -> Vec<(usize, usize)>, seatfull_swap_min: usize) -> usize {
    let mut state = parse_input(inp);
    let mut old_state = state.clone();

    //Calc neighbours for each state
    let mut neighbours = Vec2D::<Vec<(usize, usize)>> { vec: vec![Vec::new(); state.dim.0 * state.dim.1], dim: state.dim.clone()}; //;
    for (i, j) in iproduct!(0..state.dim.0, 0..state.dim.1) {
        neighbours[(i, j)] = calc_neighbours(&state, i, j);
    }

    //Calc seat positions
    let seat_positions: Vec<_> = iproduct!(0..state.dim.0, 0..state.dim.1).filter(|&(i, j)| state[(i, j)] != Floor).collect();

    loop {
        //Keep track of old state and changed
        let mut changed = false;

        //For each position (i, j)
        for &(i, j) in &seat_positions {

            //Match on old state and the amount of neighbours, return new state
            state[(i, j)] = match (old_state[(i, j)], old_state.count_neighbours(&neighbours[(i, j)])) {
                (Floor, _) => unreachable!(),
                (SeatEmpty, 0) => SeatFull,
                (SeatEmpty, _) => SeatEmpty,
                (SeatFull, v) if (seatfull_swap_min..=8).contains(&v)  => SeatEmpty,
                (SeatFull, _) => SeatFull,
            };
            //Store if state changed
            changed |= state[(i, j)] != old_state[(i, j)];
        }

        //If nothing changed, break
        if !changed { break; }

        //Swap
        swap(&mut state, &mut old_state)
    }

    //Count amount of full seats
    return state.vec.iter().filter(|&&s| s == SeatFull).count();
}

#[inline]
fn parse_input(inp: &str) -> Vec2D<Seat> {
    let dim = (inp.lines().count(), inp.lines().next().unwrap().len());
    let vec = inp.chars().filter(|&c| c != '\n').map(|c| match c {
        '.' => Floor,
        'L' => SeatFull,
        _ => unreachable!()
    }).collect();
    return Vec2D { vec, dim }
}

#[derive(Clone)]
struct Vec2D<T> {
    vec: Vec<T>,
    dim: (usize, usize)
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.vec[index.0*self.dim.1+index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {

    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.vec[index.0*self.dim.1+index.1]
    }
}

impl Vec2D<Seat> {
    #[inline]
    fn calc_neighbours_p1(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        //Count amount of directions for which there is a seat with a person on it
        Direction::iter().map(|dir: Direction| {
            //Find the position in this direction
            dir.apply_to(i, j, self.dim.0, self.dim.1, 1)
                //Take only positions that are a seat
                .filter(|&(ni, nj)| self[(ni,nj)] != Floor)
        }).flatten().collect()
    }

    #[inline]
    fn calc_neighbours_p2(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        //Count amount of directions for which there is a seat with a person on it
        Direction::iter().map(|dir: Direction| {
            //Iterate over each factor
            (1..)
                //Map each factor to a position (i, j)
                .map(|f| dir.apply_to(i, j, self.dim.0, self.dim.1, f))
                //Take all the positions that are valid
                .take_while(|opt| opt.is_some()).map(|opt| opt.unwrap())
                //Take only positions that are a seat
                .filter(|&(ni, nj)| self[(ni,nj)] != Floor)
                //Take the first seat that is found
                .next()
        }).flatten().collect()
    }

    #[inline]
    fn count_neighbours(&self, nbs: &Vec<(usize, usize)>) -> usize {
        //Iterate over each factor
        nbs.iter()
            //Check if there's anyone on the seat
            .filter(|&&(ni, nj)| self[(ni,nj)] == SeatFull)
            //Count amount of seats
            .count()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Seat {
    Floor,
    SeatEmpty,
    SeatFull,
}

#[derive(EnumIter, Eq, PartialEq, Copy, Clone)]
enum Direction {
    MinMin, MinCen, MinMax, CenMin, CenMax, MaxMin, MaxCen, MaxMax
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
        dir.0 *= count as isize; dir.1 *= count as isize;

        let (ni, nj) = (i as isize + dir.0, j as isize + dir.1);
        if !(0..leni as isize).contains(&ni) { return None }
        if !(0..lenj as isize).contains(&nj) { return None }
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



