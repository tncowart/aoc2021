use std::collections::{HashMap, HashSet};

fn numset2num(numset: HashSet<char>) -> u32 {
    if numset == HashSet::from(['a', 'b', 'c', 'e', 'f', 'g']) {
        0
    } else if numset == HashSet::from(['c', 'f']) {
        1
    } else if numset == HashSet::from(['a', 'c', 'd', 'e', 'g']) {
        2
    } else if numset == HashSet::from(['a', 'c', 'd', 'f', 'g']) {
        3
    } else if numset == HashSet::from(['b', 'c', 'd', 'f']) {
        4
    } else if numset == HashSet::from(['a', 'b', 'd', 'f', 'g']) {
        5
    } else if numset == HashSet::from(['a', 'b', 'd', 'e', 'f', 'g']) {
        6
    } else if numset == HashSet::from(['a', 'c', 'f']) {
        7
    } else if numset == HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']) {
        8
    } else if numset == HashSet::from(['a', 'b', 'c', 'd', 'f', 'g']) {
        9
    } else {
        u32::MAX
    }
}

fn process_input(input: &str) -> HashMap<char, char> {
    let one = input
        .split_ascii_whitespace()
        .find(|w| w.len() == 2)
        .unwrap();
    let four = input
        .split_ascii_whitespace()
        .find(|w| w.len() == 4)
        .unwrap();
    let counts = input
        .replace(" ", "")
        .chars()
        .fold(HashMap::<char, u32>::new(), |mut acc, c| {
            acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
            acc
        });

    counts
        .iter()
        .fold(HashMap::<char, char>::new(), |mut acc, (key, value)| {
            if *value == 4 {
                acc.insert(*key, 'e');
            } else if *value == 6 {
                acc.insert(*key, 'b');
            } else if *value == 7 {
                if four.contains(*key) {
                    acc.insert(*key, 'd');
                } else {
                    acc.insert(*key, 'g');
                }
            } else if *value == 8 {
                if one.contains(*key) {
                    acc.insert(*key, 'c');
                } else {
                    acc.insert(*key, 'a');
                }
            } else if *value == 9 {
                acc.insert(*key, 'f');
            }

            acc
        })
}

fn translate(key: HashMap<char, char>, text: &str) -> u32 {
    numset2num(text.chars().fold(HashSet::new(), |mut acc, c| {
        acc.insert(*key.get(&c).unwrap());
        acc
    }))
}

pub fn day8() {
    let p1: usize = include_str!("../resources/day8.txt")
        .lines()
        .map(|l| {
            l.split(" | ")
                .last()
                .unwrap()
                .split_ascii_whitespace()
                .filter(|w| [2, 4, 3, 7].contains(&w.len()))
                .count()
        })
        .sum();

    println!("Day 8.1: {}", p1);

    let p2: u32 = include_str!("../resources/day8.txt")
        .lines()
        .map(|line| {
            let io = line.split(" | ").collect::<Vec<&str>>();
            io[1]
                .split_whitespace()
                .map(|num| translate(process_input(io[0]), num))
                .reduce(|acc, num| acc * 10 + num)
                .unwrap()
        })
        .sum();

    println!("Day 8.2: {}", p2);
}
