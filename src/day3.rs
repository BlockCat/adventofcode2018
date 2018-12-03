use std::collections::HashMap;
use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;

#[derive(Debug, Clone)]
struct Area {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Area {
    fn new(id: i32, x: i32, y: i32, width: i32, height: i32) -> Area {
        Area { id, x, y, width, height }
    }

    fn intersects(&self, other: &Area) -> bool {
        self.id != other.id 
        && !(self.x >= other.x + other.width || other.x >= self.x + self.width || self.y >= other.y + other.height || other.y >= self.y + self.height)
    }
}


impl FromStr for Area {
    type Err = ParseIntError;    

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let re: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let caps = re.captures(source).unwrap();
        Ok(Area::new(
            caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(5).unwrap().as_str().parse::<i32>().unwrap())
        )
    }
}

pub fn execute_exercises() {        
    println!("Overlapping inches: {}", exercise_1(read_input()));
    println!("Non overlapping: {}", exercise_2(read_input()));    
}

fn read_input() -> Vec<Area> {    
    include_str!("../input/day3_in.txt").lines().map(|l| l.parse::<Area>().unwrap()).collect()
}

fn exercise_1(input: Vec<Area>) -> i32 {    

    // I feel horrible writing this
    let mut hmm = HashMap::with_capacity(1000000);
    input.iter().for_each(|a| {
        for x in a.x..(a.x + a.width) {
            for y in a.y..(a.y + a.height) {                
                *hmm.entry((x, y)).or_insert(0) += 1;
            }
        }
    });

    hmm.values().filter(|x| **x > 1).count() as i32

    // What about translating the 2d space to a 1d space and then using a sweep algorithm from left to right?

    // What about using this sweep algorithm immediatly? from top to bottom.
    // the events would be the start of a rectangle and the end of a rectangle.
    // There can be a bookkeeping of which rectangle has not overlapped
}

fn exercise_2(input: Vec<Area>) -> i32 {
    input.iter().find(|x| !input.iter().any(|y| x.intersects(y))).unwrap().id
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d3_ex1_s1() {
        let inputs = vec!(Area::new(1, 1, 3, 4, 4), Area::new(2, 3, 1, 4, 4), Area::new(3, 5, 5, 2, 2));
        assert_eq!(exercise_1(inputs), 4);
    }
    
    #[test]
    fn d3_ex1_s2() {
        let inputs = vec!(Area::new(1, 0, 0, 4, 4), Area::new(2, 3, 0, 4, 4), Area::new(3, 4, 4, 2, 2));
        assert_eq!(exercise_1(inputs), 4);
    }

    #[test]
    fn d3_ex2_s1() {
        let inputs = vec!(Area::new(1, 1, 3, 4, 4), Area::new(2, 3, 1, 4, 4), Area::new(3, 5, 5, 2, 2));
        assert_eq!(exercise_2(inputs), 3);
    }

    #[bench]
    fn d3_read(b: &mut Bencher) {
        b.iter(|| read_input());
    }

    #[bench]
    fn d3_ex1_bench(b: &mut Bencher) {
        b.iter(|| exercise_1(read_input()));
    }

    #[bench]
    fn d3_ex2_bench(b: &mut Bencher) {
        b.iter(|| exercise_2(read_input()));
    }
}