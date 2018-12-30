#![feature(const_str_len, dbg_macro)]

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
extern crate hashbrown;
#[macro_use]
extern crate intrusive_collections;
extern crate misc_utils;
extern crate rayon;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

aoc_lib! { year = 2018 }
