#![feature(test)]
#![feature(iterator_fold_self)]
#![feature(str_split_once)]
#![feature(exclusive_range_pattern)]
#![feature(or_patterns)]
#![feature(const_generics)]
extern crate lazy_static;
extern crate scan_fmt;
extern crate strum_macros;
extern crate test;

mod template;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod infinl;


fn main() {
    day17::main::part2(include_str!("day17/input"));
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use crate::*;

    #[bench]
    fn all_days(b: &mut Bencher) {
        b.iter(|| {
            day1::main::tests::test_part1_real();
            day1::main::tests::test_part2_real();
            day2::main::tests::test_part1_real();
            day2::main::tests::test_part2_real();
            day3::main::tests::test_part1_real();
            day3::main::tests::test_part2_real();
            day4::main::tests::test_part1_real();
            day4::main::tests::test_part2_real();
            day5::main::tests::test_part1_real();
            day5::main::tests::test_part2_real();
            day6::main::tests::test_part1_real();
            day6::main::tests::test_part2_real();
            day7::main_faster::tests::test_part1_real();
            day7::main_faster::tests::test_part2_real();
            day8::main_faster::tests::test_part1_real();
            day8::main_faster::tests::test_part2_real();
            day9::main::tests::test_part1_real();
            day9::main::tests::test_part2_real();
            day10::main::tests::test_part1_real();
            day10::main::tests::test_part2_real();
            day11::main_part1_simd::tests::test_part1_simd_real();
            day11::main::tests::test_part2_real();
            day12::main::tests::test_part1_real();
            day12::main::tests::test_part2_real();
            day13::main::tests::test_part1_real();
            day13::main::tests::test_part2_real();
            day14::main::tests::test_part1_real();
            day14::main::tests::test_part2_real();
            day15::main::tests::test_part1_real();
            day15::main::tests::test_part2_real();
            day16::main::tests::test_part1_real();
            day16::main::tests::test_part2_real();
            day17::main::tests::test_part1_real();
            day17::main::tests::test_part2_real();
            day18::main_part1::tests::test_part1_real();
            day18::main_part2::tests::test_part2_real();
            day19::main_part1::tests::test_part1_real();
            day19::main_part2::tests::test_part2_real();
        });
    }
}