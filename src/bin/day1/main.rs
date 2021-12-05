use std::error::Error;

fn count_3window_increases(values: &[i32]) -> i32 {
    let three_window_sums: Vec<i32> = values
        .iter()
        .zip(values.iter().skip(1))
        .zip(values.iter().skip(2))
        .map(|((i, j), k)| i + j + k)
        .collect();

    count_increases(&three_window_sums[..])
}

fn count_increases(values: &[i32]) -> i32 {
    let mut increases = 0;
    for (i, v) in values.iter().zip(values.iter().skip(1)) {
        if v > i {
            increases += 1;
        }
    }

    increases
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()?;
    let values = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<i32>, _>>()?;

    let part1 = count_increases(&values);
    println!("Part 1: {}", part1);

    let part2 = count_3window_increases(&values);
    println!("Part 2: {}", part2);

    Ok(())
}
