fn main() {
    let input: Vec<f32> = include_str!("../data/day1.txt").lines().map(|num| num.parse::<f32>().unwrap()).collect();
    let sum: Vec<f32> = input.iter()
        .map(|num| (num/3.0).floor() - 2.0)
        .collect();

    let mut recursive_sum: f32 = 0.0;

    for num in input {
        let mut sum: Vec<f32> = vec![];
        let mut fuel = (num/3.0).floor() - 2.0;

        sum.push(fuel);

        while fuel >= 1.0 {
            fuel = (fuel/3.0).floor() - 2.0;
            if fuel >= 1.0 {
                sum.push(fuel);
            }
        }

        recursive_sum += sum.iter().sum::<f32>();
    }

    println!("sum: {:?}", sum.iter().sum::<f32>());
    println!("recursive sum: {:?}", recursive_sum);
}
