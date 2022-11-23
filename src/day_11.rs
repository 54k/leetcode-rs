#[allow(dead_code)]
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;
    let mut cache = HashSet::with_capacity(board.len() * board.len());
    for r in 0..board.len() {
        for c in 0..board.len() {
            let ch = board[r][c];
            if ch != '.'
                && (!cache.insert(format!("{} in row {}", ch, r))
                    || !cache.insert(format!("{} in col {}", ch, c))
                    || !cache.insert(format!("{} in sub-square {} {}", ch, r / 3, c / 3)))
            {
                return false;
            }
        }
    }
    true
}

#[allow(dead_code)]
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let len = matrix.len();
    for row in 0..(len + 1) / 2 {
        for col in 0..len / 2 {
            let temp = matrix[len - 1 - col][row];
            matrix[len - 1 - col][row] = matrix[len - 1 - row][len - col - 1];
            matrix[len - 1 - row][len - col - 1] = matrix[col][len - 1 - row];
            matrix[col][len - 1 - row] = matrix[row][col];
            matrix[row][col] = temp;
        }
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut next = &mut head;
    for _ in 0..k {
        if let Some(n) = next {
            next = &mut n.next;
        } else {
            return head;
        }
    }

    let mut ret = reverse_k_group(next.take(), k);
    while let Some(h) = head.take() {
        ret = Some(Box::new(ListNode {
            val: h.val,
            next: ret,
        }));
        head = h.next;
    }
    ret
}

#[cfg(test)]
mod test {
    use crate::day_11::*;

    #[test]
    fn test64() {
        println!(
            "{}",
            is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '7'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '.', '9'],
            ])
        );
    }

    #[test]
    fn test65() {
        let mut matrix1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut matrix1);
        println!("{:?}", matrix1);
        let mut matrix2 = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        rotate(&mut matrix2);
        println!("{:?}", matrix2);
    }

    #[test]
    fn test66() {
        println!(
            "{:?}",
            reverse_k_group(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode { val: 7, next: None }))
                            })),
                        })),
                    })),
                })),
                2,
            )
        );
    }
}
