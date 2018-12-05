use std::collections::HashMap;


pub fn execute_exercises() {
    //preprocess::pre_process(include_str!("../input/day4_in.txt")).into_iter().for_each(|(guard, sleep, wake)| println!("{} {} {}", guard, sleep, wake));
    println!("units remaining: {}", exercise_1(read_input()));
    println!("collapsed remaining: {}", exercise_2(read_input()));    
}

fn read_input() -> Vec<u8> {  
    include_str!("../input/day5_in.txt").bytes().collect()
}

fn exercise_1(mut input: Vec<u8>) -> usize {
    let mut index: usize = 0;
    
    //println!("{}", input.iter().map(|x| *x as char).collect::<String>());       

    // In this algorithm, when removing indeces it shifts all elements coming after one to the left and then one to the left.
    // This is most likely the bottleneck in the algorithm.
    //
    // We want to keep a vector with seen chars and a vector with the unseen chars.
    // This way we should be able to use pop and dequeue methods which should be more mem efficient.
    // std::collections::VecDeque
    //
    // Start: dequeue first char C from unseen 
    // ---- part 1 ----
    // dequeue next char D from unseen.
    // If opposite polarity: don't add to seen stack and just drop chars C and D. Go to part 2
    // If no issues: add C to seen stack, set C = D and go to part 1.
    //
    // ---- part 2 ---- C and D got removed
    // scenario 1: stack is empty
    // Go to start
    //
    // scenario 2: Stack is not empty
    // take char C from stack and char D from unseen queue
    // If opposite polarity drop C and D and go to part 2.
    // If no issues: Add C back to seen stack and set C = D, and go to part 1.
    //
    // ----= part 3 ---- There is nothing left in unseen queue
    // Add char D to seen stack and exit

    

    while index < input.len() - 1 {
        let a = input[index];
        let b = input[index + 1];
        
        if a == b + 32 || a == b - 32 {
            input.remove(index);
            input.remove(index);

            if index > 0 {
                index -= 1;
            }            
            //Debug line
            //println!("{} -> {}: {}, {}", input.iter().map(|x| *x as char).collect::<String>(), index, a as char, b as char);
            
        } else {
            index += 1;
        }        
    }

    input.len()
}

fn exercise_2(input: Vec<u8>) -> usize {
    use std::thread;
    
    let lower_case: Vec<u8> = "abcdefghijklmnopqrstuvwxyz".bytes().collect();
    let upper_case: Vec<u8> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".bytes().collect();

    let children: Vec<thread::JoinHandle<usize>> = (0..26).into_iter().map(|i| {
        let l = lower_case[i];
        let u = upper_case[i];
        let next: Vec<u8> = input.iter().filter(|x| **x != l && **x != u).cloned().collect();
        thread::spawn(move || {            
            //println!("{}, {}", l as char, next.iter().map(|x| *x as char).collect::<String>());
            ////results[i] = exercise_1(next);
            exercise_1(next)
        })
    }).collect();

    children.into_iter().map(|x| x.join().unwrap()).min().unwrap()

    /*for i in 0..26 {
        let l = lower_case[i];
        let u = upper_case[i];        

        let next: Vec<u8> = input.iter().filter(|x| **x != l && **x != u).map(|x| *x).collect();
        //println!("{}, {}", l as char, next.iter().map(|x| *x as char).collect::<String>());
        results[i] = exercise_1(next);
    }*/

    //*results.into_iter().min().unwrap()
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
    fn d5_bench_ex1(b: &mut Bencher) {        
        b.iter(|| exercise_1(read_input()));
    }

    #[bench]
    fn d5_bench_ex2(b: &mut Bencher) {        
        b.iter(|| exercise_2(read_input()));
    }
}