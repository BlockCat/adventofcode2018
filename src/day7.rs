
pub fn execute_exercises() {
    println!("Order: {}", exercise_1(read_input()));
}

fn read_input() -> Vec<(char, char)> {
    parse_input(include_str!("../input/day7_in.txt"))
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    input.lines().map(|l| {
        let a = l[5..6].chars().next().unwrap();
        let b = l[36..37].chars().next().unwrap();

        (a, b)
    }).collect::<Vec<(char, char)>>()
}

#[derive(Eq, PartialEq)]
struct Node {
    value: char,
    next: Vec<char>
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


fn exercise_1(input: Vec<(char, char)>) -> String {
    use std::collections::BinaryHeap;
    use std::collections::HashMap;

    let mut steps: Vec<Node> = (b'A'..(b'Z'+1)).map(|c| {
        Node {
            value: c as char,
            next: vec!()
        }
    }).collect();

    let mut constraints = HashMap::new();

    for (prec, todo) in input.iter() {        
        steps[(*prec as u8 - b'A')  as usize].next.push(*todo);
        constraints.entry(*prec).or_insert(0);
        let cons = constraints.entry(*todo).or_insert(0);
        *cons += 1;        
        //constraints[(*todo as u8- b'A') as usize] += 1;
    }

    let mut seen = vec!();
    let mut heap = BinaryHeap::new();
    // Find all chars that have no constraints
    for (i, c) in constraints.iter().filter(|(_, c)| **c == 0) {        
        heap.push(&steps[(*i as u8 - b'A') as usize]);
    }

    while !heap.is_empty() {
        let node = heap.pop().unwrap();
        seen.push(node.value);
        for next_node in node.next.iter() {
            //constraints[((*next_node as u8) - b'A') as usize] -= 1;
            let precs = constraints.entry(*next_node).or_insert(0);
            //let precs = constraints[((*next_node as u8) - b'A') as usize];
            *precs -= 1;
            if *precs == 0 {
                heap.push(&steps[(*next_node as u8 - b'A')  as usize]);
            }
        }
    }

    seen.iter().collect::<String>()
}

fn exercise_2(input: Vec<(char, char)>, workers: u8, seconds_per_step: u8) -> i32 {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d7_ex1_s1() {
        let input = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        assert_eq!(exercise_1(parse_input(input)), "CABDFE");
    }

}