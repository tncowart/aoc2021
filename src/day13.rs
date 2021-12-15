use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day13() {
    let (coords, folds) = include_str!("../resources/day13.txt").split_once("\n\n").unwrap();
    let mut paper = coords.lines().map(|l| {
        let (x, y) = l.split_once(",").unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }).collect::<Vec<(usize, usize)>>();

    folds.lines().take(1).for_each(|l| {
        let (axis, coord) = l.split_once("=").unwrap();
        let coord = coord.parse::<usize>().unwrap();
        match axis {
            "fold along x" => {
                for v in paper.iter_mut() {
                    if v.0 > coord {
                        v.0 = coord - (v.0 - coord);
                    }
                }
            },
            "fold along y" => {
                for v in paper.iter_mut() {
                    println!("v1: {}, coord: {}", v.1, coord);
                    if v.1 > coord {
                        v.1 = coord - (v.1 - coord);
                    }
                }
            },
            _ => unreachable!()
        };
    });
    println!("Day 13.1: {}", HashSet::<&(usize, usize)>::from_iter(&paper).len());

    folds.lines().skip(1).for_each(|l| {
        let (axis, coord) = l.split_once("=").unwrap();
        let coord = coord.parse::<usize>().unwrap();
        match axis {
            "fold along x" => {
                for v in paper.iter_mut() {
                    if v.0 > coord {
                        v.0 = coord - (v.0 - coord);
                    }
                }
            },
            "fold along y" => {
                for v in paper.iter_mut() {
                    if v.1 > coord {
                        v.1 = coord - (v.1 - coord);
                    }
                }
            },
            _ => unreachable!()
        };
    });

    let mut print_paper = [["."; 12]; 80];
    for (x, y) in paper {
        print_paper[x][y] = "#";
    }

    for y in 0..12 {
        for x in print_paper {
            print!("{}", x[y]);
        }
        println!()
    }
    println!();
}