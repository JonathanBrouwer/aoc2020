use crate::day11::main::Seat::{Floor, SeatEmpty, SeatFull};

fn part1(inp: &str) -> usize {
    let mut state = Map { vec: parse_input(inp) };

    loop {
        //Calc new counts
        let mut counts: Vec<Vec<usize>> = Vec::new();
        for _ in 0..state.vec.len() {
            counts.push(vec![0; state.vec[0].len()])
        }
        for i in 0..state.vec.len() {
            for j in 0..state.vec[0].len() {
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 { continue; }
                        counts[i][j] += match state.get(i as isize + di, j as isize + dj).unwrap_or(Floor) {
                            Floor | SeatEmpty => 0,
                            SeatFull => 1
                        }
                    }
                }
            }
        }

        //Calc new state
        let mut changed = false;
        for i in 0..state.vec.len() {
            for j in 0..state.vec[0].len() {
                state.vec[i][j] = match state.vec[i][j] {
                    Floor => Floor,
                    SeatEmpty => if counts[i][j] == 0 {
                        changed = true;
                        SeatFull
                    } else {
                        SeatEmpty
                    },
                    SeatFull => if counts[i][j] >= 4 {
                        changed = true;
                        SeatEmpty
                    } else {
                        SeatFull
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    return state.vec.iter().flatten().filter(|&&s| s == SeatFull).count();
}

fn part2(inp: &str) -> usize {
    let mut state = Map { vec: parse_input(inp) };

    loop {
        //Calc new counts
        let mut counts: Vec<Vec<usize>> = Vec::new();
        for _ in 0..state.vec.len() {
            counts.push(vec![0; state.vec[0].len()])
        }
        for i in 0..state.vec.len() {
            for j in 0..state.vec[0].len() {
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 { continue; }
                        let mut factor = 1;
                        counts[i][j] += loop {
                            let result = match state.get(i as isize + di*factor, j as isize + dj*factor) {
                                None => Some(0),
                                Some(Floor) => None,
                                Some(SeatEmpty) => Some(0),
                                Some(SeatFull) => Some(1)
                            };
                            if result.is_none() {
                                factor += 1;
                                continue;
                            }
                            break result.unwrap();
                        };

                    }
                }
            }
        }

        //Calc new state
        let mut changed = false;
        for i in 0..state.vec.len() {
            for j in 0..state.vec[0].len() {
                state.vec[i][j] = match state.vec[i][j] {
                    Floor => Floor,
                    SeatEmpty => if counts[i][j] == 0 {
                        changed = true;
                        SeatFull
                    } else {
                        SeatEmpty
                    },
                    SeatFull => if counts[i][j] >= 5 {
                        changed = true;
                        SeatEmpty
                    } else {
                        SeatFull
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

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

struct Map<T: Copy> {
    vec: Vec<Vec<T>>
}

impl<T: Copy> Map<T> {
    fn get(&self, i: isize, j: isize) -> Option<T> {
        if i < 0 || j < 0 || i >= self.vec.len() as isize || j >= self.vec[0].len() as isize { return None; }
        return Some(self.vec[i as usize][j as usize]);
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Seat {
    Floor,
    SeatEmpty,
    SeatFull,
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(0, result);
    }
}



