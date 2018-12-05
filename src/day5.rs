pub fn execute_exercises() {
    //preprocess::pre_process(include_str!("../input/day4_in.txt")).into_iter().for_each(|(guard, sleep, wake)| println!("{} {} {}", guard, sleep, wake));
    println!("units remaining: {}", exercise_1(read_input()));
    println!("collapsed remaining: {}", exercise_2(read_input()));    
}

fn read_input() -> Vec<u8> {  
    include_str!("../input/day5_in.txt").bytes().collect()
}

fn exercise_1(input: Vec<u8>) -> usize {
    use std::collections::VecDeque;
    let mut seen: Vec<u8> = Vec::with_capacity(input.len());
    let mut unseen = VecDeque::from(input);
    
    // Start: dequeue first char C from unseen 
    // ---- part 1 ----
    // dequeue next char D from unseen.

    // If opposite polarity: don't add to seen stack and just drop chars C and D. Go to part 2
    // If no issues: add C to seen stack, set C = D and go to part 1.

    //
    // ---- part 2 ---- C and D got removed
    // scenario 1: stack is empty
    // Dequeue from unseen as char C go to 'part 1'
    //
    // scenario 2: Stack is not empty
    // take char C from stack and char D from unseen queue

    // If opposite polarity drop C and D and go to part 2.
    // If no issues: Add C back to seen stack and set C = D, and go to part 1.

    //
    // ----= part 3 ---- There is nothing left in unseen queue
    // Add char D to seen stack and exit
    
    let mut a = unseen.pop_front().unwrap();
    let mut b: Option<u8> = None;  

    loop {
        let bn = match b {
            Some(b) => b,
            None => match unseen.pop_front() {
                Some(b) => b,                
                None => break
            }
        };

        let result = if a == bn + 32 || a == bn - 32 { // Check if oposite polarities,
            // There are polarities, so drop a and b.
            if seen.len() == 0 { 
                // If there is no seen chars, just continue as usual from the start.                
                match unseen.pop_front() { 
                    Some(a) => (a, None),
                    None => break
                }
            } else {
                // There are seen chars, take this one
                (seen.pop().unwrap(), unseen.pop_front())
            }
        } else {
            seen.push(a);            
            (bn, None)
        };

        a = result.0;
        b = result.1;


    }

    seen.push(a);
    seen.len()
}


fn exercise_2(input: Vec<u8>) -> usize {
    use std::thread;
    
    let lower_case: Vec<u8> = "abcdefghijklmnopqrstuvwxyz".bytes().collect();
    let upper_case: Vec<u8> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".bytes().collect();
    
    // Uhm, if you remove chars and then collapse, isn't it the same as collapsing then removing chars?
    // nope, when removing c/C from aCA then collapsing gives: []
    // collapsing then removing c/C gives [aA]

    let children: Vec<thread::JoinHandle<usize>> = (0..26).into_iter().map(|i| {
        let l = lower_case[i];
        let u = upper_case[i];
        let next: Vec<u8> = input.iter().filter(|x| **x != l && **x != u).cloned().collect();
        thread::spawn(move || {            
            exercise_1(next)
        })
    }).collect();

    children.into_iter().map(|x| x.join().unwrap()).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;


    #[test]
    fn d5_ex1_s1() {
        let input = "dabAcCaCBAcCcaDA".bytes().collect();
        assert_eq!(exercise_1(input), 10);        
    }
    
    #[test]
    fn d5_ex1_s2() {
        let input = read_input();
        assert_eq!(exercise_1(input), 10368);
    }

    #[test]
    fn d5_ex2_s1() {
        let input = "dabAcCaCBAcCcaDA".bytes().collect();
        assert_eq!(exercise_2(input), 4);
    }

    #[test]
    fn d5_ex2_s2() {
        let input = read_input();
        assert_eq!(exercise_2(input), 4122);
    }

    #[bench]
    fn d5_bench_read(b: &mut Bencher) {
        b.iter(|| read_input());
    }

    #[bench]
    fn d5_bench_ex1(b: &mut Bencher) {        
        b.iter(|| exercise_1(read_input()));
    }

    #[bench]
    fn d5_bench_ex2(b: &mut Bencher) {        
        b.iter(|| exercise_2(read_input()));
    }
}