use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()?;
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.sort();
    let (gamma, epsilon) = calc_gamma_epsilon(&lines)?;

    println!("Part 1: {}", gamma * epsilon);

    let oxygen_gen_rating = calc_life_support_rating(&lines, true);
    let co2_scrbber_rating = calc_life_support_rating(&lines, false);

    println!("Part 2: {}", oxygen_gen_rating * co2_scrbber_rating);

    Ok(())
}

fn calc_life_support_rating(sorted_lines: &[&str], most_common: bool) -> u32 {
    let mut candidates = &sorted_lines[..];
    let mut pos = 0;
    while candidates.len() != 1 {
        let majority_count = candidates.len() / 2;
        let num_zeros =
            candidates.partition_point(|l| l.chars().nth(pos).expect("No match found!") == '0');

        let (majority, minority) = if most_common {
            (&candidates[0..num_zeros], &candidates[num_zeros..])
        } else {
            (&candidates[num_zeros..], &candidates[0..num_zeros])
        };

        if num_zeros > majority_count {
            candidates = majority;
        } else {
            candidates = minority;
        }

        pos += 1;
    }

    u32::from_str_radix(candidates[0], 2).unwrap()
}

fn calc_gamma_epsilon(lines: &[&str]) -> Result<(u32, u32), Box<dyn Error>> {
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

    let majority_count = (lines.len() / 2) as u32;
    let gamma_rate: u32 = bit_counts.iter().fold(0, |acc, count| {
        let bit = if *count as u32 > majority_count { 1 } else { 0 };
        (acc << 1) + bit
    });

    let epsilon_rate = ((!gamma_rate) << (32 - binary_len)) >> (32 - binary_len);

    Ok((gamma_rate, epsilon_rate))
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
    fn test_calc_gamma_epsilon() {
        let lines = SAMPLE_INPUT.lines().collect::<Vec<_>>();
        let (gamma, epsilon) = calc_gamma_epsilon(&lines).unwrap();
        assert_eq!(gamma, 0b10110);
        assert_eq!(epsilon, 0b01001);
    }

    #[test]
    fn test_calc_life_support_rating() {
        let mut lines = SAMPLE_INPUT.lines().collect::<Vec<_>>();
        lines.sort();
        let oxygen_rating = calc_life_support_rating(&lines, true);
        assert_eq!(oxygen_rating, 0b10111);

        let co2_scrubber_rating = calc_life_support_rating(&lines, false);
        assert_eq!(co2_scrubber_rating, 0b01010);
    }
}
