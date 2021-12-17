use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _input = aoc2021::get_input_string()?
        .lines()
        .filter(|l| l.trim().len() > 0)
        .collect::<Vec<_>>();
    Ok(())
}
