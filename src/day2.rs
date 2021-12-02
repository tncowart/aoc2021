use crate::utils::*;

fn day2() -> i32 {
    let mut vert = 0;
    let mut horiz = 0;

    for line in lines("resources/day2.txt") {
        let line = line.unwrap();
        let vec: Vec<&str> = line.as_str().split_whitespace().collect();
        let direction = vec[0];
        let dist = vec[1].trim().parse::<i32>().unwrap();

        match direction {
            "up" => vert -= dist,
            "down" => vert += dist,
            "forward" => horiz += dist,
            _ => {}
        };
    }

    vert * horiz
}

fn day2_2_2() -> i32 {
    let mut vert = 0;
    let mut horiz = 0;
    let mut aim = 0;

    for line in lines("resources/day2.txt") {
        let line = line.unwrap();
        let vec: Vec<&str> = line.as_str().split_whitespace().collect();
        let direction = vec[0];
        let dist = vec[1].trim().parse::<i32>().unwrap();

        match direction {
            "up" => aim -= dist,
            "down" => aim += dist,
            "forward" => {
                horiz += dist;
                vert += aim * dist;
            }
            _ => {}
        };
    }

    vert * horiz
}

pub fn day2_1() {
    println!("Day 02.1: {}", day2());
}

pub fn day2_2() {
    println!("Day 02.2: {}", day2_2_2());
}
