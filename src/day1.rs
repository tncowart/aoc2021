use std::collections::VecDeque;

use crate::utils::*;

pub fn day1(window: usize) -> i32 {
    let mut increases = 0;
    let mut prev_sum;
    let mut curr_sum = 0;

    let mut queue: VecDeque<i32> = VecDeque::with_capacity(window + 1);

    for line in lines("resources/day1.txt") {
        let num = line.unwrap().as_str().trim().parse::<i32>().unwrap();

        queue.push_back(num);
        if queue.len() <= window {
            curr_sum += num;
            continue
        }

        prev_sum = curr_sum;
        curr_sum = curr_sum - queue.pop_front().unwrap() + num;

        if curr_sum > prev_sum {
            increases += 1;
        }   
    }

    increases
}

pub fn day1_1() {
    println!("Day 01.1: {}", day1(1));
}

pub fn day1_2() {
    println!("Day 01.2: {}", day1(3));
}
