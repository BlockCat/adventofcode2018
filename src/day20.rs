use std::ops::Add;
use hashbrown::HashMap;

pub fn execute_exercises() {
    let samples = parse_input(include_str!("../input/day20_in.txt"));
    //println!("{:?}", samples);
    println!("Distance: {}", exercise_1(samples));
    
}

type Location = (isize, isize);

#[derive(Debug, Clone)]
enum Direction {
    N, E, S, W
}

impl Add<Direction> for Location {
    type Output = Location;

    fn add(self, other: Direction) -> Location {
        let (x, y) = self;
        match other {
            Direction::N => (x, y - 1),
            Direction::S => (x, y + 1),
            Direction::E => (x + 1, y),
            Direction::W => (x - 1 , y)
        }
    }
}

#[derive(Debug)]
struct Tree {
    id: usize,
    path: Vec<Direction>,
    children: Vec<usize>,
    parent: Vec<usize>,
}

impl Tree {

    fn new(id: usize, parent: Option<usize>) -> Tree {
        Tree {
            id: id,
            path: vec!(),
            children: vec!(),
            parent: match parent {
                Some(b) => vec!(b),
                None => vec!()
            }
        }
    }

    fn push(&mut self, direction: Direction) -> usize {
        self.path.push(direction);
        self.id
    }
}

fn parse_input(input: &str) -> Vec<Tree> {
    use std::collections::VecDeque;

    
    //let mut path = Vec::new();
    let mut id = 0;
    let mut nodes: Vec<Tree> = Vec::new();
    let mut tree = id;

    nodes.push(Tree::new(id, None));
    id += 1;

    

    for c in input.chars() {
        tree = match c {
            'N' => nodes[tree].push(Direction::N),
            'E' => nodes[tree].push(Direction::E),
            'S' => nodes[tree].push(Direction::S),
            'W' => nodes[tree].push(Direction::W),
            '(' => {
                let c = Tree::new(id, Some(tree));
                id += 1;
                nodes.push(c);
                nodes[tree].children.push(id - 1);

                id - 1
            },
            '|' => {
                let parent = nodes[tree].parent[0];
                let c = Tree::new(id, Some(parent));
                nodes.push(c);
                id += 1;

                nodes[parent].children.push(id - 1);
                
                id - 1
            },
            ')' => {
                let parent = nodes[tree].parent[0];
                let mut c = Tree::new(id, None);
                c.parent = nodes[parent].children.clone();

                for child in &c.parent {
                    nodes[*child].children.push(c.id);
                }

                nodes.push(c);
                id += 1;               

                id - 1
            },
            _ => unreachable!()
        };
    }

    nodes
}

fn create_map(node: usize, mut dist: usize, mut loc: Location, input: &Vec<Tree>, map: &mut HashMap<Location, usize>) {        
    // The distance can probably be done directly by keeping track of the distance it took to that square.
    // Add children of node 
    
    for d in &input[node].path {        
        loc = loc + d.clone();
        dist += 1;

        let odist = map.entry(loc).or_insert(std::usize::MAX);
        *odist= std::cmp::min(*odist, dist);
    }

    // split on children
    for c in &input[node].children {
        create_map(*c, dist, loc, input, map);
    }

    
}

fn exercise_1(input: Vec<Tree>) -> usize {
    let mut mapping = HashMap::new();
    mapping.insert((0, 0), 0);
    create_map(0, 0, (0, 0), &input, &mut mapping);


    *mapping.values().max().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day19_ex1_s1() {
       let input = r"ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN";
        let input = parse_input(input);
        println!("{:?}", input);
        let result = exercise_1(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn day19_ex1_s2() {
       let input = r"ENWWW(NEEE|SSE(EE|N))";
        let input = parse_input(input);
        println!("{:?}", input);
        let result = exercise_1(input);
        assert_eq!(result, 10);
    }

    #[test]
    fn day19_ex1_s3() {
       let input = r"WNE";
        let input = parse_input(input);
        println!("{:?}", input);
        let result = exercise_1(input);
        assert_eq!(result, 3);
    }
}