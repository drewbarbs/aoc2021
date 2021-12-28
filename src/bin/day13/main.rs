use std::collections::HashSet;
use std::error::Error;

type Dots = HashSet<(i32, i32)>;

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    value: i32,
}

fn parse_input(input: &str) -> (Dots, Vec<Fold>) {
    let mut lines = input.lines();

    let dots = lines
        .by_ref()
        .take_while(|l| l.trim().len() > 0)
        .map(|l| {
            let mut components = l.split(',');
            (
                components.next().unwrap().parse::<i32>().unwrap(),
                components.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<HashSet<_>>();

    // Discard blank line
    lines.next();

    let folds = lines
        .map(|l| {
            let mut split = l.split('=');
            let axis = match split.next().unwrap().chars().last().unwrap() {
                'x' => Axis::X,
                'y' => Axis::Y,
                _ => panic!("Unrecognized axis"),
            };
            let value = split.next().unwrap().parse().unwrap();

            Fold { axis, value }
        })
        .collect::<Vec<_>>();

    (dots, folds)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()?;

    let (dots, folds) = parse_input(&input);

    println!("{:?} {:?}", dots, folds);

    Ok(())
}
