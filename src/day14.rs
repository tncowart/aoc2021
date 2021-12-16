use std::collections::HashMap;
pub fn day14() {
    let (poly_template, insert_rules_string) = include_str!("../resources/day14.txt").split_once("\n\n").unwrap();
    let insert_rules: HashMap<&[u8], [u8; 2]> = insert_rules_string.lines().fold(HashMap::new(), |mut acc, l| {
        let (key, val) = l.split_once(" -> ").unwrap();
        let key = key.as_bytes();
        let val = val.as_bytes();
        acc.insert(key, [key[0], val[0]]);
        acc
    });
    // let pair_counts: HashMap<&[u8], u64> = insert_rules_string.lines().fold(HashMap::new(), |mut acc, l| {
    //     let (key, _) = l.split_once(" -> ").unwrap();
    //     let key = key.as_bytes();
    //     acc.insert(key, 0);
    //     acc
    // });
    // println!("{}", poly_template);
    let mut poly = (0..10).fold(Vec::<u8>::from(poly_template.trim().as_bytes()), |acc, _| {
        let mut new_template = Vec::<u8>::with_capacity((acc.len() << 1) - 1);
        for i in 0..(acc.len() - 1) {
            new_template.extend(insert_rules[&acc[i..=i+1]]);
        }
        new_template.push(*acc.last().unwrap());
        new_template
    });
    // println!("{:?}", std::str::from_utf8(&poly_template));
    poly.sort_unstable();
    let (max, min) = poly.group_by(|a, b| a == b).fold((0, usize::MAX), |(max, min), group| (max.max(group.len()), min.min(group.len())));

    println!("Day 14.1: {}", max - min);

    let mut poly = (0..40).fold(Vec::<u8>::from(poly_template.trim().as_bytes()), |acc, _| {
        let mut new_template = Vec::<u8>::with_capacity((acc.len() << 1) - 1);
        for i in 0..(acc.len() - 1) {
            new_template.extend(insert_rules[&acc[i..=i+1]]);
        }
        new_template.push(*acc.last().unwrap());
        new_template
    });

    poly.sort_unstable();
    let (max, min) = poly.group_by(|a, b| a == b).fold((0, usize::MAX), |(max, min), group| (max.max(group.len()), min.min(group.len())));
    println!("Day 14.2: {}", max - min);
}