// https://leetcode.com/problems/k-th-symbol-in-grammar/description
pub fn kth_grammar_tree_bin_search(n: i32, k: i32) -> i32 {
    fn bin_search(n: i32, k: i32, val: i32) -> i32 {
        if n == 1 {
            return val;
        }
        let total_elements = 2u32.pow(n as u32 - 1) as i32;
        if k > total_elements / 2 {
            let next = if val == 0 { 1 } else { 0 };
            return bin_search(n -1, k - total_elements / 2, next)
        } else {
            let next = if val == 0 { 0 } else { 1 };
            return bin_search(n -1, k, next)
        }
    }
    bin_search(n, k, 0)
}

pub fn kth_grammar_recursive(n: i32, k: i32) -> i32 {
    fn rec(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        let total_elements = 2u32.pow(n as u32 - 1) as i32;
        if k > total_elements / 2 {
            1 - rec(n - 1, k - total_elements / 2)
        } else {
            rec(n - 1, k)
        }
    }
    rec(n, k)
}

pub fn kth_grammar_iterative(n: i32, k: i32) -> i32 {
    if n == 1 {
        return 0;
    }

    let mut k = k;
    let mut symbol = 1;

    for row in (2..=n).rev() {
        let total_elements = 2u32.pow(row as u32 - 1) as i32;
        let half_elements = total_elements / 2;

        if k > half_elements {
            symbol = 1 - symbol;
            k -= half_elements;
        }
    }

    // we reached 1st row; if the symbol is not '0', we started with the wrong symbol
    if symbol != 0 {
        return 0;
    }

    1
}

pub fn kth_grammar_math(n: i32, k: i32) -> i32 {
    let ones = (k - 1).count_ones();
    if ones % 2 == 0 {
        0
    } else {
        1
    }
}
