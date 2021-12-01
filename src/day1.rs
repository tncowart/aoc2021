use std::collections::VecDeque;

use crate::utils::*;

pub fn day1(window: usize) -> i32 {
    let mut increases = 0;

    let mut queue: VecDeque<i32> = VecDeque::with_capacity(window + 1);

    for line in lines("resources/day1.txt") {
        let num = line.unwrap().as_str().trim().parse::<i32>().unwrap();

        queue.push_back(num);
        if queue.len() <= window {
            // Loading up the queue with the initial values
            continue
        }

        // The two windows share almost all the same numbers, except for the first 
        // number in the old window and last number in the new window -- so we just 
        // have to compare those.
        // Based on https://github.com/benknoble/advent2021/blob/main/day1/solution.rkt
        // With clarification from https://github.com/lojic/LearningRacket/blob/master/advent-of-code-2021/solutions/day01/day01-benknoble.rkt
        if num > queue.pop_front().unwrap() {
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
