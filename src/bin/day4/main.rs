use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Default)]
struct Board {
    col_count: Vec<u32>,
    row_count: Vec<u32>,
    coords: HashMap<u32, (usize, usize)>,
    total_sum: u32,
}

type Drawing = Vec<u32>;

fn parse_input(input: &str) -> Result<(Vec<Board>, Drawing), Box<dyn Error>> {
    let mut lines = input.lines();

    let drawing: Drawing = lines
        .next()
        .ok_or("Empty input")?
        .split(',')
        .map(|i| i.parse::<u32>())
        .collect::<Result<_, _>>()?;

    if lines.next().ok_or("No boards")?.len() != 0 {
        return Err("Expecting a blank line before boards".into());
    }

    let mut boards: Vec<Board> = Vec::new();
    let mut cur_board: Option<Board> = None;
    for line in lines {
        if line.len() == 0 {
            boards.push(cur_board.take().ok_or("Invalid input")?);
            continue;
        }

        if cur_board.is_none() {
            cur_board = Some(Board::default());
        }

        let cur_board: &mut Board = cur_board.as_mut().unwrap();
        cur_board.row_count.resize(cur_board.row_count.len() + 1, 0);
        line.split(' ')
            .filter_map(|i| i.parse::<u32>().ok())
            .scan((cur_board.row_count.len() - 1, 0usize), |(r, c), n| {
                if *r == 0 {
                    cur_board.col_count.resize(*c + 1, 0);
                }
                cur_board.col_count[*c] += 1;
                cur_board.row_count[*r] += 1;
                cur_board.coords.insert(n, (*r, *c));
                cur_board.total_sum += n;

                *c += 1;

                Some(())
            })
            .for_each(drop);
    }
    if let Some(last_board) = cur_board {
        boards.push(last_board)
    }

    Ok((boards, drawing))
}

fn run_game(mut boards: Vec<Board>, drawing: Drawing) -> Option<u32> {
    for n in drawing {
        for board in boards.iter_mut() {
            if let Some((r, c)) = board.coords.get(&n) {
                board.total_sum -= n;
                board.col_count[*c] -= 1;
                board.row_count[*r] -= 1;
                if board.row_count[*r] == 0 || board.col_count[*c] == 0 {
                    return Some(n * board.total_sum);
                }
            }
        }
    }

    None
}

fn part2(mut boards: Vec<Board>, drawing: Drawing) -> Option<u32> {
    let mut board_has_won = vec![false; boards.len()];

    for n in drawing {
        for (i, board) in boards.iter_mut().enumerate() {
            if board_has_won[i] {
                continue;
            }

            if let Some((r, c)) = board.coords.get(&n) {
                board.total_sum -= n;
                board.col_count[*c] -= 1;
                board.row_count[*r] -= 1;
                if board.row_count[*r] == 0 || board.col_count[*c] == 0 {
                    board_has_won[i] = true;
                    if board_has_won.iter().all(|b| *b) {
                        return Some(n * board.total_sum);
                    }
                }
            }
        }
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Err(format!(
            "Usage: {} <input file>",
            args.get(0).unwrap_or(&"prog".into())
        ))?;
    }

    let input = fs::read_to_string(&args[1])?;

    let (boards, drawing) = parse_input(&input)?;
    let result = run_game(boards.clone(), drawing.clone());
    println!("Part 1: {:?}", result);

    let result = part2(boards, drawing);
    println!("Part 2: {:?}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("sample.txt");

    #[test]
    fn test_parse_input() {
        let (boards, nums) = parse_input(SAMPLE_INPUT).unwrap();

        assert_eq!(boards.len(), 3);
        assert_eq!(boards[0].row_count[0], 5);
        assert_eq!(boards[1].col_count[4], 5);
        assert_eq!(boards[0].coords[&14], (2, 2));
        assert_eq!(boards[2].coords[&14], (0, 0));

        assert_eq!(nums[3], 5);
    }

    #[test]
    fn test_game() {
        let (boards, nums) = parse_input(SAMPLE_INPUT).unwrap();

        assert_eq!(run_game(boards, nums), Some(4512));
    }
}
