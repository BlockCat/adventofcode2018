use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn execute_exercises() {
    println!("Checksum ex1: {}", exercise_1(read_input()));
    println!("Common between boxes: {}", exercise_2(read_input()));
}

#[derive(Debug, PartialEq)]
struct PartialCheckSum {
    doubles: bool,
    triples: bool
}

fn exercise_1(input: Vec<String>) -> i32 {
    let ids: Vec<PartialCheckSum> = input.into_iter().map(check_id).collect();

    let doubles = ids.iter().filter(|pcs| pcs.doubles).count() as i32;
    let triples = ids.iter().filter(|pcs| pcs.triples).count() as i32;

    doubles * triples
}

fn exercise_2(input: Vec<String>) -> String {
    for i in 0..input.len() {        
        for j in i..input.len() {
            if input[i].chars().zip(input[j].chars()).filter(|(a, b)| a != b).count() == 1 {
                return input[i].chars().zip(input[j].chars()).filter(|(a, b)| a == b).map(|(_, b)| b).collect();
            }
        }
    }

    String::from("")
}

fn read_input() -> Vec<String> {
    let file = File::open("input/day2_in.txt").expect("Could not open file: input.txt for exercise 1");
    let reader = BufReader::new(file);
    return reader.lines().map(|l| l.unwrap()).collect();
}


fn check_id(id: String) -> PartialCheckSum {    
    let mut grouping = HashMap::new();
    for ch in id.chars() {
        *grouping.entry(ch).or_insert(0) += 1;
    }   
    PartialCheckSum {
        doubles: grouping.iter().any(|(_, v)| *v == 2),
        triples: grouping.iter().any(|(_, v)| *v == 3)
    }
}

#[cfg(test)]
mod tests {
    
    use super::{PartialCheckSum, check_id, exercise_1, exercise_2};

    #[test]
    fn ex1_s1() {
        assert_eq!(check_id("abcdef".to_string()), PartialCheckSum { doubles: false, triples: false});
        assert_eq!(check_id("bababc".to_string()), PartialCheckSum { doubles: true, triples: true});
        assert_eq!(check_id("abbcde".to_string()), PartialCheckSum { doubles: true, triples: false});
        assert_eq!(check_id("abcccd".to_string()), PartialCheckSum { doubles: false, triples: true});
        assert_eq!(check_id("aabcdd".to_string()), PartialCheckSum { doubles: true, triples: false});
        assert_eq!(check_id("abcdee".to_string()), PartialCheckSum { doubles: true, triples: false});
        assert_eq!(check_id("ababab".to_string()), PartialCheckSum { doubles: false, triples: true});
    }

    #[test]
    fn ex1_s2() {
        let v = vec!("abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab").into_iter().map(|t| t.to_string()).collect();
        assert_eq!(exercise_1(v), 12);
    }

    #[test]
    fn ex2_s1() {
        let v: Vec<String> = vec!("abcde","fghij","klmno","pqrst","fguij","axcye","wvxyz").into_iter().map(|t| t.to_string()).collect();
        assert_eq!(exercise_2(v), String::from("fgij"));
    }
}
