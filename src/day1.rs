use std::collections::VecDeque;

pub fn _day1(window: usize) -> i32 {
    let mut increases = 0;

    let mut queue: VecDeque<i32> = VecDeque::with_capacity(window + 1);

    for num in include_str!("../resources/day1.txt")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
    {
        queue.push_back(num);
        if queue.len() <= window {
            // Loading up the queue with the initial values
            continue;
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

// pub fn day1_short(window: usize) -> usize {
//     include_str!("../resources/day1.txt")
//     .lines()
//     .map(|l| l.parse::<i32>().unwrap())
//     .collect::<Vec<i32>>()
//     .array_windows()
//     .filter(|[x, _, _, y ]| x < y)
//     .count()
// }

pub fn day1() {
    println!("Day 01.1: {}", _day1(1));
    println!("Day 01.2: {}", _day1(3));
}
