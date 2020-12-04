use std::collections::HashMap;
use std::error::Error;

#[macro_use]
mod macro_check {
    macro_rules! check {
        ( $x:expr ) => {
            {
                if(!($x)) {
                    return false;
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
    return Ok(input.iter().filter(|&p| p.validate_part2()).count());
}

fn parse_input<'a>(inp: &'a str) -> Vec<Passport> {
    let mut passports = vec![Passport{ data: HashMap::new() }];

    for line in inp.lines() {
        if line.is_empty() {
            passports.push(Passport{ data: HashMap::new() });
        }
        else {
            for pair in line.split(" ") {
                let p: Vec<&'a str> = pair.split(":").collect();
                passports.last_mut().unwrap().data.insert(p[0], p[1]);
            }
        }
    }
    passports
}

struct Passport<'a> {
    data: HashMap<&'a str, &'a str>
}

impl Passport<'_> {

    fn validate_part1(&self) -> bool {
        let req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        return req_fields.iter().filter(|&f| self.data.contains_key(f)).count() == 7
    }

    fn validate_part2(&self) -> bool {
        check!(self.validate_part1());

        //BYR
        let byr = self.data.get("byr").unwrap().parse::<usize>().unwrap();
        check! (byr >= 1920 && byr <= 2002 );

        //IYR
        let iyr = self.data.get("iyr").unwrap().parse::<usize>().unwrap();
        check! (iyr >= 2010 && iyr <= 2020 );

        //EYR
        let eyr = self.data.get("eyr").unwrap().parse::<usize>().unwrap();
        check!( eyr >= 2020 && eyr <= 2030 );

        //HGT
        let hgt = self.data.get("hgt").unwrap();
        let hgt_num = hgt[..hgt.len() - 2].parse::<usize>();
        check!(hgt_num.is_ok());
        let hgt_num = hgt_num.unwrap();
        let hgt_unit = &hgt[hgt.len() - 2..];
        if hgt_unit == "cm" {
            check!(hgt_num >= 150 && hgt_num <= 193);
        }else if hgt_unit == "in" {
            check!(hgt_num >= 59 && hgt_num <= 76);
        }else{ return false; }

        //HCL
        let hcl = *self.data.get("hcl").unwrap();
        check!(hcl.chars().next().unwrap() == '#');
        check!(hcl.chars().count() == 7);
        check!(hcl.chars().skip(1).all(|f| f.is_numeric() || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&f)));

        //ECL
        let ecl  = *self.data.get("ecl").unwrap();
        check!(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl));

        //PID
        let pid = *self.data.get("pid").unwrap();
        check!(pid.len() == 9);
        check!(pid.chars().all(|c| c.is_numeric()));

        return true;
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



