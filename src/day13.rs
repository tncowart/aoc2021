use std::collections::HashSet;
use std::iter::FromIterator;

fn fold_directions(l: &str) -> (&str, usize) {
    let (axis, coord) = l.split_once("=").unwrap();
    (axis, coord.parse::<usize>().unwrap())
}

fn fold(paper: &mut Vec<(usize, usize)>, axis: &str, coord: usize) {
    match axis {
        "fold along x" => {
            for v in paper.iter_mut().filter(|(x, _)| *x > coord) {
                v.0 = (coord << 1) - v.0;
            }
        },
        "fold along y" => {
            for v in paper.iter_mut().filter(|(_, y)| *y > coord) {
                v.1 = (coord << 1) - v.1;
            }
        },
        _ => unreachable!()
    };
}

fn print(paper: &[(usize, usize)]) {
    let mut print_paper = [["."; 6]; 40];
    for (x, y) in paper {
        print_paper[*x][*y] = "#";
    }

    for y in 0..6 {
        for x in print_paper {
            print!("{}", x[y]);
        }
        println!()
    }
    println!();
}

pub fn day13() {
    let (coords, folds) = include_str!("../resources/day13.txt").split_once("\n\n").unwrap();
    let mut paper = coords.lines().map(|l| {
        let (x, y) = l.split_once(",").unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }).collect::<Vec<(usize, usize)>>();

    folds.lines().take(1).map(fold_directions).for_each(|(axis, coord)| {
        fold(&mut paper, axis, coord)
    });
    println!("Day 13.1: {}", HashSet::<&(usize, usize)>::from_iter(&paper).len());

    folds.lines().skip(1).map(fold_directions).for_each(|(axis, coord)| {
        fold(&mut paper, axis, coord)
    });
    println!("Day 13.2:");
    print(&paper);
}