use std::collections::HashSet;
use std::iter::FromIterator;

fn part1(inp: &str) -> Result<i64, ()> {
    let (nums, nums_set) = parse_input(inp);

    for num in nums{
        if nums_set.contains(&(2020 - num)) {
            return Ok(num * (2020 - num));
        }
    }

    return Err(())
}

fn part2(inp: &str) -> Result<i64, ()> {
    let (nums, nums_set) = parse_input(inp);

    for numa in nums.iter() {
        for numb in nums.iter() {
            if nums_set.contains(&(2020 - numa - numb)) {
                return Ok(numa * numb * (2020 - numa - numb));
            }
        }
    }

    return Err(())
}

fn parse_input(inp: &str) -> (Vec<i64>, HashSet<i64>) {
    let nums: Vec<i64> = inp.lines().map(|num| num.parse().unwrap()).collect();
    let nums_set: HashSet<i64> = HashSet::from_iter(nums.iter().cloned());
    return (nums, nums_set);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input")).unwrap();
        println!("Part 1: {}", result);
        assert_eq!(898299, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input")).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(143933922, result);
    }
}



