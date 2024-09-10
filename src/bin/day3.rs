use std::collections::HashSet;

fn main() {
    let wire1: Vec<&str> = include_str!("../data/day3.txt").lines().next().unwrap().split(',').collect();
    let wire2: Vec<&str> = include_str!("../data/day3.txt").lines().nth(1).unwrap().split(',').collect();

    let mut traversed_1 = HashSet::new();
    let mut traversed_2 = HashSet::new();

    let mut current_x = 0;
    let mut current_y = 0;
    for dir in wire1 {
        match &dir[0..1] {
            "R" => {
                let steps = dir[1..].parse::<i32>().unwrap();
                for i in 0..steps {
                    traversed_1.insert((current_x + i, current_y));
                }
                current_x += steps;
            }
            "L" => {
                let steps = dir[1..].parse::<i32>().unwrap();
                for i in 0..steps {
                    traversed_1.insert((current_x - i, current_y));
                }
                current_x -= steps;
            }
            "U" => {
                let steps = dir[1..].parse::<i32>().unwrap();
                for i in 0..steps {
                    traversed_1.insert((current_x, current_y + i));
                }
                current_y += steps;
            }
            "D" => {
                let steps = dir[1..].parse::<i32>().unwrap();
                for i in 0..steps {
                    traversed_1.insert((current_x, current_y - i));
                }
                current_y -= steps;
            }
            _ => panic!("Invalid direction: {}", dir),
        }
    }

    let mut current_x = 0;
    let mut current_y = 0;
    for dir in wire2 {
        match &dir[0..1] {
            "R" => {
                let steps = dir[1..].parse::<i32>().unwrap();
                for i in 0..steps {
                    traversed_2.insert((current_x + i, current_y));
                }
                current_x += steps;
            }
            "L" => {
                let steps = dir[1..].parse::<i32>().unwrap();
                for i in 0..steps {
                    traversed_2.insert((current_x - i, current_y));
                }
                current_x -= steps;
            }
            "U" => {
                let steps = dir[1..].parse::<i32>().unwrap();
                for i in 0..steps {
                    traversed_2.insert((current_x, current_y + i));
                }
                current_y += steps;
            }
            "D" => {
                let steps = dir[1..].parse::<i32>().unwrap();
                for i in 0..steps {
                    traversed_2.insert((current_x, current_y - i));
                }
                current_y -= steps;
            }
            _ => panic!("Invalid direction: {}", dir),
        }
    }

    let intersections: HashSet<_> = traversed_1.intersection(&traversed_2).cloned().collect();

    let mut min_distance = i32::MAX;

    for (x, y) in intersections {
        if x != 0 || y != 0 {
            let distance = x.abs() + y.abs();
            if distance < min_distance {
                min_distance = distance;
            }
        }
    }

    if min_distance == i32::MAX {
        println!("No intersections found");
    } else {
        println!("Closest intersection distance: {}", min_distance);
    }
}
