use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);

    let mut edgemap = HashMap::new();
    for tile in &input {
        for edge in tile.possible_edges().0 {
            let mut list = edgemap.remove(&edge);
            if list.is_none() { list = Some(Vec::new()); }
            let mut list = list.unwrap();
            list.push(tile);
            edgemap.insert(edge, list);
        }
        for edge in tile.possible_edges().1 {
            let mut list = edgemap.remove(&edge);
            if list.is_none() { list = Some(Vec::new()); }
            let mut list = list.unwrap();
            list.push(tile);
            edgemap.insert(edge, list);
        }
    }

    //Create board
    let mut board = Board::new(&edgemap);

    //Find a corner
    let corner = input.iter().find(|tile|
        tile.possible_edges().0.iter().map(|e| edgemap.get(e).unwrap().len()).sum::<usize>() == 6
    ).unwrap();
    let corner_edges = &corner.possible_edges().0;
    let corner_actual_edges: Vec<_> = corner_edges.iter().filter(|e| edgemap.get(*e).unwrap().len() == 2).collect();
    board.board[0][0] = Some(BoardTile{tile: corner, right_edge: *corner_actual_edges[0], bottom_edge: *corner_actual_edges[1]});

    //Find rest of the top row
    for x in 1..10 {
        let prev: BoardTile = board.board[x-1][0].unwrap();
        let next = *edgemap.get(&prev.right_edge).unwrap().iter().find(|tile| tile.id != prev.tile.id).unwrap();
        board.lock_in_from_left((x, 0), next, prev.right_edge);
    }


    println!("{:?}", board.board[0][0]);



    // let mut counts = [0; 100];
    // for val in edgemap.values() {
    //     counts[*val] += 1;
    // }

    // input.iter().filter_map(|tile| {
    //     let totalcount: usize = tile.possible_edges().iter()
    //         .map(|e| edgemap.get(e).unwrap())
    //         .sum();
    //     if totalcount == 12 {
    //         Some(tile.id)
    //     }else{
    //         None
    //     }
    // }).product()
    0
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    return 0;
    // return Err(())
}

fn parse_input(inp: &str) -> Vec<Tile> {
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
    edgemap: &'a HashMap<[bool; 10], Vec<&'a Tile>>,
    //Tile, right edge, bottom edge
    board: [[Option<BoardTile<'a>>; 10]; 10]
}

#[derive(Debug, Copy, Clone)]
struct BoardTile<'a> {
    tile: &'a Tile,
    right_edge: [bool; 10],
    bottom_edge: [bool; 10]
}

impl<'a> Board<'a> {
    fn new(edgemap: &'a HashMap<[bool; 10], Vec<&'a Tile>>) -> Self {
        Board{ edgemap, board: [[None; 10]; 10] }
    }
    fn lock_in_from_left(&mut self, pos: (usize, usize), tile: &'a Tile, left_edge: [bool; 10]) {
        let (elist1, elist2) = tile.possible_edges();

        let edge_list = if elist1.contains(&left_edge) { &elist1 } else { &elist2};
        let right_edge = edge_list[(edge_list.iter().position(|e| *e == left_edge).unwrap() + 2)%4];
        let bottom_edge = edge_list[(edge_list.iter().position(|e| *e == left_edge).unwrap() + 3)%4];

        self.board[pos.0][pos.1] = Some(BoardTile {tile, right_edge, bottom_edge})
    }
}

#[derive(Debug)]
struct Tile {
    id: usize,
    grid: [[bool; 10]; 10]
}

impl Tile {
    fn possible_edges(&self) -> (Vec<[bool; 10]>, Vec<[bool; 10]>) {
        let mut edges = Vec::new();
        let mut edges_alt = Vec::new();
        //Top
        edges.push(self.to_array(|i| self.grid[i][0]));
        edges_alt.push(self.to_array(|i| self.grid[9-i][0]));
        //Right
        edges.push(self.to_array(|i| self.grid[9][i]));
        edges_alt.push(self.to_array(|i| self.grid[9][9-i]));
        //Bottom
        edges_alt.push(self.to_array(|i| self.grid[i][9]));
        edges.push(self.to_array(|i| self.grid[9-i][9]));
        //Left
        edges_alt.push(self.to_array(|i| self.grid[0][i]));
        edges.push(self.to_array(|i| self.grid[0][9-i]));

        (edges, edges_alt)
    }

    fn to_array<F>(&self, mapfun: F) -> [bool; 10] where
        F: Fn(usize) -> bool {
        let mut output = [false; 10];
        for i in 0..10 {
            output[i] = mapfun(i);
        }
        output
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
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(0, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(0, result);
    }
}



