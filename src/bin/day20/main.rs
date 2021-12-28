use std::error::Error;

fn parse_input(s: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let mut lines = s.lines();

    let to_u8 = |c| if c == '#' { 1 } else { 0 };

    let alg = lines
        .next()
        .unwrap()
        .chars()
        .map(to_u8)
        .collect::<Vec<u8>>();

    // skip blank line
    lines.next();

    let mat = lines
        .map(|l| l.chars().map(to_u8).collect::<Vec<u8>>())
        .collect::<Vec<_>>();

    (alg, mat)
}

fn enhance(
    src_img: &Vec<u8>,
    src_dims: (usize, usize),
    fill: u8,
    dst_img: &mut Vec<u8>,
    alg: &Vec<u8>,
) -> u8 {
    let src_n = src_dims.0 as i32;
    let src_m = src_dims.1 as i32;

    let get_alg_idx = |new_r: i32, new_c: i32| {
        let mut idx = 0usize;
        for r in (new_r - 1)..=(new_r + 1) {
            for c in (new_c - 1)..=(new_c + 1) {
                let px_val = if (1..=src_n).contains(&r) && (1..=src_m).contains(&c) {
                    src_img[((r - 1) * src_m + c - 1) as usize]
                } else {
                    fill
                };
                idx = (idx << 1) | (px_val as usize);
            }
        }
        idx
    };

    for r in 0..(src_n + 2) {
        for c in 0..(src_m + 2) {
            let dst_idx = (r * (src_m + 2) + c) as usize;
            dst_img[dst_idx] = alg[get_alg_idx(r, c) as usize];
        }
    }

    let fill = if fill == 1 { alg[511] } else { alg[0] };

    fill
}

fn num_lit_after_m_iterations(img: &Vec<Vec<u8>>, mut fill: u8, alg: &Vec<u8>, m: usize) -> usize {
    let src_n = img.len();
    let src_m = img[0].len();
    let img = img.iter().flatten().map(|b| *b).collect::<Vec<u8>>();

    let new_n = src_n + 2 * m;
    let new_m = src_m + 2 * m;

    let mut a: Vec<u8> = Vec::with_capacity(new_n * new_m);
    a.resize(new_n * new_m, 0);

    let mut b = a.clone();
    a[0..(src_n * src_m)].copy_from_slice(img.as_slice());

    for i in 0..m {
        let (src, dst): (&mut Vec<u8>, &mut Vec<u8>) = if i % 2 == 0 {
            (&mut a, &mut b)
        } else {
            (&mut b, &mut a)
        };

        fill = enhance(src, (src_n + 2 * i, src_m + 2 * i), fill, dst, alg);
    }

    let last_dst: &Vec<u8> = if (m - 1) % 2 == 0 { &b } else { &a };

    last_dst.iter().map(|b| *b as usize).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()?;

    let (alg, img) = parse_input(&input);

    println!("Part 1: {}", num_lit_after_m_iterations(&img, 0, &alg, 2));
    println!("Part 2: {}", num_lit_after_m_iterations(&img, 0, &alg, 50));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("./sample.txt");

    #[test]
    fn test_input_parse() {
        let (alg, mat) = parse_input(SAMPLE_INPUT);

        assert_eq!(alg[0], 0);
        assert_eq!(alg[30], 1);
        assert_eq!(alg[31], 1);
        assert_eq!(alg[32], 1);
        assert_eq!(alg[33], 0);
        assert_eq!(alg[510], 0);
        assert_eq!(alg[511], 1);

        assert_eq!(alg.len(), 512);

        let expected: Vec<Vec<u8>> = vec![
            vec![1, 0, 0, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 1, 1],
        ];

        assert_eq!(expected, mat);
    }

    #[test]
    fn test_enhance() {
        let (alg, mat) = parse_input(SAMPLE_INPUT);

        let src_n = mat.len();
        let src_m = mat[0].len();
        let new_capacity = (src_n + 2) * (src_m + 2);
        let src_mat = mat.into_iter().flatten().collect::<Vec<_>>();
        let mut dst_mat: Vec<u8> = Vec::with_capacity(new_capacity);
        dst_mat.resize(new_capacity, 0);

        let fill = enhance(&src_mat, (src_n, src_m), 0, &mut dst_mat, &alg);

        let expected = vec![
            0, 1, 1, 0, 1, 1, 0, // Leave this comment
            1, 0, 0, 1, 0, 1, 0, // to avoid rust formatter
            1, 1, 0, 1, 0, 0, 1, // from destroying my nice
            1, 1, 1, 1, 0, 0, 1, // matrix
            0, 1, 0, 0, 1, 1, 0, //
            0, 0, 1, 1, 0, 0, 1, //
            0, 0, 0, 1, 0, 1, 0,
        ];

        assert_eq!(expected.len(), new_capacity);
        assert_eq!(fill, 0);

        assert_eq!(expected, dst_mat);
    }

    #[test]
    fn test_solution() {
        let (alg, mat) = parse_input(SAMPLE_INPUT);

        assert_eq!(35, num_lit_after_m_iterations(&mat, 0, &alg, 2));
    }
}
