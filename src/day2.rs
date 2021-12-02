use crate::utils::*;

fn _day2() -> (i32, i32) {
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

    (aim * horiz, vert * horiz)
}

pub fn day2() {
    let x = _day2();
    println!("Day 02.1: {}", x.0);
    println!("Day 02.2: {}", x.1);
}
