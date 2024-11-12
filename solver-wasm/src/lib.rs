use wasm_bindgen::prelude::wasm_bindgen;

type Board = [[u8; 9]; 9];

#[inline]
fn check_valid(data: &Board, r: usize, c: usize, target: u8) -> bool {
    check_row(data, r, target)
        && check_column(data, c, target)
        && check_square(data, (r, c), target)
}

#[inline]
fn check_row(data: &Board, row: usize, target: u8) -> bool {
    unsafe {
        for num in data.get_unchecked(row) {
            if num == &target {
                return false;
            }
        }
    }
    true
}

#[inline]
fn check_column(data: &Board, column: usize, target: u8) -> bool {
    for row in data {
        unsafe {
            if row.get_unchecked(column) == &target {
                return false;
            }
        }
    }
    true
}

#[inline]
fn check_square(data: &Board, pos: (usize, usize), target: u8) -> bool {
    let local_box = (pos.0 - pos.0 % 3, pos.1 - pos.1 % 3);
    for r in 0..3 {
        for c in 0..3 {
            let (r, c) = (local_box.0 + r, local_box.1 + c);
            unsafe {
                if data.get_unchecked(r).get_unchecked(c) == &target {
                    return false;
                }
            }
        }
    }
    true
}

#[wasm_bindgen]
pub fn solve(s: Vec<u8>) -> Vec<u8> {
    let mut slices: Board = [[0; 9]; 9];
    for (index, item) in s.iter().enumerate() {
        unsafe {
            *(*slices.get_unchecked_mut(index / 9)).get_unchecked_mut(index % 9) = *item;
        }
    }
    solve_b(&mut slices).0.iter().flatten().copied().collect()
}

#[inline]
pub fn solve_b(s: &mut Board) -> (&mut Board, bool) {
    for r in 0..s.len() {
        for c in 0..s[0].len() {
            if s[r][c] > 0 {
                continue;
            }
            for i in 1..=s.len() {
                if check_valid(s, r, c, i as u8) {
                    s[r][c] = i as u8;
                    if let (_, true) = solve_b(s) {
                        break;
                    } else {
                        s[r][c] = 0;
                    }
                }
            }
            if s[r][c] == 0 {
                return (s, false);
            }
        }
    }
    return (s, true);
}
