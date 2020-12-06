extern crate test;

fn part1(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    //For each group
    return Ok(input.iter().map(|group| {
        //Map each person to a bitmap of which letters it consists of
        group.iter().map(|&person| {
            //Create bitmap
            let mut bitmap = [false; 26];
            person.chars().for_each(|c| bitmap[c as usize - 'a' as usize] = true);
            //Convert the bitmap to usize
            bitmap.iter().fold(0, |acc, b| acc * 2 + *b as usize)

            //Find intersection of bitmap, then count amount of flags set
        }).fold_first(|a, b| a | b).unwrap().count_ones() as usize
    }).sum::<usize>())
}

fn part2(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    //For each group
    return Ok(input.iter().map(|group| {
        //Map each person to a bitmap of which letters it consists of
        group.iter().map(|&person| {
            //Create bitmap
            let mut bitmap = [false; 26];
            person.chars().for_each(|c| bitmap[c as usize - 'a' as usize] = true);
            //Convert the bitmap to usize
            bitmap.iter().fold(0, |acc, b| acc * 2 + *b as usize)

            //Find intersection of bitmap, then count amount of flags set
        }).fold_first(|a, b| a & b).unwrap().count_ones() as usize
    }).sum::<usize>())
}

fn parse_input(inp: &str) -> Vec<Vec<&str>> {
    inp.split("\n\n").map(|group| {
        group.lines().collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example")).unwrap();
        assert_eq!(11, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input")).unwrap();
        println!("Part 1: {}", result);
        assert_eq!(7110, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example")).unwrap();
        assert_eq!(6, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input")).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(3628, result);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part1(input).unwrap();
            assert_eq!(7110, result);
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part2(input).unwrap();
            assert_eq!(3628, result);
        });
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        let input: &str = test::black_box(include_str!("input"));
        let correct_result = test::black_box(parse_input(input));
        b.iter(|| {
            let result = parse_input(input);
            assert_eq!(correct_result, result);
        });
    }
}



