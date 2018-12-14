#![feature(vec_remove_item)]
#![feature(test)]
extern crate test;
extern crate hashbrown;
extern crate rayon;

macro_rules! execute {
    ($x:ident) => {
        $x::execute_exercises();
    };
}

/*#[allow(dead_code)] mod day1;
#[allow(dead_code)] mod day2;
#[allow(dead_code)] mod day3;
#[allow(dead_code)] mod day4;
#[allow(dead_code)] mod day5;
#[allow(dead_code)] mod day6;
#[allow(dead_code)] mod day7;
#[allow(dead_code)] mod day8;
#[allow(dead_code)] mod day9;
#[allow(dead_code)] mod day10;
#[allow(dead_code)] mod day11;
#[allow(dead_code)] mod day12;*/
//mod day13;
mod day14;
fn main() {
    /*execute!(day1);
    execute!(day2);
    execute!(day3);
    execute!(day4);
    execute!(day5);
    execute!(day6);
    execute!(day7);
    execute!(day8);
    execute!(day9);
    execute!(day10);
    execute!(day12);
    execute!(day13);*/
    execute!(day14);
}
