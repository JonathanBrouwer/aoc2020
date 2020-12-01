extern crate typenum;
use bit_array::BitArray;

fn part1(inp: &str) -> Result<usize, ()> {
    // Parse input
    let nums = parse_input(inp);

    //Create bitmap with numbers that exist
    let mut nums_bv = BitArray::<u32, typenum::U2048>::from_elem(false);
    nums.iter().for_each(|num| nums_bv.set(*num, true));

    // Loop through nums
    for num in nums {
        // Check if the other number exists, if so, return answer
        if nums_bv.get(2020 - num).unwrap() {
            return Ok(num * (2020 - num));
        }
    }

    return Err(());
}

fn part2(inp: &str) -> Result<usize, ()> {
    // Parse input
    let nums = parse_input(inp);

    //Create bitmap with numbers that exist
    let mut nums_bv = BitArray::<u32, typenum::U2048>::from_elem(false);
    nums.iter().for_each(|num| nums_bv.set(*num, true));

    // Loop through nums
    for (i, numa) in nums.iter().enumerate() {
        for numb in nums.iter().skip(i) {
            if numa + numb > 2020 { continue ; }

            // Check if the final number exists, if so, return answer
            if nums_bv.get(2020 - numa - numb).unwrap() {
                return Ok(numa * numb * (2020 - numa - numb));
            }
        }
    }

    return Err(());
}

fn parse_input(inp: &str) -> Vec<usize> {
    //Parse input into vec
    return inp.lines().map(|num| num.parse().unwrap()).collect();
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::time::SystemTime;

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
        let now = SystemTime::now();

        let result = part2(include_str!("input")).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(143933922, result);

        println!("Part 2 time: {}", now.elapsed().unwrap().as_micros());
    }
}



