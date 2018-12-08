#![feature(test)]
extern crate test;

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
#[allow(dead_code)] mod day7;*/
#[allow(dead_code)] mod day8;

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
