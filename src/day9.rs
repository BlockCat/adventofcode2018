use std::collections::VecDeque;

pub fn execute_exercises() {
    println!("High Score: {}", exercise_1v2(446, 71522));
    println!("Larger High Score: {}", exercise_1v2(446, 7152200));
}

fn read_input() -> impl Iterator<Item = u8> {
    include_str!("../input/day8_in.txt").split(' ').map(|c| c.parse::<u8>().unwrap())
}

#[derive(Debug)]
struct Reiger {
    left: Vec<u32>,
    right: VecDeque<u32>,
    current: u32
}

impl Reiger {

    fn with_capacity(capacity: usize) -> Self {
        Reiger {
            left: Vec::with_capacity(capacity / 2),
            right: VecDeque::with_capacity(capacity / 2),
            current: 0
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
         self.current = self.right.pop_front().unwrap_or_else(|| {
            self.left.pop().unwrap()
         });

        v
     }
}

fn exercise_1(players: u32, marbles: u32) -> u64 {
    let mut counter = 0;
    let mut scores = (0..players).map(|_| 0u64).collect::<Vec<_>>();
    let mut container = Vec::with_capacity(marbles as usize);
    container.push(0u32);

    for marble in 1..=marbles {
        if marble % 23 > 0 {
            counter = (counter + 2) % container.len();
            container.insert(counter + 1, marble);
        } else {
            counter = if (counter as i32 - 7) < 0 {
                counter + container.len()
            } else {
                counter
            } - 7;

            let last_marble = container.remove(counter + 1);

            scores[((marble - 1) % players) as usize] += (marble + last_marble) as u64;
        }
    }
    scores.into_iter().max().unwrap()
}

fn exercise_1v2(players: u32, marbles: u32) -> u64 {
    let mut state = Reiger::with_capacity(marbles as usize);    
    let mut scores = (0..players).map(|_| 0u64).collect::<Vec<_>>();
    for marble in 1..=marbles {
        if marble % 23 > 0 {
            state.right(1);
            state.insert(marble);
        } else {
            state.left(7);
            let value = state.remove();            
            scores[((marble - 1) % players) as usize] += (marble + value) as u64;
        }        
    }
    scores.into_iter().max().unwrap()

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d9_ex1_s1() {        
        assert_eq!(exercise_1v2(9, 25), 32);
        assert_eq!(exercise_1v2(10, 1618), 8317);
        assert_eq!(exercise_1v2(13, 7999), 146373);
        assert_eq!(exercise_1v2(17, 1104), 2764);
        assert_eq!(exercise_1v2(21, 6111), 54718);
        assert_eq!(exercise_1v2(30, 5807), 37305);
    }
    
    #[bench]
    fn d9_bench_ex1v1(b: &mut Bencher) {
        b.iter(|| exercise_1(446, 71522));
    }

    #[bench]
    fn d9_bench_ex1v2(b: &mut Bencher) {
        b.iter(|| exercise_1v2(446, 71522));
    }

    #[bench]
    fn d9_bench_ex2v2(b: &mut Bencher) {
        b.iter(|| exercise_1v2(446, 7152200));
    }
}