fn part1(input: usize) -> Result<usize, ()> {
    return Ok(solve_for_size(input));
}

fn part2(inputs: Vec<usize>) -> Result<usize, ()> {
    return Ok(inputs.iter().map(|&pop| {
        calc_perimeter(solve_for_size(pop))
    }).sum());
}

fn calc_perimeter(input: usize) -> usize {
    input * 8
}

fn solve_for_size(input: usize) -> usize {
    for i in 1.. {
        if area_of_regular_octagon(i) >= input {
            return i;
        }
    }
    panic!(); //Impossible
}

fn area_of_regular_octagon(side: usize) -> usize {
    return 9*side*side - 4 * area_of_triangle(side);
}

fn area_of_triangle(side: usize) -> usize {
    return (side * (side - 1)) / 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(1, solve_for_size(5));
        assert_eq!(2, solve_for_size(24));
        assert_eq!(3, solve_for_size(57));
        assert_eq!(4, solve_for_size(104));
        assert_eq!(10, solve_for_size(680));
        assert_eq!(25, solve_for_size(4325));
    }

    #[test]
    fn test_part1_real() {
        let result = part1(17485510).unwrap();
        println!("Part 1: {}", result);
        assert_eq!(1581, result);
    }

    #[test]
    fn test_part2_perimiter() {
        assert_eq!(8, calc_perimeter(1));
        assert_eq!(32, calc_perimeter(4));
        assert_eq!(200, calc_perimeter(25));
    }

    #[test]
    fn test_part2_full() {
        let inputs: Vec<usize> = vec![5, 104, 4325];
        let result = part2(inputs).unwrap();
        assert_eq!(240, result);
    }

    #[test]
    fn test_part2_real() {
        let inputs: Vec<usize> = vec![4541617527, 1340822918, 747693561, 430825388, 369024007, 42712846];
        let result = part2(inputs).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(0, result);
    }
}



