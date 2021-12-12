use std::collections::{HashMap, HashSet};

fn is_lower(s: &str) -> bool {
    s.to_ascii_lowercase() == s
}

fn visit_caves<'a>(
    node: &'a str,
    caves: &HashMap<&str, HashSet<&'a str>>,
    visited: &HashSet<&'a str>,
    path: &Vec<&'a str>,
    double: Option<&str>,
) -> Vec<Vec<&'a str>> {
    let to_visit = caves[node].difference(visited).collect::<HashSet<_>>();
    let mut v = visited.clone();
    if is_lower(node) && Some(node) != double {
        v.insert(node);
    }
    let mut p = path.clone();
    p.push(node);
    let double = if Some(node) == double { None } else { double };
    if node == "end" {
        vec![p]
    } else if to_visit.is_empty() {
        Vec::new()
    } else {
        to_visit.iter().fold(Vec::new(), |mut acc, n| {
            for pa in visit_caves(n, caves, &v, &p, double) {
                acc.push(pa)
            }
            acc
        })
    }
}

pub fn day12() {
    let caves = include_str!("../resources/day12.txt")
        .lines()
        .filter(|l| !(*l).is_empty())
        .map(|l| l.split('-'))
        .fold(HashMap::<&str, HashSet<&str>>::new(), |mut acc, l| {
            let rooms: Vec<&str> = l.collect();
            if let Some(connections) = acc.get_mut(rooms[0]) {
                connections.insert(rooms[1]);
            } else {
                acc.insert(rooms[0], HashSet::from([rooms[1]]));
            }
            if let Some(connections) = acc.get_mut(rooms[1]) {
                connections.insert(rooms[0]);
            } else {
                acc.insert(rooms[1], HashSet::from([rooms[0]]));
            }
            acc
        });

    println!(
        "Day 12.1: {}",
        visit_caves(
            "start",
            &caves,
            &HashSet::from(["start"]),
            &Vec::new(),
            None
        )
        .len()
    );

    println!(
        "Day 12.2: {}",
        caves
            .keys()
            .filter(|k| is_lower(k) && **k != "end" && **k != "start")
            .fold(HashSet::new(), |mut acc, k| {
                for p in visit_caves(
                    "start",
                    &caves,
                    &HashSet::from(["start"]),
                    &Vec::new(),
                    Some(k),
                ) {
                    acc.insert(p);
                }
                acc
            })
            .len()
    );
}
