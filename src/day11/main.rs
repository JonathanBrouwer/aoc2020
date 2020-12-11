extern crate test;
extern crate strum;
extern crate strum_macros;

use crate::day11::main::Seat::{Floor, SeatEmpty, SeatFull};
use itertools::iproduct;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::mem::swap;

fn part1(inp: &str) -> usize {
    let mut state = Map { vec: parse_input(inp) };
    let mut old_state = state.clone();

    loop {
        // Keep track of whether the state changed
        let mut changed = false;

        //For each position (i, j)
        for (i, j) in iproduct!(0..state.vec.len(), 0..state.vec[0].len()) {
            //Match on old state and the amount of neighbours, return new state
            state.vec[i][j] = match (old_state.vec[i][j], old_state.count_neighbours_p1(i, j)) {
                (Floor, _) => Floor,
                (SeatEmpty, 0) => SeatFull,
                (SeatEmpty, _) => SeatEmpty,
                (SeatFull, 4..=8) => SeatEmpty,
                (SeatFull, _) => SeatFull,
            };
            //Store if state changed
            changed |= state.vec[i][j] != old_state.vec[i][j];
        }

        //If nothing changed, break
        if !changed { break; }

        swap(&mut state, &mut old_state)
    }

    //Count amount of full seats
    return state.vec.iter().flatten().filter(|&&s| s == SeatFull).count();
}

fn part2(inp: &str) -> usize {
    let mut state = Map { vec: parse_input(inp) };

    loop {
        //Keep track of old state and changed
        let old_state = state.clone();
        let mut changed = false;

        //For each position (i, j)
        for (i, j) in iproduct!(0..state.vec.len(), 0..state.vec[0].len()) {
            //Match on old state and the amount of neighbours, return new state
            state.vec[i][j] = match (old_state.vec[i][j], old_state.count_neighbours_p2(i, j)) {
                (Floor, _) => Floor,
                (SeatEmpty, 0) => SeatFull,
                (SeatEmpty, _) => SeatEmpty,
                (SeatFull, 5..=8) => SeatEmpty,
                (SeatFull, _) => SeatFull,
            };
            //Store if state changed
            changed |= state.vec[i][j] != old_state.vec[i][j];
        }

        //If nothing changed, break
        if !changed { break; }
    }

    //Count amount of full seats
    return state.vec.iter().flatten().filter(|&&s| s == SeatFull).count();
}

fn parse_input(inp: &str) -> Vec<Vec<Seat>> {
    inp.lines().map(|line| {
        line.chars().map(|c| match c {
            '.' => Floor,
            'L' => SeatEmpty,
            _ => unreachable!()
        }).collect()
    }).collect()
}

#[derive(Clone)]
struct Map {
    vec: Vec<Vec<Seat>>
}

impl Map {
    #[inline(always)]
    fn count_neighbours_p1(&self, i: usize, j: usize) -> usize {
        //Count amount of directions for which there is a seat with a person on it
        Direction::iter().filter(|dir: &Direction| {
            //Find the position in this direction
            dir.apply_to(i, j, self.vec.len(), self.vec[0].len(), 1)
                //Check if there's anyone on the seat
                .map(|(ni, nj)| self.vec[ni][nj] == SeatFull)
                //If there was no seat, return false
                .unwrap_or(false)
        }).count()
    }

    #[inline(always)]
    fn count_neighbours_p2(&self, i: usize, j: usize) -> usize {
        //Count amount of directions for which there is a seat with a person on it
        Direction::iter().filter(|dir: &Direction| {
            //Iterate over each factor
            (1..)
                //Map each factor to a position (i, j)
                .map(|f| dir.apply_to(i, j, self.vec.len(), self.vec[0].len(), f))
                //Take all the positions that are valid
                .take_while(|opt| opt.is_some()).map(|opt| opt.unwrap())
                //Take only positions that are a seat
                .filter(|&(ni, nj)| self.vec[ni][nj] != Floor)
                //Check if there's anyone on the seat
                .map(|(ni, nj)| self.vec[ni][nj] == SeatFull)
                //Take the first seat that is found
                .next().unwrap_or(false)
        }).count()
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
    #[inline(always)]
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

    #[inline(always)]
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
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(37, result);
    }

    #[test]
    fn test_part1_real() {
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
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(2121, result);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part1(input);
            assert_eq!(2275, result);
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part2(input);
            assert_eq!(2121, result);
        });
    }
}



