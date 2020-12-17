use itertools::Itertools;
use std::mem;
use std::ops::{Index, IndexMut};

const max_pos: usize = 14;
const max_neg: usize = 7;

fn part1(inp: &str) -> usize {
    let mut state = Grid3D::new();
    parse_input(inp, &mut state);

    let mut new_state = Grid3D::new();
    for _ in 0..6 {
        next(&state, &mut new_state);
        mem::swap(&mut state, &mut new_state)
    }
    state.vec.iter().filter(|&&b| b).count()
}

fn part2(inp: &str) -> usize {
    // let input = parse_input(inp);

    return 0;
    // return Err(())
}

fn next(state: &Grid3D, new_state: &mut Grid3D) {
    for ((x,y),z) in (-(max_neg as isize)+1..=max_pos as isize-1).cartesian_product(-(max_neg as isize)+1..=max_pos as isize-1).cartesian_product(-(max_neg as isize)+1..=max_pos as isize-1) {
        let mut count = 0;
        for ((dx, dy), dz) in (-1isize..=1).cartesian_product(-1isize..=1).cartesian_product(-1isize..=1) {
            if dx == 0 && dy == 0 && dz == 0 {continue}
            if state[(x+dx, y+dy, z+dz)] {
                count += 1;
            }
        }
        new_state[(x,y,z)] = match (state[(x,y,z)], count) {
            (true, 2|3) => true,
            (false, 3) => true,
            _ => false
        }

    }
}

fn parse_input(inp: &str, state: &mut Grid3D) -> () {
    inp.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            state[(x as isize, y as isize, 0)] = c == '#'
        })
    })
}

struct Grid3D {
    vec: Vec<bool>
}

impl Grid3D {
    fn new() -> Self {
        Grid3D { vec: vec![false; (max_pos + max_neg + 1).pow(3) ]}
    }
}

impl Index<(isize, isize, isize)> for Grid3D {
    type Output = bool;

    #[inline]
    fn index(&self, index: (isize, isize, isize)) -> &bool {
        let index_abs = (
            (index.0 + max_neg as isize) as usize,
            (index.1 + max_neg as isize) as usize,
            (index.2 + max_neg as isize) as usize
            );
        let final_index = index_abs.0*(max_pos+max_neg).pow(2) + index_abs.1*(max_pos+max_neg) + index_abs.2;
        &self.vec[final_index]
    }
}

impl IndexMut<(isize, isize, isize)> for Grid3D {
    #[inline]
    fn index_mut(&mut self, index: (isize, isize, isize)) -> &mut bool {
        let index_abs = (
            (index.0 + max_neg as isize) as usize,
            (index.1 + max_neg as isize) as usize,
            (index.2 + max_neg as isize) as usize
        );
        let final_index = index_abs.0*(max_pos+max_neg).pow(2) + index_abs.1*(max_pos+max_neg) + index_abs.2;
        &mut self.vec[final_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(112, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(289, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(848, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(0, result);
    }
}



