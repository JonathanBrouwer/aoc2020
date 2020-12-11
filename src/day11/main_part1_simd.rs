extern crate test;
extern crate strum;
extern crate strum_macros;
extern crate bitvec;
extern crate packed_simd_2;

use std::mem::swap;
use packed_simd_2::*;
use arrayvec::ArrayVec;

fn part1_simd(inp: &str) -> usize {
    let dim = (inp.lines().count(), inp.lines().next().unwrap().len());
    let width = dim.1 + 2;

    // -- PARSE INPUT --
    let mut state = ArrayVec::<[u8; 128 * 128]>::new();
    for _ in 0..width { state.push(16) } //Buffer
    for line in inp.lines() {
        state.push(16); //Buffer
        for char in line.chars() {
            if char == '.' {
                state.push(16);
            } else {
                state.push(1);
            }
        }
        state.push(16); //Buffer
    }
    for _ in 0..width { state.push(16) }

    // -- INIT COUNT --
    let mut state_new = ArrayVec::<[u8; 128 * 128]>::new();
    unsafe { state_new.set_len(state.len()) };

    loop {
        let mut changed = false;
        for row in 1..dim.0 + 1 {
            for side in 0..2 {
                //Calculate where this entry starts
                let start = if side == 0 { row * width + 1 } else { row * width + width - 65 };

                //Calculate new count in lower 4 bits
                let mut count = u8x64::splat(0);
                count += u8x64::from_slice_unaligned(&state[start - width - 1..]);
                count += u8x64::from_slice_unaligned(&state[start - width..]);
                count += u8x64::from_slice_unaligned(&state[start - width + 1..]);
                count += u8x64::from_slice_unaligned(&state[start - 1..]);
                count += u8x64::from_slice_unaligned(&state[start + 1..]);
                count += u8x64::from_slice_unaligned(&state[start + width - 1..]);
                count += u8x64::from_slice_unaligned(&state[start + width..]);
                count += u8x64::from_slice_unaligned(&state[start + width + 1..]);
                count &= u8x64::splat(0x0f);

                //To calculate the new state, start with the old state
                let old = u8x64::from_slice_unaligned(&state[start..]);
                let mut new = old;

                //If count == 0, set seat to full (1), else keep old value
                new = count.eq(u8x64::splat(0)).select(u8x64::splat(1), new);

                //If counts >= 4, set seat to empty (0), else keep old value
                new = count.ge(u8x64::splat(4)).select(u8x64::splat(0), new);

                //If the old value was a floor (16), set new value to floor (16)
                new = old.eq(u8x64::splat(16)).select(u8x64::splat(16), new);

                //Write new value to new state
                new.write_to_slice_unaligned(&mut state_new[start..]);
                //Check if anything changed
                changed |= new.ne(old).any();
            }
        }

        if !changed {break;}
        swap(&mut state, &mut state_new);
    }

    let mut count = 0;
    for row in 1..dim.0 + 1 {
        let start = row*width+1;
        count += state_new[start..start+dim.1].iter().filter(|&&x| x != 16 && x != 0).count();
    }
    count
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use test::Bencher;
    use std::time::Instant;

    #[test]
    pub(crate) fn test_part1_simd_test() {
        let result = part1_simd(include_str!("test"));
        println!("Part 1: {}", result);
        assert_eq!(37, result);
    }

    #[test]
    pub(crate) fn test_part1_simd_real() {
        let start = Instant::now();

        let result = part1_simd(include_str!("input"));
        assert_eq!(2275, result);

        println!("elapsed {:?}", start.elapsed());
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            part1_simd(input)
        });
    }
}
