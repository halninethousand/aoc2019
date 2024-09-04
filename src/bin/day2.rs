fn main() {
    let mut input: Vec<usize> = include_str!("../data/day2.txt").trim_end().split(',').map(|num| num.parse::<usize>().unwrap()).collect();
    let input_2 = input.clone();
    input[1] = 12;
    input[2] = 2;
    let target_output = 19690720;

    let len = input.len();

    let mut i = 0;
    while i < len {
        let operation = input[i];
        let operand1_idx = input[i + 1];
        let operand2_idx = input[i + 2];
        let target_idx = input[i + 3];

        match operation {
            99 => break,
            1 => {
                let add = input[operand1_idx] + input[operand2_idx];
                input[target_idx] = add;
            }
            2 => {
                let multiply = input[operand1_idx] * input[operand2_idx];
                input[target_idx] = multiply;
            }
            _ => println!("unknown pattern"),
        }


        i += 4; // Move to the next chunk
    }

    println!("number at position 0: {:?}", input[0]);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut input = input_2.clone();
            input[1] = noun;
            input[2] = verb;

            let mut i = 0;
            while i < input.len() {
                let operation = input[i];
                let operand1_idx = input[i + 1];
                let operand2_idx = input[i + 2];
                let target_idx = input[i + 3];

                match operation {
                    99 => break,
                    1 => {
                        let add = input[operand1_idx] + input[operand2_idx];
                        input[target_idx] = add;
                    }
                    2 => {
                        let multiply = input[operand1_idx] * input[operand2_idx];
                        input[target_idx] = multiply;
                    }
                    _ => println!("unknown pattern"),
                }

                i += 4;
            }

            if input[0] == target_output {
                println!("Found: noun = {}, verb = {}", noun, verb);
                println!("Result: {}", 100 * noun + verb);
                return;
            }
        }
    }

}
