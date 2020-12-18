use crate::day18::main_part1::Operation::{ADD, MUL};

/// part1 = {SOI ~ (expr ~ "\n")* ~ EOI}
fn part1(inp: &str) -> usize {
    inp.lines().map(|line| {
        let line_without_space = &line.replace(" ", "");
        let (inp, res) = expr(line_without_space).unwrap();
        assert_eq!(inp.len(), 0); // Make sure entire input is parsed
        res
    }).sum()
}

///expr = { term ~ (operation ~ term)* }
fn expr(inp: &str) -> Result<(&str, usize), ()> {
    let (mut inp, mut res) = term(inp)?;
    loop {
        let op = operation(inp);
        if op.is_err() { return Ok((inp, res)) }
        let (new_inp, op) = op?;
        let (new_inp, new_val) = term(new_inp)?;
        inp = new_inp;
        res = match op {
            ADD => res + new_val,
            MUL => res * new_val
        }
    }
}

///operation = { add | mul }
///    add = {"+"}
///    mul = {"*"}
fn operation(inp: &str) -> Result<(&str, Operation), ()> {
    if inp.chars().next().ok_or(())? == '+' {
        Ok((&inp[1..], ADD))
    } else if inp.chars().next().ok_or(())? == '*' {
       Ok((&inp[1..], MUL))
    } else {
        Err(())
    }
}
enum Operation { ADD, MUL }

///term = {num | "(" ~ expr ~ ")"}
///    num = { ASCII_DIGIT+ }
fn term(inp: &str) -> Result<(&str, usize), ()> {
    if inp.starts_with("(") {
        let (inp, res) = expr(&inp[1..])?;
        Ok((&inp[1..], res))
    } else if inp.chars().next().ok_or(())?.is_digit(10) {
        Ok((&inp[1..], inp.chars().next().ok_or(())?.to_digit(10).ok_or(())? as usize))
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(part1("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(part1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(11297104473091, result);
    }
}



