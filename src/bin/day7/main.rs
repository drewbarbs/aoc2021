use std::error::Error;

fn parse_input(s: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    s.trim()
        .split(',')
        .map(|i| i.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.into())
}

fn get_gte_counts(sorted_positions: &[i32]) -> Vec<i32> {
    let min = sorted_positions[0];
    assert_eq!(min, 0);
    let max = sorted_positions.last().unwrap();

    let mut counts: Vec<i32> = vec![0; (max + 1) as usize];
    let mut count_iter = counts.iter_mut().enumerate();
    let mut count_entry = count_iter.next().unwrap();
    for (i, p) in sorted_positions.iter().enumerate() {
        while (count_entry.0 as i32) <= *p {
            *count_entry.1 = (sorted_positions.len() - i) as i32;
            if let Some(ce) = count_iter.next() {
                count_entry = ce;
                continue;
            } else {
                return counts;
            }
        }
    }

    counts
}

fn part1(mut positions: Vec<i32>) -> i32 {
    positions.sort();
    let gte = get_gte_counts(&positions);
    let lt = gte
        .iter()
        .map(|c| (positions.len() as i32) - *c)
        .collect::<Vec<_>>();

    let mut dists = vec![0; gte.len()];
    dists[0] = positions.iter().filter(|p| **p > 0).sum();

    for i in 1..dists.len() {
        dists[i] = dists[i - 1] + lt[i] - gte[i];
    }

    *dists.iter().min().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()?;
    let mut positions = parse_input(&input)?;

    println!("Part 1: {}", part1(positions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("sample.txt");

    #[test]
    fn test_parse_input() {
        let result = parse_input(SAMPLE_INPUT).unwrap();
        let expected = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_gte_counts() {
        let mut positions = parse_input(SAMPLE_INPUT).unwrap();
        positions.sort();
        let result = get_gte_counts(&positions);
        let expected = vec![10, 9, 7, 4, 4, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 1, 1];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part1() {
        let mut positions = parse_input(SAMPLE_INPUT).unwrap();
        let result = part1(positions);

        assert_eq!(37, result);
    }
}
