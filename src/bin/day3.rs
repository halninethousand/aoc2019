use std::collections::{HashMap, HashSet};

fn main() {
    let wire1: Vec<&str> = include_str!("../data/day3.txt").lines().next().unwrap().split(',').collect();
    let wire2: Vec<&str> = include_str!("../data/day3.txt").lines().nth(1).unwrap().split(',').collect();

    let mut traversed_1 = HashSet::new();
    let mut traversed_2 = HashSet::new();

    let mut current_x = 0;
    let mut current_y = 0;
    for dir in &wire1 {
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
    for dir in &wire2 {
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

    let (trace1, _) = trace_wire(&wire1);
    let (trace2, _) = trace_wire(&wire2);

    let intersections: HashSet<_> = trace1.keys().collect::<HashSet<_>>().intersection(&trace2.keys().collect::<HashSet<_>>()).cloned().collect();
    
    let mut min_steps = i32::MAX;
    for point in intersections {
        if *point != (0, 0) {
            let total_steps = trace1[point] + trace2[point];
            if total_steps < min_steps {
                min_steps = total_steps;
            }
        }
    }

    if min_steps == i32::MAX {
        println!("No intersections found");
    } else {
        println!("Fewest combined steps to an intersection: {}", min_steps);
    }
}

fn trace_wire(wire: &[&str]) -> (HashMap<(i32, i32), i32>, (i32, i32)) {
    let mut trace = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;

    for dir in wire {
        let (dx, dy) = match &dir[0..1] {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Invalid direction: {}", dir),
        };
        let distance = dir[1..].parse::<i32>().unwrap();

        for _ in 0..distance {
            x += dx;
            y += dy;
            steps += 1;
            trace.entry((x, y)).or_insert(steps);
        }
    }

    (trace, (x, y))
}
