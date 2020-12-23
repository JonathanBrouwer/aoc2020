use itertools::Itertools;

fn part1(inp: &str) -> String {
    let state = parse_input(inp);
    let mut cll = solve(state, 100, 9);

    (0..8).map(|_| cll.remove_next(1)).map(|n| (n as u8 + '0' as u8) as char).join("")
}

fn part2(inp: &str) -> usize {
    let mut state = parse_input(inp);
    for i in 10..=1000000 {
        state.push(i);
    }

    let cll = solve(state, 10000000, 1000000);

    let a = cll.next(1);
    let b = cll.next(a);
    a as usize * b as usize
}

fn solve(state: Vec<u32>, iterations: usize, max_num: u32) -> FakeCLL {
    //Make into cll
    let mut cur_value = state[0];
    let mut cll = FakeCLL::from_vec(state);

    for _ in 0..iterations {
        //Remove next 3 elements
        let removed_vals: Vec<_> = (0..3).map(|_| cll.remove_next(cur_value)).collect();

        //Find destination
        let mut dest_val = cur_value - 1;
        if dest_val == 0 { dest_val = max_num; }
        while removed_vals.contains(&dest_val) {
            dest_val = dest_val - 1;
            if dest_val == 0 { dest_val = max_num; }
        }

        //Insert after destination
        cll.add_after(dest_val, removed_vals[0]);
        cll.add_after(removed_vals[0], removed_vals[1]);
        cll.add_after(removed_vals[1], removed_vals[2]);

        //Increase index
        cur_value = cll.next(cur_value);
    }

    cll
}

fn parse_input(inp: &str) -> Vec<u32> {
    inp.chars().map(|c| c as u32 - '0' as u32).collect()
}

struct FakeCLL {
    map: Vec<u32>
}

impl FakeCLL {
    fn from_vec(vec: Vec<u32>) -> Self {
        let mut fll = FakeCLL {map: vec![0; vec.len()+1] };

        //Make linked list
        vec.windows(2).for_each(|slice| {
            fll.map[slice[0] as usize] = slice[1];
        });

        //Make circularly linked list
        fll.map[vec[vec.len()-1] as usize] = vec[0];

        fll
    }

    #[inline]
    fn next(&self, cur: u32) -> u32 {
        self.map[cur as usize]
    }

    #[inline]
    fn add_after(&mut self, cur: u32, next: u32) {
        self.map[next as usize] = self.next(cur);
        self.map[cur as usize] = next;
    }

    #[inline]
    fn remove_next(&mut self, cur: u32) -> u32 {
        let val = self.map[cur as usize];
        self.map[cur as usize] = self.next(self.next(cur));
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1_ex1() {
        let result = part1("389125467");
        assert_eq!("67384529", result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!("97624853", result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2("389125467");
        assert_eq!(149245887792, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(664642452305, result);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            assert_eq!(664642452305, part2(input));
        });
    }
}



