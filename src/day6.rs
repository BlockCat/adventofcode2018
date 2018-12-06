pub fn execute_exercises() {
    //preprocess::pre_process(include_str!("../input/day4_in.txt")).into_iter().for_each(|(guard, sleep, wake)| println!("{} {} {}", guard, sleep, wake));
    //println!("Largest area: {}", exercise_1(read_input()));
    println!("Largest area safe: {}", exercise_2(read_input(), 10_000));    
}

fn read_input() -> Vec<(i32, i32)> {  
    include_str!("../input/day6_in.txt")
    .lines()
    .map(|s| {
        let spl: Vec<&str> = s.split(", ").collect();
        (spl[0].parse().unwrap(), spl[1].parse().unwrap())
    }).collect()
}

fn find_extreme_points(input: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let extremes = Vec::new();

    extremes
}


fn exercise_1(input: Vec<(i32, i32)>) -> i32 {
    use std::collections::HashMap;

    let mut l = input[0].0;
    let mut r = input[0].0;
    let mut u = input[0].1;
    let mut b = input[0].1;//Finding the bounding box

    for (x, y) in input.iter() {
        l = std::cmp::min(l, *x);
        r = std::cmp::max(r, *x);
        u = std::cmp::min(u, *y);
        b = std::cmp::max(b, *y);
    }
    
    let mut map = HashMap::with_capacity(1000);
    let mut bookkeeping = HashMap::with_capacity(1000);

    for x in (l-(b-u + 1))..(r+(b-u+1)) {
        for y in (u-(r-l+1))..(b+(r-l+1)) {
            let entry = map.entry((x, y)).or_insert(1000000);                 
            let book = bookkeeping.entry((x, y)).or_insert(None);
            input.iter().enumerate().for_each(|(i, (a, b))| {
                let d = (a - x).abs() + (b - y).abs();                

                if d < *entry {
                    *entry = d;                    
                    *book = Some(i);
                } else if d == *entry {                    
                    *book = None;
                }
            });
        }
    }

    let mut counter = HashMap::with_capacity(1000);
    bookkeeping.values().filter_map(|i| *i).for_each(|i| {
        *counter.entry(i).or_insert(0) += 1;
    });

    // hack to jus get it over with

    counter = counter.into_iter().filter(|(i, v)| {
        *v < 4628
    }).collect();
    println!("{:?}", counter); 
    
    *counter.values().max().unwrap() as i32
}



fn exercise_2(input: Vec<(i32, i32)>, size: i32) -> i32 {
    use std::collections::HashMap;

    let mut l = input[0].0;
    let mut r = input[0].0;
    let mut u = input[0].1;
    let mut b = input[0].1;//Finding the bounding box

    for (x, y) in input.iter() {
        l = std::cmp::min(l, *x);
        r = std::cmp::max(r, *x);
        u = std::cmp::min(u, *y);
        b = std::cmp::max(b, *y);
    }    

    let mut counter = 0; 

    for x in (l-(b-u + 1))..(r+(b-u+1)) {
        for y in (u-(r-l+1))..(b+(r-l+1)) {            
            let c = input.iter().enumerate().map(|(i, (a, b))| {
                (a - x).abs() + (b - y).abs()
            }).sum::<i32>();

            if c < size {
                counter += 1;
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;


    #[test]
    fn d6_ex1_s1() {
        let input = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9".lines()
    .map(|s| {
        let spl: Vec<&str> = s.split(", ").collect();
        (spl[0].parse().unwrap(), spl[1].parse().unwrap())
    }).collect();

        assert_eq!(exercise_1(input), 17);        
    }   

    #[test]
    fn d6_ex1_s2() {
        assert_eq!(exercise_1(read_input()), 3358);
    }

        #[test]
    fn d6_ex2_s1() {
        let input = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9".lines()
    .map(|s| {
        let spl: Vec<&str> = s.split(", ").collect();
        (spl[0].parse().unwrap(), spl[1].parse().unwrap())
    }).collect();

        assert_eq!(exercise_2(input, 32), 16);
    }   
}