
fn main() {
    let mut input: Vec<usize> = include_str!("../data/day2.txt").trim_end().split(',').map(|num| num.parse::<usize>().unwrap()).collect();

    let len = input.len();

    let i = 0;

    while i < len {
        let peek = input[i];

        if peek == 3 {
            input[input[i+1]] = 1;
            i += 2;
            continue;
        } else if peek == 4 {
            println!(input[input[i+1]]);
            i += 2;
            continue;
        } else {
            let operation = input[i];
            match operation.to_string().chars().count() {
                2 => 
                _ => println!("putka");

            }
            let operand1_idx = input[i + 1];
            let operand2_idx = input[i + 2];
            let target_idx = input[i + 3];

            match operation {
                99 => break,
                1 => {
                    let add = input[operand1_idx] + input[operand2_idx];
                    input[target_idx] = add;
                    i += 4; // Move to the next chunk
                }
                2 => {
                    let multiply = input[operand1_idx] * input[operand2_idx];
                    input[target_idx] = multiply;
                    i += 4; // Move to the next chunk
                }

                _ => println!("unknown pattern"),
            }

            i+=4;

        }
    }

    println!("number at position 0: {:?}", input[0]);

}
