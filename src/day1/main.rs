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
    let mut nums = parse_input(inp);
    nums.sort_unstable();

    //Create bitmap with numbers that exist
    let mut nums_bitmap = [false; 2048];
    nums.iter().for_each(|num| nums_bitmap[*num] = true);

    // Loop through nums
    for (i, numa) in nums.iter().enumerate() {
        let inv_numa = 2020 - numa;
        for numb in nums.iter().skip(i + 1) {
            if *numb > inv_numa { continue; }

            // Check if the final number exists, if so, return answer
            let numc = inv_numa - numb;
            if nums_bitmap[numc] {
                return Ok(numa * numb * numc);
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
mod tests {
    use std::time::SystemTime;

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



