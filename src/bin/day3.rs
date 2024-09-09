fn main() {
    let input: Vec<String> = include_str!("../data/day3.txt").lines().map(|text| text.to_owned()).collect();
    println!("{input:?}");

    let size = 50000;
    let mut grid: Vec<Vec<u32>> = vec![vec![0; size]; size];
    
    let start: [usize; 2]  = [size/2, size/2];
    for item in input {
        let mut current: [usize; 2]  = [size/2, size/2];
        let directions: Vec<String> = item.split(',').map(|dir| dir.to_owned()).collect();
        for dir in directions {
            println!("current: {current:?}, dir: {dir:?}");
            match dir.chars().next() {
                Some('R') =>  {
                    let steps = dir[1..].parse::<usize>().unwrap();
                    increment_right(&mut grid, &mut current, steps);
                },
                Some('U') => {
                    let steps = dir[1..].parse::<usize>().unwrap();
                    increment_up(&mut grid, &mut current, steps);
                },
                Some('L') => {
                    let steps = dir[1..].parse::<usize>().unwrap();
                    increment_left(&mut grid, &mut current, steps);
                },
                Some('D') => {
                    let steps = dir[1..].parse::<usize>().unwrap();
                    increment_down(&mut grid, &mut current, steps);
                },
                _ => panic!("what is this?"),
            }
        }
    }    

    // print_grid(&grid);
    let result = find_items_gte_2_with_indices(&grid);
    println!("Items greater than or equal to 2 with indices:");
    let mut smallest_taxicab: isize = 10000;
    for (y, x) in result {
        println!("y: {}, x: {}", y, x);
        let taxicab = taxicab_distance(x, y, start);
        if taxicab < smallest_taxicab {
            smallest_taxicab = taxicab;
        }
        println!("taxicab distance: {} for x: {}, y: {}", taxicab, x, y);
    }
    println!("smallest taxicab distance: {}", smallest_taxicab);

}

fn taxicab_distance(x: usize, y: usize, start: [usize; 2]) -> isize {
    (x as isize - start[1] as isize).abs() + (y as isize - start[0] as isize).abs()
}

fn increment_up(matrix: &mut [Vec<u32>], current: &mut [usize; 2], steps: usize) {
    let start_row = current[0];
    let col = current[1];
    let end_row = start_row.saturating_sub(steps);
    
    for row in (end_row..start_row).rev() {
        if col < matrix[row].len() {
            matrix[row][col] += 1;
        }
    }
    current[0] = end_row;
}

fn increment_down(matrix: &mut [Vec<u32>], current: &mut [usize; 2], steps: usize) {
    let start_row = current[0];
    let col = current[1];
    let end_row = (start_row + steps + 1).min(matrix.len());
    
    for row in (start_row + 1)..end_row {
        if col < matrix[row].len() {
            matrix[row][col] += 1;
        }
    }
    current[0] = end_row - 1;
}

fn increment_left(matrix: &mut [Vec<u32>], current: &mut [usize; 2], steps: usize) {
    let row = current[0];
    let start_col = current[1];
    let end_col = start_col.saturating_sub(steps);
    
    if row < matrix.len() {
        for col in end_col..start_col {
            matrix[row][col] += 1;
        }
    }
    current[1] = end_col;
}

fn increment_right(matrix: &mut [Vec<u32>], current: &mut [usize; 2], steps: usize) {
    let row = current[0];
    let start_col = current[1];
    
    if row < matrix.len() {
        let end_col = (start_col + steps + 1).min(matrix[row].len());
        
        for col in (start_col + 1)..end_col {
            matrix[row][col] += 1;
        }
        current[1] = end_col - 1;
    }
}

fn find_items_gte_2_with_indices(nested_vec: &[Vec<u32>]) -> Vec<(usize, usize)> {
    nested_vec
        .iter()
        .enumerate()
        .flat_map(|(outer_idx, inner_vec)| {
            inner_vec
                .iter()
                .enumerate()
                .filter(|&(_, &num)| num >= 2)
                .map(move |(inner_idx, _)| (outer_idx, inner_idx))
        })
        .collect()
}


fn print_grid(matrix: &[Vec<u32>]) {
    for row in matrix {
        for &num in row {
            print!("{}", num);
        }
        println!();
    }
}
