use std::{cell::RefCell, rc::Rc};

// https://leetcode.com/problems/sliding-window-maximum/description/
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut ans = vec![];

    for right in 0..nums.len() {
        while !queue.is_empty() && nums[right] > nums[queue[queue.len() - 1]] {
            queue.pop_back();
        }
        queue.push_back(right);

        while !queue.is_empty() && (queue[0] as i32) < right as i32 - k as i32 + 1 {
            queue.pop_front();
        }

        if right + 1 >= k as usize {
            ans.push(nums[queue[0]]);
        }
    }
    ans
}

// https://leetcode.com/problems/determine-if-two-events-have-conflict/description/
pub fn have_conflict_1(event1: Vec<String>, event2: Vec<String>) -> bool {
    if event1[0] > event2[0] {
        return have_conflict_1(event2, event1);
    }
    event1[1] >= event2[0]
}

pub fn have_conflict_2(event1: Vec<String>, event2: Vec<String>) -> bool {
    let r = event1[1].clone().min(event2[1].clone());
    let l = event1[0].clone().max(event2[0].clone());
    l <= r
}

// https://leetcode.com/problems/basic-calculator/description/
pub fn calculate(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let mut stack = vec![];
    let mut operand = 0;
    let mut result = 0;
    let mut sign = 1;

    for i in 0..s.len() {
        let ch = s[i];
        if char::is_digit(ch, 10) {
            operand = 10 * operand + (ch as i32 - '0' as i32);
        } else if ch == '+' {
            result += sign * operand;
            sign = 1;
            operand = 0;
        } else if ch == '-' {
            result += sign * operand;
            sign = -1;
            operand = 0;
        } else if ch == '(' {
            stack.push(result);
            stack.push(sign);
            sign = 1;
            result = 0;
        } else if ch == ')' {
            result += sign * operand;
            result *= stack.pop().unwrap();
            result += stack.pop().unwrap();
            operand = 0;
        }
    }
    result + (sign * operand)
}

// https://leetcode.com/problems/search-in-a-binary-search-tree/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn search_bst(
    mut root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    while root.is_some() {
        if let Some(mut r) = root.clone() {
            let r = r.borrow();
            let v = r.val;
            if v == val {
                return root;
            }
            if v != val {
                if val < v {
                    root = r.left.clone();
                } else {
                    root = r.right.clone();
                }
            }
        }
    }
    root
}

// https://leetcode.com/problems/the-skyline-problem/description/
pub fn get_skyline(mut buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    struct UF {
        root: Vec<usize>,
    }
    impl UF {
        fn new(n: usize) -> Self {
            let mut root = vec![0; n];
            for i in 0..n {
                root[i] = i;
            }
            Self { root }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.root[x] == x {
                return x;
            }
            self.root[x] = self.find(self.root[x]);
            self.root[x]
        }

        fn union(&mut self, x: usize, y: usize) {
            self.root[x] = self.root[y];
        }
    }

    use std::collections::{BTreeSet, HashMap};
    let mut edge_set = BTreeSet::new();
    for building in &buildings {
        edge_set.insert(building[0]);
        edge_set.insert(building[1]);
    }

    let edges = edge_set.into_iter().collect::<Vec<_>>();
    let mut edge_idx_map = HashMap::new();
    for i in 0..edges.len() {
        edge_idx_map.insert(edges[i], i);
    }

    // sort the buildings in desc order
    buildings.sort_by(|a, b| b[2].cmp(&a[2]));

    // println!("{:?}", buildings);

    let n = edges.len();
    let mut edge_uf = UF::new(n);
    let mut heights = vec![0; n];

    for building in buildings {
        let left_edge = building[0];
        let right_edge = building[1];
        let height = building[2];

        let mut left_index = edge_idx_map[&left_edge];
        let right_index = edge_idx_map[&right_edge];

        // println!("li {} ri {}", left_index, right_index);

        while left_index < right_index {
            left_index = edge_uf.find(left_index);

            if left_index < right_index {
                edge_uf.union(left_index, right_index);
                heights[left_index] = height;
                left_index += 1;
            }
        }
    }

    // println!("{:?}", heights);

    let mut ans = vec![];
    for i in 0..n {
        if i == 0 || heights[i] != heights[i - 1] {
            ans.push(vec![edges[i], heights[i]]);
        }
    }
    ans
}

// https://leetcode.com/problems/minimum-time-takes-to-reach-destination-without-drowning/
pub fn minimum_seconds(land: Vec<Vec<String>>) -> i32 {
    use std::collections::VecDeque;
    const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn is_valid(x: i32, y: i32, land: &Vec<Vec<String>>) -> bool {
        0 <= x && x < land.len() as i32 && 0 <= y && y < land[0].len() as i32
    }

    fn mark_flooded(land: &Vec<Vec<String>>) -> Vec<Vec<i32>> {
        let mut flood_time = vec![vec![i32::MAX; land[0].len()]; land.len()];
        let mut queue = VecDeque::new();
        let mut vis = vec![vec![false; land[0].len()]; land.len()];

        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if land[i][j] == "*".to_string() {
                    queue.push_back((i as i32, j as i32));
                    vis[i][j] = true;
                }
            }
        }

        let mut time = 0;
        while queue.len() > 0 {
            let mut k = queue.len();
            while k > 0 {
                let (x, y) = queue.pop_front().unwrap();
                flood_time[x as usize][y as usize] = time;

                for d in &DIR {
                    let (nx, ny) = (x + d.0, y + d.1);
                    if is_valid(nx, ny, &land)
                        && !vis[nx as usize][ny as usize]
                        && land[nx as usize][ny as usize] != "X".to_string()
                        && land[nx as usize][ny as usize] != "D".to_string()
                    {
                        vis[nx as usize][ny as usize] = true;
                        queue.push_back((nx, ny));
                    }
                }
                k -= 1;
            }
            time += 1;
        }
        flood_time
    }

    fn get_time(land: &Vec<Vec<String>>, flood_time: &Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut vis = vec![vec![false; land[0].len()]; land.len()];

        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if land[i][j] == "S".to_string() {
                    queue.push_back((i as i32, j as i32, 0));
                    vis[i][j] = true;
                    break;
                }
            }
        }

        while let Some((x, y, time)) = queue.pop_front() {
            if land[x as usize][y as usize] == "D".to_string() {
                return time;
            }
            for d in &DIR {
                let (nx, ny) = (x + d.0, y + d.1);

                if is_valid(nx, ny, &land)
                    && !vis[nx as usize][ny as usize]
                    && flood_time[nx as usize][ny as usize] > time + 1
                    && land[nx as usize][ny as usize] != "X".to_string()
                {
                    vis[nx as usize][ny as usize] = true;
                    queue.push_back((nx, ny, time + 1));
                }
            }
        }

        -1
    }
    let flooded = mark_flooded(&land);
    get_time(&land, &flooded)
}

#[test]
fn test_flooded() {
    let res = minimum_seconds(vec![
        vec!["D".to_string(), ".".to_string()],
        vec![".".to_string(), "S".to_string()],
    ]);
    println!("{res}"); // 2

    let res = minimum_seconds(vec![
        vec!["D".to_string(), ".".to_string(), "*".to_string()],
        vec![".".to_string(), ".".to_string(), ".".to_string()],
        vec![".".to_string(), "S".to_string(), ".".to_string()],
    ]);
    println!("{res}"); // 3
}
