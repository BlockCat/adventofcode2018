use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn execute_exercises() {
    println!("Calibrated frequency: {}", exercise_1(frequency_delta_list()));
    println!("First reoccuring frequency: {}", exercise_2(frequency_delta_list()));
}

fn exercise_1(frequency_list: Vec<i32>) -> i32{
    frequency_list.into_iter().sum::<i32>()
}

fn exercise_2(frequency_list: Vec<i32>) -> i32{    
    let mut counter = 0i32;

    let mut visited_set = HashSet::with_capacity(frequency_list.len());    
    visited_set.insert(0);
    
    for freq_change in frequency_list.into_iter().cycle() {
        counter += freq_change;
        if !visited_set.insert(counter) {
            break;
        }
    }
    
    counter
}

fn frequency_delta_list() -> Vec<i32> {
    let file = File::open("input/day1_in.txt").expect("Could not open file: input.txt for exercise 1");
    let reader = BufReader::new(file);

    return reader.lines().map(|l| l.unwrap()
            .parse::<i32>()
            .unwrap()).collect();
}

#[cfg(test)]
mod tests {
    
    use super::{exercise_1, exercise_2};

    #[test]
     fn ex1_s1() {        
        assert_eq!(exercise_1(vec![1, 1, 1]), 3);
    }

    #[test]
    fn ex1_s2() {        
        assert_eq!(exercise_1(vec![1, 1, -2]), 0);
    }

    #[test]
    fn ex1_s3() {
        assert_eq!(exercise_1(vec![-1, -2, -3]), -6);
    }
    
    #[test]
    fn ex2_s1() {
        assert_eq!(exercise_2(vec![1, -2, 3, 1]), 2);
    }

    #[test]
    fn ex2_s2() {
        assert_eq!(exercise_2(vec![1, -1]), 0);
    }

    #[test]
    fn ex2_s3() {
        assert_eq!(exercise_2(vec![3, 3, 4, -2, -4]), 10);
    }

    #[test]
    fn ex2_s4() {
        assert_eq!(exercise_2(vec![-6, 3, 8, 5, -6]), 5);
    }

    #[test]
    fn ex2_s5() {
        assert_eq!(exercise_2(vec![7, 7, -2, -7, -4]), 14);
    }

}