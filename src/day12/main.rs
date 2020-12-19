use crate::day12::main::Direction::{West, North, East, South};
use crate::day12::main::Instruction::{MoveDir, Turn, MoveForward};
use crate::day12::main::TurnDirection::{Left, Right};

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);

    let finalpos: (isize, isize) = input.iter().fold(((0, 0), East), |(pos, boatdir), &instr| {
        match instr {
            MoveDir(movedir, amount) => ((pos.0 + movedir.dir().0 * amount as isize, pos.1 + movedir.dir().1 * amount as isize), boatdir),
            MoveForward(amount) => ((pos.0 + boatdir.dir().0 * amount as isize, pos.1 + boatdir.dir().1 * amount as isize), boatdir),
            Turn(turndir, amount) => (pos, boatdir.turn(turndir, amount))
        }
    }).0;
    return (finalpos.0.abs() + finalpos.1.abs()) as usize;
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    let finalpos: (isize, isize) = input.iter().fold(((0, 0), (10, 1)), |(boatpos, waypoint), &instr| {
        match instr {
            MoveDir(movedir, amount) => {
                (
                    boatpos,
                    (waypoint.0 + movedir.dir().0 * amount as isize, waypoint.1 + movedir.dir().1 * amount as isize)
                )
            }
            MoveForward(amount) => {
                (
                    (boatpos.0 + waypoint.0 * amount as isize, boatpos.1 + waypoint.1 * amount as isize),
                    waypoint
                )
            }
            Turn(turndir, amount) => {
                (
                    boatpos,
                    match North.turn(turndir, amount) {
                        North => waypoint,
                        East => (waypoint.1, -waypoint.0),
                        South => (-waypoint.0, -waypoint.1),
                        West => (-waypoint.1, waypoint.0)
                    }
                )
            }
        }
    }).0;
    return (finalpos.0.abs() + finalpos.1.abs()) as usize;
}

fn parse_input(inp: &str) -> Vec<Instruction> {
    inp.lines().map(|line| {
        match line.chars().next().unwrap() {
            'N' => MoveDir(North, line[1..].parse().unwrap()),
            'E' => MoveDir(East, line[1..].parse().unwrap()),
            'S' => MoveDir(South, line[1..].parse().unwrap()),
            'W' => MoveDir(West, line[1..].parse().unwrap()),
            'L' => Turn(Left, line[1..].parse().unwrap()),
            'R' => Turn(Right, line[1..].parse().unwrap()),
            'F' => MoveForward(line[1..].parse().unwrap()),
            _ => unreachable!()
        }
    }).collect()
}

#[derive(Copy, Clone)]
enum Instruction {
    MoveDir(Direction, usize),
    MoveForward(usize),
    Turn(TurnDirection, usize),
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
enum TurnDirection {
    Left,
    Right,
}

impl Direction {
    fn dir(&self) -> (isize, isize) {
        match *self {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0)
        }
    }

    fn turn(&self, dir: TurnDirection, times: usize) -> Direction {
        let mut d = *self;
        for _ in 0..times / 90 {
            d = match dir {
                Left => d.left(),
                Right => d.right()
            }
        }
        d
    }
    fn left(self) -> Direction {
        match self {
            North => West,
            East => North,
            South => East,
            West => South
        }
    }

    fn right(self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(25, result);
    }

    #[test]
    pub(crate) fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(938, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(286, result);
    }

    #[test]
    pub(crate) fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(54404, result);
    }
}



