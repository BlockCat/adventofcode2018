use hashbrown::HashSet;
use hashbrown::HashMap;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::ops::Add;

type Location = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    North, East, South, West
}

impl Add<Direction> for Location {
    type Output = Location;

    fn add(self, other: Direction) -> Location {
        let (x, y) = self;
        match other {
            Direction::North => (x, y.saturating_sub(1)),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x.saturating_sub(1), y)
        }
    }
}

pub fn execute_exercises() {
    let (mapping, min_y, max_y) = parse_input(include_str!("../input/day17_in.txt"));
    println!("Amount: {}", exercise_1(mapping.clone(), (min_y, max_y)));
}

fn parse_input(input: &str) -> (HashSet<Location>, usize, usize) {
    let mut map = HashSet::new();
    let (mut min_y, mut max_y) = (std::usize::MAX, std::usize::MIN);

    for line in input.lines() {
        let mut line = line.split(", ");
        let a = line.next().unwrap();
        let b = line.next().unwrap();

        let first: usize = a[2..].parse().unwrap();
        let range: Vec<_> = b[2..].split("..").map(|s| s.parse().unwrap()).collect();        

        if &a[0..1] == "x" {
            min_y = std::cmp::min(min_y, range[0]);
            max_y = std::cmp::max(max_y, range[1]);
            for i in range[0]..=range[1] {
                map.insert((first, i));
            }
        } else {
            min_y = std::cmp::min(min_y, first);
            max_y = std::cmp::max(max_y, first);
            for i in range[0]..=range[1] {
                map.insert((i, first));
            }
        }
    }
    (map, min_y, max_y)
}

//what about this then
// store points that can go down (can hit water)
// loop{
// take point 
// Go down till hit
// scan left and right to see if we can go down further, 
// if yes: store point and set water flowing. end
// if no: set static and go one up

// end once
//}

//probably A*
fn exercise_1(mapping: HashSet<Location>, (min_y, max_y): (usize, usize)) -> usize {
    let mut water_mapping: HashMap<Location, bool> = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_front((500, 0));

    let mut downwards = VecDeque::new();

    while !queue.is_empty() {
        let nnn = queue.pop_back().unwrap();
        
        if nnn.1 > max_y {            
            //continue;
        } else {
            water_mapping.insert(nnn, true);
            let south = nnn + Direction::South;

            if mapping.contains(&south) || *water_mapping.entry(south).or_insert(false) { //hmm, don't thnk this is really correct
                let east = nnn + Direction::East;
                let west = nnn + Direction::West;

                if !water_mapping.contains_key(&east) && !mapping.contains(&east) {
                    queue.push_back(east);
                }

                if !water_mapping.contains_key(&west) && !mapping.contains(&west) {
                    queue.push_back(west);
                }
            } else {
                queue.push_back(south);
                downwards.push_back(nnn);
            }
        }

        if queue.is_empty() && !downwards.is_empty() {
            queue.push_back(downwards.pop_back().unwrap());              
        }

//        pretty_print(&mapping, &water_mapping, (min_y, max_y));
    }

    pretty_print(&mapping, &water_mapping, (min_y, max_y));
    
    water_mapping.len() - 1
}

fn pretty_print(mapping: &HashSet<Location>, water: &HashMap<Location, bool>, (min_y, max_y): (usize, usize)) {
    let min_x = mapping.iter().map(|s| s.0).min().unwrap();
    let max_x = mapping.iter().map(|s| s.0).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x-1..=max_x+1 {
            let pos = (x, y);
            if mapping.contains(&pos) {
                print!("#");
            } else if water.contains_key(&pos) {
                print!("~");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day17_ex1_s1() {
        let input = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
        let (mapping, min_y, max_y) = parse_input(input);
        assert_eq!(exercise_1(mapping, (min_y, max_y)), 57);
    }

    #[test]
    fn day17_ex1_s2() {
        let input = r"x=495, y=3..7
y=7, x=495..501
x=501, y=3..7
x=506, y=1..2
x=498, y=10..13
x=505, y=10..13
y=13, x=498..505
x=501, y=10..11
x=503, y=10..11
y=11, x=501..503
";
        let (mapping, min_y, max_y) = parse_input(input);
        exercise_1(mapping, (min_y, max_y));
    }
}