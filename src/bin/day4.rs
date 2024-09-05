fn main() {
    let input: Vec<String> = include_str!("../data/day4.txt").split('-').map(|num| num.trim().to_owned()).collect();

    println!("input: {:?}", input);

    let start = input[0].parse::<u32>().unwrap();
    let end = input[1].parse::<u32>().unwrap();

    let mut valid_numbers: u32 = 0;

    for item in start..end {
        let mut valid = true;
        
        let num = item.to_string();
        let mut i = 0;
        let length = num.chars().clone().count();

        // // two adjacents are the saem
        // let mut adjacent: u8 = 0;
        // while i < length - 1 {
        //     
        //     if num.chars().nth(i) == num.chars().nth(i+1) {
        //         adjacent += 1;
        //     }
        //
        //     i += 1;
        // }
        //
        // if adjacent == 0 {
        //     continue;
        // }
        //
        // i = 0;

        // digits are increasing from left to right
        while i < length - 1 {
            
            if num.chars().nth(i) <= num.chars().nth(i+1) {

            } else {
                valid = false;
            }

            i += 1;
        }

        if !valid {
            continue;
        }
        
        // adjacent at most 2 (444123 not valid, 441234 valid, 441111, also valid because there's atleast one double group)
        let groups = group_identical_letters(&num);
        let groups_of_doubles = groups.iter().filter(|group| group.chars().count() == 2).count();
        if groups_of_doubles == 0 {
            continue;
        }

        valid_numbers += 1;

    }

    println!("valid numbers: {valid_numbers}")
} 

fn group_identical_letters(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();
    let mut group = String::new();

    while let Some(c) = chars.next() {
        group.push(c);
        // Check if the next character is the same as the current one
        if chars.peek() != Some(&c) {
            result.push(group.clone());
            group.clear();
        }
    }

    result
}
