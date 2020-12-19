/// part2 = {SOI ~ (expr ~ "\n")* ~ EOI}
fn part2(inp: &str) -> usize {
    inp.lines().map(|line| {
        let line_without_space = &line.replace(" ", "");
        let (inp, res) = expr_muls(line_without_space).unwrap();
        assert_eq!(inp.len(), 0);
        res
    }).sum()
}

///expr_muls = { expr_adds ~ ("*" ~ expr_adds)* }
fn expr_muls(inp: &str) -> Result<(&str, usize), ()> {
    let (mut inp, mut res) = expr_adds(inp)?;
    loop {
        let op = mul(inp);
        if op.is_err() { return Ok((inp, res)) }
        let new_inp = op?;
        let (new_inp, new_val) = expr_adds(new_inp)?;
        inp = new_inp;
        res *= new_val;
    }
}

///expr_adds = { term ~ ("+" ~ term)*
fn expr_adds(inp: &str) -> Result<(&str, usize), ()> {
    let (mut inp, mut res) = term(inp)?;
    loop {
        let op = add(inp);
        if op.is_err() { return Ok((inp, res)) }
        let new_inp = op?;
        let (new_inp, new_val) = term(new_inp)?;
        inp = new_inp;
        res += new_val;
    }
}

///add = {"+"}
fn add(inp: &str) -> Result<&str, ()> {
    if inp.chars().next().ok_or(())? == '+' {
        Ok(&inp[1..])
    } else {
        Err(())
    }
}

///mul = {"*"}
fn mul(inp: &str) -> Result<&str, ()> {
    if inp.chars().next().ok_or(())? == '*' {
        Ok(&inp[1..])
    } else {
        Err(())
    }
}

///term = {num | "(" ~ expr_muls ~ ")"}
///    num = { ASCII_DIGIT+ }
fn term(inp: &str) -> Result<(&str, usize), ()> {
    if inp.starts_with("(") {
        let (inp, res) = expr_muls(&inp[1..])?;
        Ok((&inp[1..], res))
    } else if inp.chars().next().ok_or(())?.is_digit(10) {
        Ok((&inp[1..], inp.chars().next().ok_or(())?.to_digit(10).ok_or(())? as usize))
    } else {
        Err(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn test_part2_ex1() {
        assert_eq!(part2("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
    }

    #[test]
    pub(crate) fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(185348874183674, result);
    }
}



