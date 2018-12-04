use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeSet;

use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        
        let parts: Vec<&str> = source.split(' ').collect();
        let id = parts[0].replace('#', "").parse::<i32>().unwrap();
        let loc: Vec<i32> = parts[2].replace(':', "").split(',').map(|n| n.parse().unwrap()).collect();
        let size: Vec<i32> = parts[3].split('x').map(|n| n.parse().unwrap()).collect();
        
        Ok(Area::new(
            id,
            loc[0],
            loc[1],
            size[0],
            size[1]
        ))
    }
}

#[derive(Eq)]
enum SweepEvent<'a> {
    StartArea(&'a Area, i32),
    EndArea(&'a Area, i32)
}

impl<'a> std::cmp::Ord for SweepEvent<'a> {

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sy = match self {
            SweepEvent::StartArea(_, y) => y,
            SweepEvent::EndArea(_, y) => y
        };
        let oy = match other {
            SweepEvent::StartArea(_, y) => y,
            SweepEvent::EndArea(_, y) => y
        };

        sy.cmp(oy)
    }
}

impl<'a> std::cmp::PartialOrd for SweepEvent<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> std::cmp::PartialEq for SweepEvent<'a> {

    fn eq(&self, other: &Self) -> bool {
        let sy = match self {
            SweepEvent::StartArea(b, y) => (b, y),
            SweepEvent::EndArea(b, y) => (b, y)
        };
        let oy = match other {
            SweepEvent::StartArea(b, y) => (b, y),
            SweepEvent::EndArea(b, y) => (b, y)
        };

        sy == oy
    }
}

#[derive(Eq, PartialEq)]
enum HorizontalSwipe {
    Start(i32),
    End(i32)
}

impl std::cmp::Ord for HorizontalSwipe {

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sy = match self {
            HorizontalSwipe::Start(y) => y,
            HorizontalSwipe::End(y) => y
        };
        let oy = match self {
            HorizontalSwipe::Start(y) => y,
            HorizontalSwipe::End(y) => y
        };

        sy.cmp(oy)
    }
}


impl std::cmp::PartialOrd for HorizontalSwipe {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/*
impl std::cmp::PartialEq for HorizontalSwipe {

    fn eq(&self, other: &Self) -> bool {
        let sy = match self {
            HorizontalSwipe::Start(y) => y,
            HorizontalSwipe::End(y) => y
        };
        let oy = match self {
            HorizontalSwipe::Start(y) => y,
            HorizontalSwipe::End(y) => y
        };

        sy == oy
    }
}*/


pub fn execute_exercises() {        
    println!("Overlapping inches: {}", exercise_1(read_input()));
    println!("Non overlapping: {}", exercise_2(read_input()));    
}

fn read_input() -> Vec<Area> {    
    include_str!("../input/day3_in.txt").lines().map(|l| l.parse::<Area>().unwrap()).collect()
}

fn exercise_1_sl(input: Vec<Area>) -> i32 {
    
    let event_vector: Vec<SweepEvent> = input.iter()
        .map(|s| SweepEvent::StartArea(s, s.y))        
        .chain(input.iter().map(|s| SweepEvent::EndArea(s, s.y + s.height)))
        .collect();    

    let min_heap = BinaryHeap::from(event_vector);

    let mut minutes = 0i32;
    let mut previous_y = 0;
    let mut previous_overlapped = 0;
    let mut state: BTreeSet<HorizontalSwipe> = BTreeSet::new(); // Should be some ordered lists of

    min_heap.iter().fold((0i32, 0i32, 0i32), |(prev_y, prev_overlap, result), event| {
        match event {
            SweepEvent::StartArea(b, y) => {
                let n_result = result + (y - previous_y) * previous_overlapped; // Increment minutes with overlapped
                // Add x and x+width to state.
                state.insert(HorizontalSwipe::Start(b.x));
                state.insert(HorizontalSwipe::End(b.x + b.width));
                // Calculate overlapping
                let overlapped = 0;
                (*y, overlapped, n_result)
            },
            SweepEvent::EndArea(b, y) => {
                let n_result = result + (y - previous_y) * previous_overlapped; // Increment minutes with overlapped
                // Remove x and x+width to state.
                state.remove(&HorizontalSwipe::Start(b.x));
                state.remove(&HorizontalSwipe::End(b.x + b.width));
                // Calculate overlapping
                let overlapped = 0;
                (*y, overlapped, n_result)
            }
        }        
    }).2
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