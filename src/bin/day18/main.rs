use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum SnailfishNumber {
    Regular(u64),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

#[derive(Debug)]
enum SnailfishParseError {
    PrematureEnd,
    InvalidChar,
}

impl Error for SnailfishParseError {}

impl fmt::Display for SnailfishParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SnailfishParseError::InvalidChar => write!(f, "Encountered invalid character"),
            &SnailfishParseError::PrematureEnd => write!(f, "Unexpected end of input"),
        }
    }
}

fn read_sn<I>(it: &mut Peekable<I>) -> Result<SnailfishNumber, SnailfishParseError>
where
    I: Iterator<Item = char>,
{
    let c = it.next().ok_or(SnailfishParseError::PrematureEnd)?;
    match c {
        '[' => {
            let left = read_sn(it)?;
            if it.next().ok_or(SnailfishParseError::PrematureEnd)? != ',' {
                return Err(SnailfishParseError::InvalidChar);
            }
            let right = read_sn(it)?;
            if it.next().ok_or(SnailfishParseError::PrematureEnd)? != ']' {
                return Err(SnailfishParseError::InvalidChar);
            }

            Ok(SnailfishNumber::Pair(left.into(), right.into()))
        }
        '0'..='9' => {
            let mut val = u64::from(c) - u64::from('0');
            while it
                .peek()
                .ok_or(SnailfishParseError::PrematureEnd)?
                .is_ascii_digit()
            {
                val = val * 10 + (u64::from(it.next().unwrap()) - u64::from('0'));
            }
            Ok(SnailfishNumber::Regular(val))
        }
        _ => Err(SnailfishParseError::InvalidChar),
    }
}

impl FromStr for SnailfishNumber {
    type Err = SnailfishParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        read_sn(&mut s.chars().filter(|c| !c.is_whitespace()).peekable())
    }
}

enum Edit {
    Complete,
    AddLeft(u64),
    AddRight(u64),
}

fn add_to_leftmost(n: SnailfishNumber, v: u64) -> SnailfishNumber {
    match n {
        SnailfishNumber::Regular(x) => SnailfishNumber::Regular(x + v),
        SnailfishNumber::Pair(l, r) => SnailfishNumber::Pair(add_to_leftmost(*l, v).into(), r),
    }
}

fn add_to_rightmost(n: SnailfishNumber, v: u64) -> SnailfishNumber {
    match n {
        SnailfishNumber::Regular(x) => SnailfishNumber::Regular(x + v),
        SnailfishNumber::Pair(l, r) => SnailfishNumber::Pair(l, add_to_rightmost(*r, v).into()),
    }
}

fn do_explode(n: SnailfishNumber, depth: u32) -> (SnailfishNumber, Option<Edit>) {
    fn get_regular_value(n: SnailfishNumber) -> u64 {
        match n {
            SnailfishNumber::Regular(v) => v,
            _ => panic!("Getting regular value of non-regular number"),
        }
    }

    match (n, depth) {
        (SnailfishNumber::Regular(v), 0..=4) => (SnailfishNumber::Regular(v), None),
        (SnailfishNumber::Pair(l, r), 0..=2) => match do_explode(*l, depth + 1) {
            (l, Some(Edit::AddRight(v))) => (
                SnailfishNumber::Pair(l.into(), add_to_leftmost(*r, v).into()),
                Some(Edit::Complete),
            ),
            (l, Some(edit)) => (SnailfishNumber::Pair(l.into(), r), Some(edit)),
            (l, None) => match do_explode(*r, depth + 1) {
                (r, Some(Edit::AddLeft(v))) => (
                    SnailfishNumber::Pair(add_to_rightmost(l, v).into(), r.into()),
                    Some(Edit::Complete),
                ),
                (r, opt) => (SnailfishNumber::Pair(l.into(), r.into()), opt),
            },
        },
        (SnailfishNumber::Pair(l, r), 3) => match (*l, *r) {
            (SnailfishNumber::Pair(ll, lr), r) => {
                let llv = get_regular_value(*ll);
                let lrv = get_regular_value(*lr);
                (
                    SnailfishNumber::Pair(
                        SnailfishNumber::Regular(0).into(),
                        add_to_leftmost(r, lrv).into(),
                    ),
                    Some(Edit::AddLeft(llv)),
                )
            }
            (SnailfishNumber::Regular(lv), SnailfishNumber::Pair(rl, rr)) => {
                let rlv = get_regular_value(*rl);
                let rrv = get_regular_value(*rr);
                (
                    SnailfishNumber::Pair(
                        SnailfishNumber::Regular(lv + rlv).into(),
                        SnailfishNumber::Regular(0).into(),
                    ),
                    Some(Edit::AddRight(rrv)),
                )
            }
            (reg_left, reg_right) => (
                SnailfishNumber::Pair(reg_left.into(), reg_right.into()),
                None,
            ),
        },
        _ => panic!("Unexpected depth"),
    }
}

