#![feature(test)]
#![feature(iterator_fold_self)]
#![feature(str_split_once)]
#![feature(exclusive_range_pattern)]
extern crate lazy_static;
extern crate scan_fmt;
#[macro_use]
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
mod infinl;


fn main() {}

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
            day7::main::tests::test_part1_real();
            day7::main::tests::test_part2_real();
            day8::main::tests::test_part1_real();
            day8::main::tests::test_part2_real();
            day9::main::tests::test_part1_real();
            day9::main::tests::test_part2_real();
            day10::main::tests::test_part1_real();
            day10::main::tests::test_part2_real();
            day11::main::tests::test_part1_real();
            day11::main::tests::test_part2_real();
        });
    }
}