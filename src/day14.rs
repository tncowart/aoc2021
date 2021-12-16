use std::collections::HashMap;
pub fn day14() {
    let (poly_template, insert_rules_string) = include_str!("../resources/day14.txt").split_once("\n\n").unwrap();
    let insert_rules: HashMap<String, u8> = insert_rules_string.lines().fold(HashMap::new(), |mut acc, l| {
        let (key, val) = l.split_once(" -> ").unwrap();
        acc.insert(key.to_string(), val.as_bytes()[0]);
        acc
    });
    let pair_counts_blank: HashMap<String, u64> = insert_rules_string.lines().fold(HashMap::new(), |mut acc, l| {
        acc.insert(l.split_once(" -> ").unwrap().0.to_string(), 0);
        acc
    });
    let mut letter_counts: HashMap<u8, u64> = poly_template.bytes().fold(HashMap::new(), |mut acc, b| {
        acc.entry(b).and_modify(|e| *e += 1).or_insert(1);
        acc
    });
    let poly_template = poly_template.trim();
    let mut new_pair_counts = pair_counts_blank.clone();
    for i in 0..(poly_template.len() - 1) {
        new_pair_counts.entry(poly_template[i..=i+1].to_string()).and_modify(|e| { *e += 1 });
    }

    let pair_counts = (0..10).fold(new_pair_counts, |acc, _| {
        let mut new_pair_counts = pair_counts_blank.clone();
        for key in acc.keys() {
            let key_count = acc[key];
            let (k1, k2) = (vec![key.as_bytes()[0], insert_rules[key]], vec![insert_rules[key], key.as_bytes()[1]]);
            new_pair_counts.entry(String::from_utf8(k1).unwrap()).and_modify(|e| *e += key_count);
            new_pair_counts.entry(String::from_utf8(k2).unwrap()).and_modify(|e| *e += key_count);
            letter_counts.entry(insert_rules[key]).and_modify(|e| *e += key_count).or_insert(0);
        }
        new_pair_counts
    });

    let (max, min) = letter_counts.values().fold((0, u64::MAX), |(max, min), count| (max.max(*count), min.min(*count)));

    println!("Day 14.1: {}", max - min);

    (0..30).fold(pair_counts, |acc, _| {
        let mut new_pair_counts = pair_counts_blank.clone();
        for key in acc.keys() {
            let key_count = acc[key];
            let (k1, k2) = (vec![key.as_bytes()[0], insert_rules[key]], vec![insert_rules[key], key.as_bytes()[1]]);
            new_pair_counts.entry(String::from_utf8(k1).unwrap()).and_modify(|e| *e += key_count);
            new_pair_counts.entry(String::from_utf8(k2).unwrap()).and_modify(|e| *e += key_count);
            letter_counts.entry(insert_rules[key]).and_modify(|e| *e += key_count).or_insert(0);
        }
        new_pair_counts
    });

    let (max, min) = letter_counts.values().fold((0, u64::MAX), |(max, min), count| (max.max(*count), min.min(*count)));

    println!("Day 14.2: {}", max - min);
}