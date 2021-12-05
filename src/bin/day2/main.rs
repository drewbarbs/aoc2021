use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

use Command::*;

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components = s.trim().split_ascii_whitespace().collect::<Vec<_>>();
        if components.len() != 2 {
            Err("Improper command format")?;
        }

        let num_units = components[1].parse::<i32>()?;
        match components[0] {
            "forward" => Ok(Forward(num_units)),
            "up" => Ok(Up(num_units)),
            "down" => Ok(Down(num_units)),
            _ => Err(format!("Unrecognized direction: {}", components[0]).into()),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Command>, Box<dyn Error>> {
    input.lines().map(|l| l.parse::<Command>()).collect()
}

fn part1(commands: &[Command]) -> (i32, i32) {
    commands
        .iter()
        .fold((0, 0), |(depth, horiz_pos), c| match c {
            Up(i) => (depth - i, horiz_pos),
            Down(i) => (depth + i, horiz_pos),
            Forward(i) => (depth, horiz_pos + i),
        })
}

fn part2(commands: &[Command]) -> (i32, i32) {
    let (depth, horiz_pos, _aim) = commands
        .iter()
        .fold((0, 0, 0), |(depth, horiz_pos, aim), c| match c {
            Up(i) => (depth, horiz_pos, aim - i),
            Down(i) => (depth, horiz_pos, aim + i),
            Forward(i) => (depth + aim * i, horiz_pos + i, aim),
        });
    (depth, horiz_pos)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()?;
    let commands = parse_input(&input)?;

    let part1 = part1(commands.as_slice());
    println!("Part 1: {}", part1.0 * part1.1);

    let part2 = part2(commands.as_slice());
    println!("Part 1: {}", part2.0 * part2.1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_COMMANDS: &'static [Command] =
        &[Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];

    #[test]
    fn test_input_parse() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";
        let parsed_commands = parse_input(input);
        assert!(parsed_commands.is_ok());
        assert_eq!(TEST_COMMANDS, parsed_commands.unwrap());
    }

    #[test]
    fn test_part1() {
        let (depth, horiz_pos) = part1(TEST_COMMANDS);
        assert_eq!((depth, horiz_pos), (10, 15));
    }

    #[test]
    fn test_part2() {
        let (depth, horiz_pos) = part2(TEST_COMMANDS);
        assert_eq!((depth, horiz_pos), (60, 15));
    }
}
