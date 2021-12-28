use std::error::Error;

fn parse_input(s: &str) -> Vec<Vec<char>> {
    s.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>()
}

#[allow(dead_code)]
fn to_str(grid: &Vec<Vec<char>>) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn run_iteration(src: &Vec<Vec<char>>, dst: &mut Vec<Vec<char>>, tmp: &mut Vec<Vec<char>>) {
    let num_rows = src.len();
    let row_len = src[0].len();

    for (i, row) in src.iter().enumerate() {
        (&mut tmp[i]).copy_from_slice(row.as_slice());
        (&mut dst[i]).copy_from_slice(row.as_slice());
    }

    // eastbound
    for (i, row) in src.iter().enumerate() {
        for (j, cucumber) in row.iter().enumerate() {
            if *cucumber == '>' && src[i][(j + 1) % row_len] == '.' {
                dst[i][j] = '.';
                dst[i][(j + 1) % row_len] = '>';
                tmp[i][j] = '.';
                tmp[i][(j + 1) % row_len] = '>';
            }
        }
    }

    // southbound
    for (i, row) in tmp.iter().enumerate() {
        for (j, cucumber) in row.iter().enumerate() {
            if *cucumber == 'v' && tmp[(i + 1) % num_rows][j] == '.' {
                dst[i][j] = '.';
                dst[(i + 1) % num_rows][j] = 'v';
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_ = aoc2021::get_input_string()?;

    let mut initial_grid = parse_input(&input_);
    let mut other_grid = initial_grid.clone();
    let mut tmp = initial_grid.clone();

    for i in 1.. {
        let (src, dst) = if i % 2 == 0 {
            (&mut initial_grid, &mut other_grid)
        } else {
            (&mut other_grid, &mut initial_grid)
        };

        run_iteration(src, dst, &mut tmp);

        if src == dst {
            println!("{}", i);
            break;
        }
    }

    // println!("{}", to_str(&initial_grid));
    // println!("");
    // println!("{}", to_str(&other_grid));

    Ok(())
}
