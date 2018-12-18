#![feature(vec_remove_item)]
#![feature(test)]
extern crate test;
extern crate hashbrown;
extern crate rayon;
extern crate ocl;

macro_rules! run {
    ($($x:ident), *) => {
        $(
            mod $x;
        )*
        fn main() {
            $(
                $x::execute_exercises();
            )*
        }
    }
}

//run!(day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14, day15, day16);
run!(day18);

