use wasm_bindgen::prelude::wasm_bindgen;

type Board = Vec<Vec<u8>>;

#[inline]
fn check_valid(data: &Board, row: Option<usize>, column: Option<usize>, target: u8) -> bool {
    match (row, column) {
        (Some(r), Some(c)) => {
            check_row(data, r, target)
                && check_column(data, c, target)
                && check_square(data, (r, c), target)
        }
        (Some(r), None) => check_row(data, r, target),
        (None, Some(c)) => check_column(data, c, target),
        _ => false,
    }
}

#[inline]
fn check_row(data: &Board, row: usize, target: u8) -> bool {
    for num in &data[row] {
        if num == &target {
            return false;
        }
    }
    true
}

#[inline]
fn check_column(data: &Board, column: usize, target: u8) -> bool {
    for row in data {
        if row[column] == target {
            return false;
        }
    }
    true
}

#[inline]
fn check_square(data: &Board, pos: (usize, usize), target: u8) -> bool {
    let local_box = (pos.0 - pos.0 % 3, pos.1 - pos.1 % 3);
    for r in 0..=2 {
        for c in 0..=2 {
            if data[local_box.0 + r][local_box.1 + c] == target {
                return false;
            }
        }
    }
    true
}

#[wasm_bindgen]
pub fn solve(s: Vec<u8>) -> Vec<u8> {
    let mut slices: Board = vec![];
    let mut row = Vec::with_capacity(9);
    for (index, item) in s.iter().enumerate() {
        if index % 9 == 0 {
            slices.push(row.clone());
            row.clear();
        }
        row.push(*item);
    }
    slices.push(row.clone());
    slices.remove(0);
    solve_b(&mut slices).0.iter().flatten().copied().collect()
}

#[inline]
pub fn solve_b(s: &mut Vec<Vec<u8>>) -> (Vec<Vec<u8>>, bool) {
    for r in 0..s.len() {
        for c in 0..s[0].len() {
            if s[r][c] > 0 {
                continue;
            }
            for i in 1..=s.len() {
                if check_valid(
                    s,
                    Some(r),
                    Some(c),
                    i as u8,
                ) {
                    s[r][c] = i as u8;
                    if let (s, true) = solve_b(s) {
                        return (s.to_vec(), true);
                    } else {
                        s[r][c] = 0;
                    }
                }
            }
        }
    }
    (s.to_vec(), true)
}

