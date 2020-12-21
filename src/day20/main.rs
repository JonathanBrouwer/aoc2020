use std::collections::HashMap;
use integer_sqrt::IntegerSquareRoot;
use std::fmt::{Debug, Formatter};

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    let board = find_board(&input);
    board.print();

    let edgelength = input.len().integer_sqrt();
    let id1 = board.board[0][0].unwrap().0.id;
    let id2 = board.board[edgelength-1][0].unwrap().0.id;
    let id3 = board.board[0][edgelength-1].unwrap().0.id;
    let id4 = board.board[edgelength-1][edgelength-1].unwrap().0.id;
    id1*id2*id3*id4
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);
    let board = find_board(&input);

    let edgelength = input.len().integer_sqrt();
    let mut finalboard = [[false; 8*12]; 8*12];
    for y in 0..edgelength {
        for x in 0..edgelength {
            for dy in 0..8 {
                for dx in 0..8 {
                    finalboard[8*x+dx][8*y+dy] = board.board[x][y].unwrap().1[dx+1][dy+1];
                }
            }
        }
    }

    //Pattern[y][x]
    let pattern: Vec<Vec<_>> = include_str!("pattern.txt").lines().map(|line| {
        line.chars().map(|c| c == '#').collect()
    }).collect();

    let finaltile = Tile::<96> { id: 0, grid: finalboard };
    let maxcount = finaltile.orientations().iter().map(|orientation| {
        let mut count = 0;
        for y in 0..(8*edgelength-pattern.len()+1) {
            'a: for x in 0..(8*edgelength-pattern[0].len()+1) {
                for dy in 0..pattern.len() {
                    for dx in 0..pattern[0].len() {
                        if pattern[dy][dx] && !orientation[x+dx][y+dy] { continue 'a; }
                    }
                }
                count += 1;
            }
        }
        println!("{}", count);
        count
    }).max().unwrap();
    finalboard.iter().flatten().filter(|x| **x).count() - pattern.iter().flatten().filter(|x| **x).count()*maxcount
}

fn find_board(input: &Vec<Tile<10>>) -> Board {
    let edgelength = input.len().integer_sqrt();

    let mut neighbours = HashMap::new();
    for tile in input {
        neighbours.insert(tile, Vec::new());
    }

    for tile1 in input {
        for tile2 in input {
            if tile1.id == tile2.id { continue; }
            'outer: for orient1 in tile1.orientations() {
                for orient2 in tile2.orientations() {
                    if orient1[0] == orient2[0] {
                        let nbs = neighbours.get_mut(tile1).unwrap();
                        nbs.push(tile2);
                        break 'outer;
                    }
                }
            }
        }
    }

    //Create board
    let mut board = Board { board: [[None; 12]; 12] };

    //Find top-left corner
    let corner = *neighbours.iter().find(|(k,v)| v.len() == 2).unwrap().0;
    let corner_config = *corner.orientations().iter().find(|orientation| {
        let nbs = neighbours.get(corner).unwrap();
        nbs[0].orientations().iter().any(|nborientation| {
            orientation[9] == nborientation[0]
        }) && nbs[1].orientations().iter().any(|nborientation| {
            (0..10).all(|x| orientation[x][9] == nborientation[x][0])
        })
    }).unwrap();
    board.board[0][0] = Some((corner, corner_config));

    //Find top row
    for x in 1..edgelength {
        let prev = board.board[x-1][0].unwrap();
        for nb in neighbours.get(prev.0).unwrap() {
            for nb_orient in nb.orientations() {
                if prev.1[9] == nb_orient[0] {
                    board.board[x][0] = Some((nb, nb_orient));
                }
            }
        }
    }

    //Find other rows
    for y in 1..edgelength {
        for x in 0..edgelength {
            let prev = board.board[x][y-1].unwrap();
            for nb in neighbours.get(prev.0).unwrap() {
                for nb_orient in nb.orientations() {
                    if (0..10).all(|dx| prev.1[dx][9] == nb_orient[dx][0]) {
                        board.board[x][y] = Some((nb, nb_orient));
                    }
                }
            }
        }
    }

    board
}

fn parse_input(inp: &str) -> Vec<Tile<10>> {
    inp.split("\n\n").map(|tile| {
        let id: usize = tile.lines().next().unwrap()
            .split(" ").nth(1).unwrap()
            .split(":").nth(0).unwrap()
            .parse().unwrap();
        let mut grid = [[false; 10]; 10];
        tile.lines().skip(1).enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                grid[x][y] = c == '#';
            });
        });
        Tile {id, grid}
    }).collect()
}

#[derive(Debug)]
struct Board<'a> {
    board: [[Option<(&'a Tile<10>, [[bool; 10]; 10])>; 12]; 12]
}

impl<'a> Board<'a> {
    fn print(&self) {
        for y in 0..12 {
            for x in 0..12 {
                if self.board[x][y].is_none() { continue }
                print!("{} ", self.board[x][y].unwrap().0.id);
            }
            println!();
        }
        println!();
        for y in 0..12 {
            for dy in 0..10 {
                for x in 0..12 {
                    for dx in 0..10 {
                        if self.board[x][y].is_none() { continue }
                        let val: bool = self.board[x][y].unwrap().1[dx][dy];
                        if val {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    print!(" ");
                }
                println!();
            }
            println!();
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Tile<const LEN: usize> {
    id: usize,
    grid: [[bool; LEN]; LEN]
}

impl<const LEN: usize> Tile<LEN> {
    fn orientations(&self) -> Vec<[[bool; LEN]; LEN]> {
        let mut result = Vec::new();
        let mut state = self.grid.clone(); result.push(state.clone());
        Tile::rot90(&mut state);  result.push(state.clone());
        Tile::rot90(&mut state);  result.push(state.clone());
        Tile::rot90(&mut state);  result.push(state.clone());
        Tile::transpose(&mut state); result.push(state.clone());
        Tile::rot90(&mut state);  result.push(state.clone());
        Tile::rot90(&mut state);  result.push(state.clone());
        Tile::rot90(&mut state);  result.push(state.clone());

        result
    }

    fn rot90(state: &mut [[bool; LEN]; LEN]) {
        //Transpose
        Tile::transpose(state);

        //Reverse each row
        for y in 0..LEN {
            for x in 0..LEN/2 {
                let temp = state[x][y];
                state[x][y] = state[LEN-x-1][y];
                state[LEN-x-1][y] = temp;
            }
        }
    }

    fn transpose(state: &mut [[bool; LEN]; LEN]) {
        //Transpose
        for y in 0..LEN {
            for x in 0..y {
                let temp = state[x][y];
                state[x][y] = state[y][x];
                state[y][x] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(20899048083289, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(84116744709593, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1957, result);

        //<9036 (>12)

        //<8721 (>33)
    }
}



