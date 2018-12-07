
pub fn execute_exercises() {
    println!("Order: {}", exercise_1(read_input()));
    println!("parallel time: {}", exercise_2(read_input(), 5, 60)); //1266 is too high
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

fn exercise_2(input: Vec<(char, char)>, workers: u8, seconds_per_step: i32) -> i32 {
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
    
    let mut heap = BinaryHeap::new();
    // Find all chars that have no constraints
    for (i, c) in constraints.iter().filter(|(_, c)| **c == 0) {        
        heap.push(&steps[(*i as u8 - b'A') as usize]);
    }

    let mut event_heap = BinaryHeap::new();
    let mut working: Vec<bool> = (0..workers).map(|x| false).collect();
    let mut time = 0;
    for worker in 0..workers { // Take initial jobs
        while !heap.is_empty() {
            let node = heap.pop().unwrap();
            let time = seconds_per_step + (node.value as u8 - b'A' + 1) as i32;
            event_heap.push((-time, worker, node));
            working[worker as usize] = true;
        }
    }

    while !event_heap.is_empty() {
        let (t, worker, node) = event_heap.pop().unwrap();
        time = -t; //We don't have min heaps
        println!("{}", time);

        for next_node in node.next.iter() {
            //constraints[((*next_node as u8) - b'A') as usize] -= 1;
            let precs = constraints.entry(*next_node).or_insert(0);
            //let precs = constraints[((*next_node as u8) - b'A') as usize];
            *precs -= 1;
            if *precs == 0 {
                heap.push(&steps[(*next_node as u8 - b'A')  as usize]);
            }
        }

        working[worker as usize] = false;        
        let c = working.iter().enumerate().filter(|(i, is_working)| !**is_working).map(|(a, b)| (a, *b)).collect::<Vec<(usize, bool)>>();
        for (free_worker, _) in c {
            while !heap.is_empty() {
                let node = heap.pop().unwrap();
                let time = time + seconds_per_step + (node.value as u8 - b'A' + 1) as i32;
                event_heap.push((-time, free_worker as u8, node));
                working[free_worker as usize] = true;
            }
        }
        
    }

    time + 1
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

    #[test]
    fn d7_ex2_s1() {
        let input = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        assert_eq!(exercise_2(parse_input(input), 2, 0), 15);
    
    }

}