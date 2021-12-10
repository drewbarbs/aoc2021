fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()
        .lines()
        .filter(|l| l.strip().len() > 0)
        .collect::<Vec<_>>();
}
