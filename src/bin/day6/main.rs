use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()?;
    let initial_counts = parse_counts(&input.trim())?;

    let population_after_80_days = advance_n_days(initial_counts.clone(), 80usize);
    let total_pop: usize = population_after_80_days.iter().sum();
    println!("Part 1: {}", total_pop);

    let population_after_256_days = advance_n_days(initial_counts.clone(), 256usize);
    let total_pop: usize = population_after_256_days.iter().sum();
    println!("Part 2: {}", total_pop);

    Ok(())
}

fn parse_counts(line: &str) -> Result<[usize; 9], Box<dyn Error>> {
    let mut result = [0usize; 9];
    for i in line.split(',') {
        let idx = i.parse::<usize>()?;
        result[idx] += 1;
    }

    Ok(result)
}

fn advance_n_days(mut population: [usize; 9], days: usize) -> [usize; 9] {
    for _ in 0..(days / 7usize) {
        population = advance_7_days(population);
    }

    for _ in 0..(days % 7usize) {
        population = advance_1_day(population);
    }

    population
}

fn advance_1_day(population: [usize; 9]) -> [usize; 9] {
    let mut new_pop = population.clone();
    new_pop[7] = 0;
    new_pop[8] = 0;

    (&mut new_pop[..7]).rotate_left(1);
    new_pop[8] += population[0];
    new_pop[7] += population[8];
    new_pop[6] += population[7];

    new_pop
}

fn advance_7_days(population: [usize; 9]) -> [usize; 9] {
    let mut new_pop = population.clone();
    new_pop[7] = 0;
    new_pop[8] = 0;

    for i in 0i32..9i32 {
        new_pop[(i - 7).rem_euclid(9) as usize] += population[i as usize];
    }

    new_pop
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_VALUES: &str = include_str!("sample.txt");

    #[test]
    fn test_parse_counts() {
        let lines = SAMPLE_VALUES.lines().collect::<Vec<_>>();
        let result = parse_counts(lines[0]).unwrap();

        let expected: [usize; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_advance_7_days() {
        let lines = SAMPLE_VALUES.lines().collect::<Vec<_>>();
        let initial_state = parse_counts(lines[11]).unwrap();
        let expected = parse_counts(lines[18]).unwrap();

        assert_eq!(expected, advance_7_days(initial_state));
    }

    #[test]
    fn test_advance_1_day() {
        let lines = SAMPLE_VALUES.lines().collect::<Vec<_>>();
        let initial_state = parse_counts(lines[15]).unwrap();
        let expected = parse_counts(lines[16]).unwrap();

        assert_eq!(expected, advance_1_day(initial_state));
    }

    #[test]
    fn test_advance_n_days() {
        let lines = SAMPLE_VALUES.lines().collect::<Vec<_>>();
        let initial_state = parse_counts(lines[0]).unwrap();
        let expected = parse_counts(lines[18]).unwrap();

        assert_eq!(expected, advance_n_days(initial_state, 18usize));
    }
}
