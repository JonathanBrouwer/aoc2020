use std::collections::{VecDeque, HashSet};

fn part1(inp: &str) -> usize {
    let (mut p1, mut p2) = parse_input(inp);

    //Play the game
    while !p1.is_empty() && !p2.is_empty() {
        let card1 = p1.pop_front().unwrap();
        let card2 = p2.pop_front().unwrap();
        if card1 > card2 {
            p1.push_back(card1);
            p1.push_back(card2);
        } else {
            p2.push_back(card2);
            p2.push_back(card1);
        }
    }

    //Count score
    let winningdeck = if p1.is_empty() {p2} else {p1};
    winningdeck.iter().enumerate().map(|(i, num)| (winningdeck.len()-i)*num).sum()
}

fn part2(inp: &str) -> usize {
    let (mut p1, mut p2) = parse_input(inp);

    //Play the game
    play_rec_game(&mut p1, &mut p2);

    //Count score
    let winningdeck = if p1.is_empty() {p2} else {p1};
    winningdeck.iter().enumerate().map(|(i, num)| (winningdeck.len()-i)*num).sum()
}

///True if p1 wins, False if p2 wins
fn play_rec_game(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> bool {
    let mut prev_games: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if prev_games.contains(&(p1.clone(), p2.clone())) {
            return true;
        }
        prev_games.insert((p1.clone(), p2.clone()));

        let card1 = p1.pop_front().unwrap();
        let card2 = p2.pop_front().unwrap();
        if if p1.len() >= card1 && p2.len() >= card2 {
            let mut p1_sub: VecDeque<usize> = p1.iter().take(card1).map(|x| *x).collect();
            let mut p2_sub: VecDeque<usize> = p2.iter().take(card2).map(|x| *x).collect();
            play_rec_game(&mut p1_sub, &mut p2_sub)
        } else {
            card1 > card2
        } {
            p1.push_back(card1);
            p1.push_back(card2);
        }else {
            p2.push_back(card2);
            p2.push_back(card1);
        }
    }

    return p2.is_empty();
}

fn parse_input(inp: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut dqs: Vec<VecDeque<usize>> = inp.split("\n\n").map(|player| {
        player.lines().skip(1).map(|line| line.parse().unwrap()).collect()
    }).collect();
    (dqs.remove(0), dqs.remove(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(306, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(33010, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(291, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example_rec"));
        assert_eq!(105, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(32769, result);
    }
}



