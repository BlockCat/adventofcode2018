use std::collections::VecDeque;

pub fn execute_exercises() {
    exercise_1(read_input());
}

fn read_input() -> Vec<(i32, i32, i32, i32)> {
    parse_input(include_str!("../input/day10_in.txt"))
}

fn parse_input(input: &'static str) -> Vec<(i32, i32, i32, i32)> {
    input.lines().map(|l| {
        let c: Vec<i32> = l.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        c.iter();
        (c[0], c[1], c[2], c[3])
    }).collect()
}

fn exercise_1(mut input: Vec<(i32, i32, i32, i32)>) {    
    
    let max_y = input.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min_y = input.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let mut prev_h = max_y - min_y + 1;    
    let mut counter = 0;
    loop {
        counter+=1;
        let (mut up, mut down) = (1000, -1000);
        for (x, y, dx, dy) in input.iter_mut() {
            *x += *dx;
            *y += *dy;

            if *y > down {
                down = *y;
            }
            if *y < up {
                up = *y;
            }            
            //println!("{}, {}", x, y);
        }
        if down-up < 10 {
            print(&input);
            println!("seconds: {}", counter);
        }

        if (down - up) > prev_h {
            break;
        } else {
            prev_h = down - up;
        }
    }    
}

fn print(input: &Vec<(i32, i32, i32, i32)>) {
    use hashbrown::HashSet;
    let (mut up, mut down) = (1000, -1000);
    let (mut left, mut right) = (1000, -1000);
    let c: HashSet<(i32, i32)> = input.iter()
        .inspect(|(x, y, _, _)| {
            if *x < left { left = *x; }
            if *x > right { right = *x; }
            if *y > down { down = *y; }
            if *y < up { up = *y; }
        }).map(|(x, y, _, _)| {
            (*x, *y)
        }).collect();

    for y in up-4..=down+4 {
        for x in left-4..=right+4 {
            if c.contains(&(x, y)) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!("");
    }
        
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d10_ex1_s1() {
        let input = r"9 1 0 2
7 0 -1 0
3 -2 -1 1
6 10 -2 -1
2 -4 2 2
-6 10 2 -2
1 8 1 -1
1 7 1 0
-3 11 1 -2
7 6 -1 -1
-2 3 1 0
-4 3 2 0
10 -3 -1 1
5 11 1 -2
4 7 0 -1
8 -2 0 1
15 0 -2 0
1 6 1 0
8 9 0 -1
3 3 -1 1
0 5 0 -1
-2 2 2 0
5 -2 1 2
1 4 2 1
-2 7 2 -2
3 6 -1 -1
5 0 1 0
-6 0 2 0
5 9 1 -2
14 7 -2 0
-3 6 2 -1";

        exercise_1(parse_input(input));
    }

    #[bench]
    fn d10_bench_ex1(b: &mut Bencher) {
        b.iter(|| exercise_1(read_input()));
    }
}