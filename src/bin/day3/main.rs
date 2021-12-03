use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Err(format!(
            "Usage: {} <input file>",
            args.get(0).unwrap_or(&"prog".into())
        ))?;
    }

    let input = fs::read_to_string(&args[1])?;
    let lines = input.lines().collect::<Vec<_>>();
    let bit_counts = get_bit_counts(&lines)?;
    let (gamma, epsilon) = calc_gamma_epsilon(lines.len(), &bit_counts);

    println!("Part 1: {}", gamma * epsilon);

    let oxygen_gen_rating = calc_oxygen_generator_rating(&lines);
    let co2_scrbber_rating = calc_co2_scrubber_rating(&lines);

    println!("Part 2: {}", oxygen_gen_rating * co2_scrbber_rating);

    Ok(())
}

fn calc_oxygen_generator_rating(lines: &[&str]) -> u32 {
    let mut lines = lines.iter().collect::<Vec<_>>();
    lines.sort();

    let mut candidates = &lines[..];
    let mut pos = 0;
    while candidates.len() != 1 {
        let majority_count = candidates.len() / 2;
        let num_zeros =
            candidates.partition_point(|l| l.chars().nth(pos).expect("No match found!") == '0');

        if num_zeros > majority_count {
            candidates = &candidates[0..num_zeros];
        } else {
            candidates = &candidates[num_zeros..];
        }

        pos += 1;
    }

    u32::from_str_radix(candidates[0], 2).unwrap()
}

fn calc_co2_scrubber_rating(lines: &[&str]) -> u32 {
    let mut lines = lines.iter().collect::<Vec<_>>();
    lines.sort();

    let mut candidates = &lines[..];
    let mut pos = 0;
    while candidates.len() != 1 {
        let majority_count = candidates.len() / 2;
        let num_zeros =
            candidates.partition_point(|l| l.chars().nth(pos).expect("No match found!") == '0');

        if num_zeros > majority_count {
            candidates = &candidates[num_zeros..];
        } else {
            candidates = &candidates[0..num_zeros];
        }

        pos += 1;
    }

    u32::from_str_radix(candidates[0], 2).unwrap()
}

fn get_bit_counts(lines: &[&str]) -> Result<Vec<usize>, Box<dyn Error>> {
    let binary_len = lines.get(0).ok_or("Empty input")?.len();
    let mut bit_counts: Vec<usize> = vec![0; binary_len];

    for l in lines {
        if l.len() > binary_len {
            return Err("Uneven line lengths!".into());
        }

        for (i, b) in l.chars().enumerate() {
            match b {
                '1' => bit_counts[i] += 1,
                '0' => (),
                _ => return Err("Invalid character".into()),
            }
        }
    }

    Ok(bit_counts)
}

fn calc_gamma_epsilon(n_lines: usize, bit_counts: &Vec<usize>) -> (u32, u32) {
    let binary_len = bit_counts.len();
    let majority_count = (n_lines / 2) as u32;
    let gamma_rate: u32 = bit_counts.iter().fold(0, |acc, count| {
        let bit = if *count as u32 > majority_count { 1 } else { 0 };
        (acc << 1) + bit
    });

    let epsilon_rate = ((!gamma_rate) << (32 - binary_len)) >> (32 - binary_len);

    (gamma_rate, epsilon_rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_get_bit_counts() {
        let expected_bit_counts = vec![7, 5, 8, 7, 5];
        let lines = SAMPLE_INPUT.lines().collect::<Vec<_>>();
        let bit_counts = get_bit_counts(&lines).ok().unwrap();
        assert_eq!(expected_bit_counts, bit_counts);
    }

    #[test]
    fn test_calc_gamma_epsilon() {
        let lines = SAMPLE_INPUT.lines().collect::<Vec<_>>();
        let bit_counts = get_bit_counts(&lines).ok().unwrap();
        let (gamma, epsilon) = calc_gamma_epsilon(lines.len(), &bit_counts);
        assert_eq!(gamma, 0b10110);
        assert_eq!(epsilon, 0b01001);
    }

    #[test]
    fn test_calc_oxygen_generator_rating() {
        let lines = SAMPLE_INPUT.lines().collect::<Vec<_>>();
        let gen_rating = calc_oxygen_generator_rating(&lines);
        assert_eq!(gen_rating, 0b10111);
    }

    #[test]
    fn test_co2_scrubber_rating() {
        let lines = SAMPLE_INPUT.lines().collect::<Vec<_>>();
        let rating = calc_co2_scrubber_rating(&lines);
        assert_eq!(rating, 0b01010);
    }
}
