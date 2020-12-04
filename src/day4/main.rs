use std::collections::HashMap;
use std::error::Error;

fn part1(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    let req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    return Ok(input.iter().filter(|&p| {
        if req_fields.iter().filter(|&f| p.data.contains_key(f)).count() != 7 {return false;}
        return true;
    }).count())
    // return Err(())
}

fn part2(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    return Ok(input.iter().filter(|&p| p.validate()).count());
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
    fn validate(&self) -> bool {
        let req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        if req_fields.iter().filter(|&f| self.data.contains_key(f)).count() != 7 {return false;}

        let byr = self.data.get("byr").unwrap().parse::<usize>();
        if byr.is_err() { return false; }
        let byr = byr.unwrap();
        if byr < 1920 || byr > 2002 { return false; }

        let iyr = self.data.get("iyr").unwrap().parse::<usize>();
        if iyr.is_err() { return false; }
        let iyr = iyr.unwrap();
        if iyr < 2010 || iyr > 2020 { return false; }

        let eyr = self.data.get("eyr").unwrap().parse::<usize>();
        if eyr.is_err() { return false; }
        let eyr = eyr.unwrap();
        if eyr < 2020 || eyr > 2030 { return false; }

        let hgt = self.data.get("hgt").unwrap();
        let hgt_num = hgt[..hgt.len() - 2].parse::<usize>();
        if hgt_num.is_err() { return false; }
        let hgt_num = hgt_num.unwrap();
        let hgt_unit = &hgt[hgt.len() - 2..];
        if hgt_unit == "cm" {
            if hgt_num < 150 || hgt_num > 193 { return false; }
        }else if hgt_unit == "in" {
            if hgt_num < 59 || hgt_num > 76 { return false; }
        }else{ return false; }

        let hcl = *self.data.get("hcl").unwrap();
        if hcl.chars().next().unwrap() != '#' { return false; }
        if hcl.chars().count() != 7 {return false;}
        if !hcl.chars().skip(1).all(|f| f.is_numeric() || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&f)) { return false; }

        let ecl  = *self.data.get("ecl").unwrap();
        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl) {return false;}

        let pid = *self.data.get("pid").unwrap();
        if pid.len() != 9 { return false; }
        if !pid.chars().all(|c| c.is_numeric()) { return false; }

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



