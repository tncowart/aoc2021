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
    let to_visit = &caves[node] - visited;
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
            acc.extend(visit_caves(n, caves, &v, &p, double));
            acc
        })
    }
}

pub fn day12() {
    let caves = include_str!("../resources/day12.txt")
        .lines()
        .filter(|l| !(*l).is_empty())
        .map(|l| l.split_once('-').unwrap())
        .fold(HashMap::<&str, HashSet<&str>>::new(), |mut acc, (ra, rb)| {
            acc.entry(ra).or_insert_with(HashSet::new).insert(rb);
            acc.entry(rb).or_insert_with(HashSet::new).insert(ra);
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
                acc.extend(visit_caves(
                    "start",
                    &caves,
                    &HashSet::from(["start"]),
                    &Vec::new(),
                    Some(k),
                ));
                acc
            })
            .len()
    );
}
