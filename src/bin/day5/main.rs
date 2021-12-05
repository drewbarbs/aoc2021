use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
struct LineSeg {
    pub p0: (usize, usize),
    pub p1: (usize, usize),
}

impl LineSeg {
    pub fn is_diagonal(&self) -> bool {
        !(self.p0.0 == self.p1.0 || self.p0.1 == self.p1.1)
    }
}

impl FromStr for LineSeg {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .split(" -> ")
            .flat_map(|pt| pt.split(','))
            .map(|i| i.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        if coords.len() != 4 {
            return Err("Line does not have 2 points".into());
        }

        Ok(LineSeg {
            p0: (coords[0], coords[1]),
            p1: (coords[2], coords[3]),
        })
    }
}

fn get_line_segs(s: &str) -> Result<Vec<LineSeg>, Box<dyn Error>> {
    s.lines()
        .map(|l| l.parse::<LineSeg>())
        .collect::<Result<Vec<_>, _>>()
}

fn calc_overlaps(segs: &[LineSeg], with_diagonals: bool) -> usize {
    let max_x = segs
        .iter()
        .fold(0, |cur_max, seg| cur_max.max(seg.p0.0.max(seg.p1.0)));
    let max_y = segs
        .iter()
        .fold(0, |cur_max, seg| cur_max.max(seg.p0.1.max(seg.p1.1)));

    let mut grid: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];

    let mut overlapped_points = 0usize;
    for seg in segs {
        if !with_diagonals && seg.is_diagonal() {
            continue;
        }

        // draw the segment on the grid. Note that our implementation
        // using dx, dy as signum(x1-x0)/signum(y1-y0) only works
        // because we're told our input only has horizontal, vertical,
        // or diagonal lines that are at 45 degrees. Hence, we never
        // need to deal with fractional dx, dy
        let dx = ((seg.p1.0 as i32) - (seg.p0.0 as i32)).signum();
        let dy = ((seg.p1.1 as i32) - (seg.p0.1 as i32)).signum();
        let mut cur_point = seg.p0;
        loop {
            grid[cur_point.1][cur_point.0] += 1;
            if grid[cur_point.1][cur_point.0] == 2 {
                overlapped_points += 1;
            }

            if cur_point == seg.p1 {
                break;
            }
            cur_point.0 = (cur_point.0 as i32 + dx) as usize;
            cur_point.1 = (cur_point.1 as i32 + dy) as usize;
        }
    }

    overlapped_points
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()?;
    let line_segs = get_line_segs(&input)?;

    println!("Part 1: {}", calc_overlaps(line_segs.as_slice(), false));
    println!("Part 2: {}", calc_overlaps(line_segs.as_slice(), true));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("sample.txt");

    #[test]
    fn test_parse_line_segs() {
        let line_segs = get_line_segs(SAMPLE_INPUT).expect("Failed to parse line segments");
        assert_eq!(line_segs.len(), 10);
        assert_eq!(line_segs[4].p0, (7, 0));
        assert_eq!(line_segs[4].p1, (7, 4));
    }

    #[test]
    fn test_part1() {
        let line_segs = get_line_segs(SAMPLE_INPUT).unwrap();
        assert_eq!(calc_overlaps(line_segs.as_slice(), false), 5);
    }

    #[test]
    fn test_part2() {
        let line_segs = get_line_segs(SAMPLE_INPUT).unwrap();
        assert_eq!(calc_overlaps(line_segs.as_slice(), true), 12);
    }
}
