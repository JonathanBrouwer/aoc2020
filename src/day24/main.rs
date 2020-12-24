use crate::day24::main::Direction::{NE, E, W, SW, SE, NW};

const START_SIZE: usize = 150;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);

    let board = get_board(input);

    return board.iter().flatten().filter(|x| **x).count()
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    let mut board = get_board(input);

    for i in 0..100 {
        let mut counts = [[0u8; START_SIZE]; START_SIZE];
        for x in 1..START_SIZE-1 {
            for y in 1..START_SIZE-1 {
                counts[x][y] = Pos { x, y }.neighbours().iter().filter(|nb| board[nb.x][nb.y]).count() as u8;
            }
        }
        for x in 1..START_SIZE-1 {
            for y in 1..START_SIZE-1 {
                board[x][y] = match (board[x][y], counts[x][y]) {
                    (true, 1|2) => true,
                    (true, _) => false,
                    (false, 2) => true,
                    _ => false
                }
            }
        }
    }

    return board.iter().flatten().filter(|x| **x).count()
}

fn get_board(input: Vec<Vec<Direction>>) -> [[bool; START_SIZE]; START_SIZE] {
    let mut board = [[false; START_SIZE]; START_SIZE];
    let start = Pos{ x: START_SIZE/2, y: START_SIZE/2 };

    for moves in input {
        let finalpos = moves.iter().fold(start, |pos, dir| {
            pos.neighbour_to(*dir)
        });
        board[finalpos.x][finalpos.y] = !board[finalpos.x][finalpos.y];
    }

    board
}

fn parse_input(inp: &str) -> Vec<Vec<Direction>> {
    inp.lines().map(|line| {
        let mut cur = line;
        let mut result = Vec::new();
        while cur.len() != 0 {
            let (dir, incr) = match (cur.chars().next().unwrap(), cur.chars().nth(1)) {
                ('n', Some('e')) => (NE, 2),
                ('n', Some('w')) => (NW, 2),
                ('s', Some('e')) => (SE, 2),
                ('s', Some('w')) => (SW, 2),
                ('e', _) => (E, 1),
                ('w', _) => (W, 1),
                _ => unreachable!()
            };
            result.push(dir);
            cur = &cur[incr..];
        }
        result
    }).collect()

}

#[derive(Copy, Clone)]
enum Direction {
    NE, NW, SE, SW, E, W
}

#[derive(Copy, Clone)]
struct Pos {
    x: usize, y: usize
}

impl Pos {
    fn neighbour_to(&self, dir: Direction) -> Pos {
        let pos = match dir {
            NE => if self.y % 2 == 0 { (self.x, self.y + 1) } else { (self.x + 1, self.y + 1)  },
            NW => if self.y % 2 == 0 { (self.x - 1, self.y + 1) } else { (self.x, self.y + 1)  },
            SE => if self.y % 2 == 0 { (self.x, self.y - 1) } else { (self.x + 1, self.y - 1)  },
            SW => if self.y % 2 == 0 { (self.x - 1, self.y - 1) } else { (self.x, self.y - 1)  },
            E => (self.x + 1, self.y),
            W => (self.x - 1, self.y)
        };
        Pos { x: pos.0, y: pos.1 }
    }

    fn neighbours(&self) -> Vec<Pos> {
        let dirs = [NE, NW, SE, SW, E, W];
        dirs.iter().map(|dir| self.neighbour_to(*dir)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(10, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(497, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(2208, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(4156, result);
    }
}