fn do_split(n: SnailfishNumber) -> (SnailfishNumber, bool) {
    match n {
        SnailfishNumber::Pair(l, r) => match do_split(*l) {
            (l, false) => {
                let (r, have_split) = do_split(*r);
                (SnailfishNumber::Pair(l.into(), r.into()), have_split)
            }
            (l, true) => (SnailfishNumber::Pair(l.into(), r), true),
        },
        SnailfishNumber::Regular(v) => {
            if v >= 10 {
                (
                    SnailfishNumber::Pair(
                        SnailfishNumber::Regular(v / 2).into(),
                        SnailfishNumber::Regular(v / 2 + (v % 2)).into(),
                    ),
                    true,
                )
            } else {
                (SnailfishNumber::Regular(v), false)
            }
        }
    }
}

fn reduce(mut n: SnailfishNumber) -> SnailfishNumber {
    loop {
        let (reduced, edit) = do_explode(n, 0);
        if edit.is_some() {
            n = reduced;
            continue;
        }

        let (reduced, have_split) = do_split(reduced);
        if !have_split {
            return reduced;
        }
        n = reduced;
    }
}

fn add(a: SnailfishNumber, b: SnailfishNumber) -> SnailfishNumber {
    reduce(SnailfishNumber::Pair(a.into(), b.into()))
}

fn magnitude(a: &SnailfishNumber) -> u64 {
    match a {
        SnailfishNumber::Regular(v) => *v,
        SnailfishNumber::Pair(l, r) => 3 * magnitude(&l) + 2 * magnitude(&r),
    }
}

fn part2(nums: &Vec<SnailfishNumber>) -> u64 {
    let mut max_sum_magnitude: Option<u64> = None;

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                // we need to sum *different* numbers
                continue;
            }

            let cur_magnitude = magnitude(&add(nums[i].clone(), nums[j].clone()));

            max_sum_magnitude =
                Some(max_sum_magnitude.map_or(cur_magnitude, |v| v.max(cur_magnitude)));
        }
    }

    max_sum_magnitude.unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()?;

    let nums = input
        .lines()
        .into_iter()
        .map(|l| l.parse::<SnailfishNumber>())
        .collect::<Result<Vec<_>, _>>()?;

    let sum = nums
        .clone()
        .into_iter()
        .reduce(|a, b| add(a, b))
        .ok_or("Empty input list?")?;

    println!("Part 1: {}", magnitude(&sum));

    println!("Part 2: {}", part2(&nums));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_snailfish_numbers() {
        let pair_1_2 = SnailfishNumber::Pair(
            SnailfishNumber::Regular(1).into(),
            SnailfishNumber::Regular(2).into(),
        );

        let parsed = "[1,2]".parse::<SnailfishNumber>().unwrap();
        assert_eq!(pair_1_2, parsed);

        let parsed = "[[1,  2],   3]".parse::<SnailfishNumber>().unwrap();
        assert_eq!(
            SnailfishNumber::Pair(pair_1_2.into(), SnailfishNumber::Regular(3).into()),
            parsed
        );
    }

    #[test]
    fn test_reduce() {
        let unreduced = "[[[[[9,8],1],2],3],4]".parse::<SnailfishNumber>().unwrap();
        let reduced = "[[[[0,9],2],3],4]".parse::<SnailfishNumber>().unwrap();
        assert_eq!(reduced, reduce(unreduced));

        let unreduced = "[7,[6,[5,[4,[3,2]]]]]".parse::<SnailfishNumber>().unwrap();
        let reduced = "[7,[6,[5,[7,0]]]]".parse::<SnailfishNumber>().unwrap();
        assert_eq!(reduced, reduce(unreduced));

        let unreduced = "[[6,[5,[4,[3,2]]]],1]".parse::<SnailfishNumber>().unwrap();
        let reduced = "[[6,[5,[7,0]]],3]".parse::<SnailfishNumber>().unwrap();
        assert_eq!(reduced, reduce(unreduced));

        let unreduced = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
            .parse::<SnailfishNumber>()
            .unwrap();
        let reduced = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
            .parse::<SnailfishNumber>()
            .unwrap();
        assert_eq!(reduced, reduce(unreduced));

        let unreduced = "[10, 11]".parse::<SnailfishNumber>().unwrap();
        let reduced = "[[5,5],[5,6]]".parse::<SnailfishNumber>().unwrap();
        assert_eq!(reduced, reduce(unreduced));

        let unreduced = "[1, [2, 13]]".parse::<SnailfishNumber>().unwrap();
        let reduced = "[1, [2,[6,7]]]".parse::<SnailfishNumber>().unwrap();
        assert_eq!(reduced, reduce(unreduced));
    }

    #[test]
    fn test_add() {
        let a = "[[[[4,3],4],4],[7,[[8,4],9]]]"
            .parse::<SnailfishNumber>()
            .unwrap();
        let b = "[1,1]".parse::<SnailfishNumber>().unwrap();

        let result = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
            .parse::<SnailfishNumber>()
            .unwrap();

        assert_eq!(result, add(a, b));

        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        let nums = input
            .lines()
            .map(|l| l.parse::<SnailfishNumber>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let result = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse::<SnailfishNumber>()
            .unwrap();
        assert_eq!(result, nums.into_iter().reduce(|a, b| add(a, b)).unwrap());
    }

    #[test]
    fn test_magnitude() {
        let num = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            .parse::<SnailfishNumber>()
            .unwrap();

        assert_eq!(4140, magnitude(&num));
    }
}
