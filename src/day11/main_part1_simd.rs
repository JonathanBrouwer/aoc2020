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
    let width = dim.1 + 1;

    // -- PARSE INPUT --
    let mut state = ArrayVec::<[u8; 128 * 128]>::new();
    for _ in 0..width { state.push(16) } //Buffer
    for line in inp.lines() {
        //Buffer, to make sure first element of a row is not next to last element of previous row
        state.push(16);
        for char in line.chars() {
            if char == '.' {
                state.push(16);
            } else {
                state.push(1);
            }
        }
    }
    for _ in 0..width { state.push(16) } //Buffer
    for _ in 0..64 { state.push(16) } //Buffer

    // -- INIT COUNT --
    let mut state_new = ArrayVec::<[u8; 128 * 128]>::new();
    for _ in 0..state.len() {
        state_new.push(0);
    }

    loop {
        let mut changed = false;
        for start in ((width+1)..=(state.len()-width-64-1)).step_by(64) {
            unsafe {
                //Calculate new count in lower 4 bits
                let mut count = u8x64::splat(0);
                count += u8x64::from_slice_unaligned_unchecked(&state[start - width - 1..]);
                count += u8x64::from_slice_unaligned_unchecked(&state[start - width..]);
                count += u8x64::from_slice_unaligned_unchecked(&state[start - width + 1..]);
                count += u8x64::from_slice_unaligned_unchecked(&state[start - 1..]);
                count += u8x64::from_slice_unaligned_unchecked(&state[start + 1..]);
                count += u8x64::from_slice_unaligned_unchecked(&state[start + width - 1..]);
                count += u8x64::from_slice_unaligned_unchecked(&state[start + width..]);
                count += u8x64::from_slice_unaligned_unchecked(&state[start + width + 1..]);
                count &= u8x64::splat(0x0f);

                //To calculate the new state, start with the old state
                let old = u8x64::from_slice_unaligned_unchecked(&state[start..]);
                let mut new = old;

                //If count == 0, set seat to full (1), else keep old value
                new = count.eq(u8x64::splat(0)).select(u8x64::splat(1), new);

                //If counts >= 4, set seat to empty (0), else keep old value
                new = count.ge(u8x64::splat(4)).select(u8x64::splat(0), new);

                //If the old value was a floor (16), set new value to floor (16)
                new = old.eq(u8x64::splat(16)).select(u8x64::splat(16), new);

                //Write new value to new state
                new.write_to_slice_unaligned_unchecked(&mut state_new[start..]);
                //Check if anything changed
                changed |= new.ne(old).any();
            }
        }

        if !changed {break;}
        swap(&mut state, &mut state_new);
    }

    let mut count = 0;
    for start in ((width+1)..=(state.len()-width-64-1)).step_by(64) {
        unsafe {
            let vec = u8x64::from_slice_unaligned_unchecked(&state[start..]);
            let occ = (vec & u8x64::splat(0x0f)).ne(u8x64::splat(0));
            count += occ.bitmask().count_ones();
        }
    }



    count as usize
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
    fn bench_part1_simd(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            part1_simd(input)
        });
    }
}
