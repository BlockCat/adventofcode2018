use std::collections::HashMap;
use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32
}


impl FromStr for Date {
    type Err = ParseIntError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let re: Regex = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\]").unwrap();
        let caps = re.captures(source).unwrap();
        Ok(Date {
            year: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            month: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            day: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            hour: caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            minute: caps.get(5).unwrap().as_str().parse::<i32>().unwrap()})
        
    }
}

#[derive(Debug)]
enum GuardEvent {
    StartShift(Date, i32),
    StartSleep(Date),
    EndSleep(Date)
}

impl FromStr for GuardEvent {
    type Err = ();
    
    fn from_str(source: &str) -> Result<Self, ()> {
        let date = source.parse::<Date>().unwrap();
        let significant_char = &source[19..20];       

        match significant_char {
            "G" => {
                let re: Regex = Regex::new(r"#(\d+) ").unwrap();
                let caps = re.captures(source).unwrap();
                Ok(GuardEvent::StartShift(date, caps.get(1).unwrap().as_str().parse::<i32>().unwrap()))
            },
            "f" => Ok(GuardEvent::StartSleep(date)),
            "w" => Ok(GuardEvent::EndSleep(date)),
            _ => Err(())
        }
    }
}

pub fn execute_exercises() {
    //println!("strat 1: {}", exercise_1(read_input()));
    println!("strat 2: {}", exercise_2(read_input()));    
}

fn read_input() -> Vec<GuardEvent> {  
    read_input_str(include_str!("../input/day4_in.txt"))    
}

fn read_input_str(input:  &str) -> Vec<GuardEvent> {
    input.lines().map(|l| { l.parse::<GuardEvent>().unwrap()}).collect()
}

fn exercise_1(input: Vec<GuardEvent>) -> i32 {
    let sleepy_guard = find_most_sleeping_guard(&input);

    #[derive(Debug, PartialEq)] enum State {
        Sleep {guard: i32, sleep: i32, wake: i32},
        Wait
    };

    let (mm, _) = input
        .into_iter()
        .scan((0i32, 0i32), |(guard, sleep), event| {
        match event {
            GuardEvent::StartShift(_, g) => {
                *guard = g;
                Some(State::Wait)}
            GuardEvent::StartSleep(d) => { 
                *sleep = d.minute; 
                Some(State::Wait)}
            GuardEvent::EndSleep(d) => {
                if *guard == sleepy_guard {
                    Some(State::Sleep {
                        guard: *guard, 
                        sleep: *sleep, 
                        wake: d.minute
                    })
                } else {
                    Some(State::Wait)
                }
            }
        }        
    }).filter_map(|s| match s {
        State::Sleep { guard, sleep, wake} => Some((guard, sleep, wake)),
        _ => None
    }).fold([0u8; 60], |mut acc, (guard, sleep, wake)| {
        for i in sleep..wake {
            acc[i as usize] += 1;
        }

        acc
    }).into_iter()
    .enumerate()
    .max_by(|(_, a), (_, b)| a.cmp(b))
    .unwrap();

    sleepy_guard * (mm as i32)    
}

fn process_input(input: Vec<GuardEvent>) -> Vec<(i32, i32, i32)> {

    #[derive(Debug, PartialEq)] enum State {
        Sleep {guard: i32, sleep: i32, wake: i32},
        Wait
    };

    input.into_iter()
        .scan((0i32, 0i32), |(guard, sleep), event| {
        match event {
            GuardEvent::StartShift(_, g) => {
                *guard = g;
                Some(State::Wait)}
            GuardEvent::StartSleep(d) => { 
                *sleep = d.minute; 
                Some(State::Wait)}
            GuardEvent::EndSleep(d) => {
                Some(State::Sleep {
                    guard: *guard, 
                    sleep: *sleep, 
                    wake: d.minute
                })                
            }
        }        
    }).filter_map(|s| match s {
        State::Sleep { guard, sleep, wake} => Some((guard, sleep, wake)),
        _ => None
    }).collect()
}

fn find_most_sleeping_guard(input: &Vec<GuardEvent>) -> i32 {
    let mut cumulative_map = HashMap::with_capacity(365); 

    let mut current_shift: Option<i32> = None;    
    let mut date: Option<Date> = None;

    let mut current_guard = 0;
    let mut current_max = 0;

    for event in input {
        match event {
            GuardEvent::StartShift(_, g) => current_shift = Some(*g),
            GuardEvent::StartSleep(d) => date = Some(*d),
            GuardEvent::EndSleep(d) => {                              
                let minutes = cumulative_map.entry(current_shift.unwrap()).or_insert(0);    
                *minutes += d.minute - date.unwrap().minute;

                if *minutes > current_max {
                    current_guard = current_shift.unwrap();
                    current_max = *minutes;
                }
            }
        }
    }


    current_guard
}

fn exercise_2(input: Vec<GuardEvent>) -> i32 {
    let mut guard_minutes = [[0u8; 60];4000];

    let mut current_shift: Option<i32> = None;
    let mut date: Option<Date> = None;

    for event in input {
        match event {
            GuardEvent::StartShift(d, g) => {                
                current_shift = Some(g);
            },
            GuardEvent::StartSleep(d) => date = Some(d),
            GuardEvent::EndSleep(d) => {                                
                let old_date = date.unwrap();
                for m in old_date.minute..d.minute {                    
                    guard_minutes[current_shift.unwrap() as usize][m as usize] += 1;
                }                
            }
        }
    }

    let mut max_guard = 0;
    let mut max_minute = 0;
    let mut max_sleepy_minute = 0;

    for g in 0..4000 {
        for m in 0..60 {
            if guard_minutes[g][m] > max_minute {
                max_minute = guard_minutes[g][m];
                max_guard = g as i32;
                max_sleepy_minute = m as i32;
            }
        }
    }

    max_guard * max_sleepy_minute
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d4_ex1_s1() {
        let c = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let input = read_input_str(c);
        //println!("{:?}", input);
        assert_eq!(find_most_sleeping_guard(&input), 10);
        assert_eq!(exercise_1(input), 240);
    }
    
    #[test]
    fn d4_ex1_s2() {     
        assert_eq!(exercise_1(read_input()), 36898);   
    }

    #[test]
    fn d4_ex2_s1() {
      let c = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let input = read_input_str(c);
        assert_eq!(exercise_2(input), 4455);
    }

    #[test]
    fn d4_ex2_s2() {
        assert_eq!(exercise_2(read_input()), 80711);
    }

    #[test]
    fn d4_ex2_s3() {
        let v = vec!(3, 1, 4, 2);
        println!("{:?}", v.iter().enumerate().max());
    }

    #[bench]
    fn d4_read(b: &mut Bencher) {
       b.iter(|| read_input());
    }

    #[bench]
    fn d4_ex1_bench(b: &mut Bencher) {
        
    }

    #[bench]
    fn d4_ex2_bench(b: &mut Bencher) {
       
    }
}