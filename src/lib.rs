#[macro_use]
extern crate aoc_runner_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[allow(unused_imports)]
#[macro_use]
extern crate indoc;

#[macro_use]
mod aoc_tests;

pub mod day1;
pub mod day2;

aoc_runner_derive::aoc_lib!{ year = 2022 }
