fn main() {
    let input: Vec<String> = include_str!("../data/day3_short.txt").lines().map(|text| text.to_owned()).collect();
    println!("{input:?}");

    let size = 50;
    let mut current = size/2;
    let mut grid: Vec<Vec<u32>> = vec![vec![0; size]; size];
    
    for item in input {
        let directions: Vec<String> = item.split(',').map(|dir| dir.to_owned()).collect();
        for dir in directions {
            match dir.chars().next() {
                Some('R') =>  {
                    let amount: u32 = dir[1..].parse::<u32>().unwrap();
                    println!("amount: {}", amount);
                },
                Some('U') => {
                    println!("{:?}", &dir[1..]);
                },
                Some('L') => {
                    println!("{:?}", &dir[1..]);
                },
                Some('D') => {
                    println!("{:?}", &dir[1..]);
                },
                _ => panic!("what is this?"),
            }
        }
    }    

}

fn increment_row_left(matrix: &mut Vec<Vec<u32>>, row: usize, start_col: usize, steps: usize) {
    let mut col = start_col;

    while col >= steps.min(start_col + 1) {
        matrix[row][col] += 1;
        if col == 0 {
            break; // Prevent underflow of usize when col is 0
        }
        col -= 1; // Move left
    }
}

fn increment_row_right(matrix: &mut Vec<Vec<u32>>, row: usize, start_col: usize, steps: usize) {
    let mut col = start_col;

    while col < matrix[row].len() && (col - start_col) < steps {
        matrix[row][col] += 1;
        col += 1; // Move right
    }
}

fn increment_column_down(matrix: &mut Vec<Vec<u32>>, start_row: usize, column: usize, steps: usize) {
    let mut row = start_row;

    while row < matrix.len() && (row - start_row) < steps {
        if column < matrix[row].len() {
            matrix[row][column] += 1;
        }
        row += 1; // Move down
    }
}

fn increment_column_up(matrix: &mut Vec<Vec<u32>>, start_row: usize, column: usize, steps: usize) {
    let mut row = start_row;

    while row >= steps.min(start_row + 1) {
        if column < matrix[row].len() {
            matrix[row][column] += 1;
        }
        if row == 0 {
            break; // Prevent underflow of usize when row is 0
        }
        row -= 1; // Move up
    }
}
