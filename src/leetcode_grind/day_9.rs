#[allow(dead_code)]
pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    use std::collections::VecDeque;

    let dirs = [[0, -1], [0, 1], [1, 0], [-1, 0]];

    let (rows, cols) = (maze.len() as i32, maze[0].len() as i32);
    let (r, c) = (entrance[0] as usize, entrance[1] as usize);

    let mut q = VecDeque::new();
    q.push_back(vec![r, c, 0]);
    maze[r][c] = '+';

    while !q.is_empty() {
        let el = q.pop_front().unwrap();
        let (cur_r, cur_c, cur_dist) = (el[0] as i32, el[1] as i32, el[2]);

        for dir in dirs {
            let (next_r, next_c) = (cur_r + dir[0], cur_c + dir[1]);
            // is unvisited move
            if next_r >= 0
                && next_r < rows
                && next_c >= 0
                && next_c < cols
                && maze[next_r as usize][next_c as usize] == '.'
            {
                if next_r == 0 || next_r == rows - 1 || next_c == 0 || next_c == cols - 1 {
                    return (cur_dist + 1) as i32;
                }

                maze[next_r as usize][next_c as usize] = '+';
                q.push_back(vec![next_r as usize, next_c as usize, cur_dist + 1]);
            }
        }
    }
    -1
}

// https://leetcode.com/problems/divide-two-integers/discuss/1516367/Complete-Thinking-Process-or-Intuitive-Explanation-or-All-rules-followed-or-C%2B%2B-code
// https://leetcode.com/problems/divide-two-integers/discuss/2090591/Rust-or-Negative-Division-or-i32-Only
#[allow(dead_code)]
pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let is_neg = (dividend < 0) ^ (divisor < 0);
    let mut p = if dividend > 0 { -dividend } else { dividend };
    let q = if divisor > 0 { -divisor } else { divisor };

    // res is the *negative* absolute value of the quotient, in case the quotient is i32::MIN.
    let mut res = 0;
    // Here is the magic: the negative divisor(aka q) can left-shift at most q.leading_ones() - 1 bits before it overflows.
    for shift in (0..q.leading_ones()).rev() {
        // Don't apply "p -= (q << shift)" before overflow checking. Consider p = 0, q << shift = i32::MIN.
        if p <= (q << shift) {
            p -= q << shift;
            res += -1 << shift; // Not "res -= 1 << shift;" which may cause overflow.
        }
    }

    if is_neg {
        res
    } else if res == i32::MIN {
        i32::MAX
    } else {
        -res
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut tail = head.as_mut()?;

    while let Some(v) = tail.next.as_mut() {
        if v.val == tail.val {
            tail.next = v.next.take();
        } else {
            tail = tail.next.as_mut().unwrap();
        }
    }

    head
}

#[allow(dead_code)]
pub fn delete_duplicates_distinct(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut sentinel = ListNode { val: 0, next: head };

    let mut pred = &mut sentinel;

    while let Some(mut h) = pred.next.as_mut() {
        if h.next.is_some() && h.val == h.next.as_ref().unwrap().val {
            while h.next.is_some() && h.val == h.next.as_ref().unwrap().val {
                h = h.next.as_mut().unwrap();
            }
            pred.next = h.next.take();
        } else {
            pred = pred.next.as_mut().unwrap();
        }
    }

    sentinel.next
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test55() {
        println!(
            "{}",
            nearest_exit(
                vec![
                    vec!['+', '+', '+'],
                    vec!['.', '.', '.'],
                    vec!['+', '+', '+'],
                ],
                vec![1, 0],
            )
        );
    }

    #[test]
    fn test56() {
        println!("{}", divide(58, 5));
        println!("{}", divide(-2147483648, -1));
    }

    #[test]
    fn test57() {
        println!(
            "{:?}",
            delete_duplicates(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 5, next: None })),
                        })),
                    })),
                })),
            })))
        );
    }

    #[test]
    fn test58() {
        println!(
            "{:?}",
            delete_duplicates_distinct(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode { val: 5, next: None })),
                        })),
                    })),
                })),
            })))
        );
    }
}
