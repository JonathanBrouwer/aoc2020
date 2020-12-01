use std::collections::HashSet;
use std::iter::FromIterator;

fn part1(inp: &str) -> Result<i64, ()> {
    // Parse input
    let (nums, nums_set) = parse_input(inp);

    // Loop through nums
    for num in nums {
        // Check if the other number exists, if so, return answer
        if nums_set.contains(&(2020 - num)) {
            return Ok(num * (2020 - num));
        }
    }

    return Err(());
}

fn part2(inp: &str) -> Result<i64, ()> {
    // Parse input
    let (nums, nums_set) = parse_input(inp);

    // Loop through nums
    for numa in nums.iter() {
        for numb in nums.iter() {
            // Check if the final number exists, if so, return answer
            if nums_set.contains(&(2020 - numa - numb)) {
                return Ok(numa * numb * (2020 - numa - numb));
            }
        }
    }

    return Err(());
}

fn parse_input(inp: &str) -> (Vec<i64>, HashSet<i64>) {
    //Parse input into vec
    let nums: Vec<i64> = inp.lines().map(|num| num.parse().unwrap()).collect();
    //Parse vec into hashset
    let nums_set: HashSet<i64> = HashSet::from_iter(nums.iter().cloned());
    return (nums, nums_set);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example")).unwrap();
        assert_eq!(514579, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input")).unwrap();
        println!("Part 1: {}", result);
        assert_eq!(898299, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example")).unwrap();
        assert_eq!(241861950, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input")).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(143933922, result);
    }
}



