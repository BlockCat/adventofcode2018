use std::collections::VecDeque;

pub fn execute_exercises() {
    println!("High Score: {}", exercise_1(446, 71522));
    println!("Larger High Score: {}", exercise_1(446, 7_152_200));
}

#[derive(Debug)]
struct Reiger {    
    left: Vec<u32>,
    right: VecDeque<u32>,
    current: u32,
    counter: u32,
}

impl Iterator for Reiger {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        for _ in 0..22 {
            self.counter += 1;
            self.right(1);
            self.insert(self.counter);
        }

        self.counter += 1;
        self.left(7);
        let value = self.remove();
        Some((self.counter, value))            
    }
}

impl Reiger {

    fn with_capacity(capacity: usize) -> Self {
        Reiger {
            left: Vec::with_capacity(capacity / 2),
            right: VecDeque::with_capacity(capacity / 2),
            current: 0,
            counter: 0,
        }
    }
    
    fn len(&self) -> usize {
        (self.left.len() + self.right.len() + 1)
    }

    fn left(&mut self, count: usize) {
        let current_pos = self.left.len();        
        let target_pos = (self.len() + current_pos - count) % self.len();
        self.move_to(target_pos);
    }

    fn right(&mut self, count: usize) {
        let current_pos = self.left.len();
        let target_pos = (current_pos + count) % self.len();
        self.move_to(target_pos);
    }

     fn move_to(&mut self, index: usize) {
        let current_pos = self.left.len();        
        if current_pos > index {
            for _ in 0..(current_pos - index) {
                self.right.push_front(self.current);
                self.current = self.left.pop().unwrap();
            }
        } 
        if current_pos < index {
            for _ in 0..(index - current_pos) {
                self.left.push(self.current);
                self.current = self.right.pop_front().unwrap();                
            }
        }
     }

     fn insert(&mut self, value: u32) {
        self.left.push(self.current);
        self.current = value;
     }

     fn remove(&mut self) -> u32 {
        let v = self.current;
        self.current = self.right.pop_front().unwrap_or_else(|| {self.left.pop().unwrap()});

        v
     }
}


fn exercise_1(players: u32, marbles: usize) -> u64 {    
    let mut scores = (0..players).map(|_| 0u64).collect::<Vec<_>>();

    for (marble, score) in Reiger::with_capacity(marbles).take(marbles / 23) {
        scores[((marble - 1) % players) as usize] += (marble + score) as u64;
    }

    scores.into_iter().max().unwrap()

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d9_ex1_s1() {        
        assert_eq!(exercise_1(9, 25), 32);
        assert_eq!(exercise_1(10, 1618), 8317);
        assert_eq!(exercise_1(13, 7999), 146373);
        assert_eq!(exercise_1(17, 1104), 2764);
        assert_eq!(exercise_1(21, 6111), 54718);
        assert_eq!(exercise_1(30, 5807), 37305);
    }
    
    #[bench]
    fn d9_bench_ex1(b: &mut Bencher) {
        b.iter(|| exercise_1(446, 71522));
    }

    #[bench]
    fn d9_bench_ex2(b: &mut Bencher) {
        b.iter(|| exercise_1(446, 7152200));
    }
}