fn launch(x1: i32, x2: i32, y1: i32, y2: i32) -> (i32, i32, i32) {
    let mut max_y = i32::MIN;
    let mut max_y_x = 0;
    let mut total = 0;
    for x in 0..=(x2 + 1) {
        for y in y2..1000 {
            let mut curr_x = x;
            let mut curr_y = y;
            let mut pos_x = 0;
            let mut pos_y = 0;
            while pos_x <= x2 && pos_y >= y2 {
                if pos_x >= x1 && pos_x <= x2 && pos_y <= y1 && pos_y >= y2 {
                    if y > max_y {
                        max_y = y;
                        max_y_x = x;
                    }
                    total += 1;
                    break;
                }
                pos_x += curr_x;
                pos_y += curr_y;
                curr_x = 0.max(curr_x - 1);
                curr_y -= 1;
            }
        }
    }
    (max_y_x, max_y, total)
}

pub fn day17() {
    // target area: x=34..67, y=-215..-186
    let (_, y, total) = launch(34, 67, -186, -215);
    println!("Day 17.1: {}", (y*(y+1)) / 2);
    println!("Day 17.2: {}", total);
}