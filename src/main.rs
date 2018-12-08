#![feature(test)]
extern crate test;

macro_rules! execute {
    ($x:ident) => {
        $x::execute_exercises();
    };
}
/*
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;*/
mod day8;

fn main() {
    /*execute!(day1);
    execute!(day2);
    execute!(day3);
    execute!(day4);
    execute!(day5);
    execute!(day6);
    execute!(day7);*/
    execute!(day8);
}
