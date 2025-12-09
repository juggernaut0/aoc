#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

pub mod answers;
mod counter;
mod dir;
mod grid;
mod point;
mod point3d;
mod runner;
mod search;

pub use counter::*;
pub use dir::*;
pub use grid::*;
pub use point::*;
pub use point3d::*;
pub use runner::*;
pub use search::*;
use std::fmt::Debug;
use std::str::FromStr;

/**
# Panics

If a line does not parse into the given type.
*/
pub fn parse_lines<'a, T: FromStr + 'a>(input: &'a str) -> impl Iterator<Item = T> + 'a
where
    T::Err: Debug,
{
    parse_lines_with(input, |line| line.parse().unwrap())
}

pub fn parse_lines_with<'a, T, P: FnMut(&'a str) -> T + 'a>(
    input: &'a str,
    parser: P,
) -> impl Iterator<Item = T> + 'a {
    input.lines().map(parser)
}

pub fn rev_chars(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    loop {
        if b == 0 {
            return a;
        }
        let t = a % b;
        a = b;
        b = t;
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn pairs_without_dups<T>(a: &[T]) -> impl Iterator<Item = (&T, &T)> {
    a.iter().enumerate().flat_map(move |(i, x)| {
        a[(i+1)..]
            .iter()
            .map(move |y| (x, y))
    })
}
