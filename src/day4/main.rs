use std::collections::HashMap;
use std::error::Error;
use std::iter::FromIterator;

#[macro_use]
mod macro_check {
    macro_rules! check {
        ( $x:expr ) => {
            {
                if(!($x)) {
                    return None;
                }
            }
        };
    }
}

fn part1(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    return Ok(input.iter().filter(|&p| p.validate_part1()).count());
}

fn part2(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    return Ok(input.iter().filter_map(|p| p.validate_part2()).count());
}

fn parse_input<'a>(inp: &'a str) -> Vec<Passport> {
    //For each passport
    inp.split("\n\n").into_iter().map(|pp| {
        //Create a new passport
        Passport {
            //Split passport string into key:value pairs
            data: HashMap::from_iter(pp.split(&['\n', ' '][..]).into_iter().map(|entry| {
                //For each pair, split on : and create pair for the hashmap
                let kv: Vec<&str> = entry.split(":").collect();
                (kv[0], kv[1])
            }))
        }
    }).collect()
}

struct Passport<'a> {
    data: HashMap<&'a str, &'a str>
}

impl Passport<'_> {
    fn validate_part1(&self) -> bool {
        let req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        return req_fields.iter().filter(|&f| self.data.contains_key(f)).count() == 7;
    }

    fn validate_part2(&self) -> Option<()> {
        check!(self.validate_part1());

        //BYR
        let byr = self.data.get("byr")?.parse::<usize>().ok()?;
        check!( (1920..=2002).contains(&byr) );

        //IYR
        let iyr = self.data.get("iyr")?.parse::<usize>().ok()?;
        check!( (2010..=2020).contains(&iyr) );

        //EYR
        let eyr = self.data.get("eyr")?.parse::<usize>().ok()?;
        check!( (2020..=2030).contains(&eyr) );

        //HGT
        let hgt = *self.data.get("hgt")?;
        let hgt_num = hgt[..hgt.len() - 2].parse::<usize>().ok()?;
        match hgt {
            //If height is in cm, check range
            hgt if hgt.strip_suffix("cm").is_some() => {
                check!((150..=193).contains(&hgt_num))
            }
            //If height is in in, check range
            hgt if hgt.strip_suffix("in").is_some() => {
                check!((59..=76).contains(&hgt_num))
            }
            _ => return None
        }

        //HCL
        let hcl = *self.data.get("hcl")?;
        check!(hcl.chars().next()? == '#');
        check!(hcl.chars().count() == 7);
        check!(hcl.chars().skip(1).all(|f| f.is_numeric() || ('a'..='f').contains(&f)));

        //ECL
        let ecl = *self.data.get("ecl")?;
        check!(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl));

        //PID
        let pid = *self.data.get("pid")?;
        check!(pid.len() == 9);
        check!(pid.chars().all(|c| c.is_numeric()));

        return Some(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example")).unwrap();
        assert_eq!(2, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input")).unwrap();
        println!("Part 1: {}", result);
        assert_eq!(264, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1")).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2")).unwrap();
        assert_eq!(3, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input")).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(224, result);
    }
}



