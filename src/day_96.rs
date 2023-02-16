#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/maximum-depth-of-binary-tree/
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::collections::*;
    let mut ans = 0;
    let mut q = VecDeque::new();
    q.push_back(root);
    while !q.is_empty() {
        let mut len = q.len();
        ans += 1;
        while len > 0 {
            len -= 1;
            if let Some(n) = q.pop_front().unwrap() {
                let n = n.borrow();
                if n.left.is_some() {
                    q.push_back(n.left.clone());
                }
                if n.right.is_some() {
                    q.push_back(n.right.clone());
                }
            }
        }
    }
    ans
}

// https://leetcode.com/problems/maximum-height-by-stacking-cuboids/description/
pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
    use std::cmp::*;
    let m = cuboids.len();

    let mut cuboids = cuboids
        .into_iter()
        .map(|mut cub| {
            cub.sort();
            cub.reverse();
            (cub[0], cub[1], cub[2])
        })
        .collect::<Vec<_>>();

    cuboids.sort();
    cuboids.sort_by(|(x1, y1, z1), (x2, y2, z2)| {
        // -- prevent wrong formatting
        if x1 < x2 && y1 < y2 && z1 < z2 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let mut ans = vec![(0, 0, 0); m];
    // walk through every pair since n size is only 100 and try to maximize stack in each position
    ans[0] = cuboids[0];
    for (i, (x, y, z)) in cuboids.into_iter().skip(1).enumerate() {
        for j in 0..=i {
            let prev = ans[j];
            if prev.1 >= y && prev.2 >= z {
                ans[i + 1] = ans[i + 1].max((x + prev.0, y, z));
            } else {
                ans[i + 1] = ans[i + 1].max((x, y, z));
            }
        }
    }

    ans.into_iter().map(|(x, _, _)| x).max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test229() {
        println!(
            "{}",
            max_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))))
        );
    }

    #[test]
    fn test230() {
        println!(
            "{}",
            max_height(vec![vec![50, 45, 20], vec![95, 37, 53], vec![45, 23, 12]])
        ); // 190

        // Cuboid 1 is placed on the bottom with the 53x37 side facing down with height 95.
        // Cuboid 0 is placed next with the 45x20 side facing down with height 50.
        // Cuboid 2 is placed next with the 23x12 side facing down with height 45.
        // The total height is 95 + 50 + 45 = 190.

        println!("{}", max_height(vec![vec![38, 25, 45], vec![76, 35, 3]])); // 76

        // You can't place any of the cuboids on the other.
        // We choose cuboid 1 and rotate it so that the 35x3 side is facing down and its height is 76.

        println!(
            "{}",
            max_height(vec![
                vec![7, 11, 17],
                vec![7, 17, 11],
                vec![11, 7, 17],
                vec![11, 17, 7],
                vec![17, 7, 11],
                vec![17, 11, 7]
            ])
        ); // 102

        // After rearranging the cuboids, you can see that all cuboids have the same dimension.
        // You can place the 11x7 side down on all cuboids so their heights are 17.
        // The maximum height of stacked cuboids is 6 * 17 = 102.

        println!(
            "{}",
            max_height(vec![vec![50, 26, 84], vec![2, 55, 62], vec![64, 63, 72]])
        ); // 134

        println!(
            "{}",
            max_height(vec![
                vec![36, 46, 41],
                vec![15, 100, 100],
                vec![75, 91, 59],
                vec![13, 82, 64]
            ])
        ); // 182
    }
}
