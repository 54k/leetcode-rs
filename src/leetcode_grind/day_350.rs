// https://leetcode.com/problems/maximum-subarray-min-product/description/
pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
    let mut sum = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        sum[i + 1] += nums[i] as i128 + sum[i] as i128;
    }

    let mut stack = vec![];
    let mut ans = 0i128;
    for i in 0..=nums.len() {
        while !stack.is_empty() && (i == nums.len() || nums[stack[stack.len() - 1]] > nums[i]) {
            let j = stack.pop().unwrap();
            let r = nums[j] as i128
                * (sum[i]
                    - sum[if stack.is_empty() {
                        0
                    } else {
                        stack[stack.len() - 1] + 1
                    }]) as i128;
            ans = ans.max(r);
        }
        stack.push(i);
    }
    (ans % 1_000_000_007) as i32
}

// https://leetcode.com/problems/find-positive-integer-solution-for-a-given-equation/
struct CustomFunction;
impl CustomFunction {
    pub fn f(x: i32, y: i32) -> i32 {
        0
    }
}

pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    for x in 1..=1000 {
        for y in 1..=1000 {
            if customfunction.f(x, y) == z {
                ans.push(vec![x, y]);
            }
        }
    }
    ans
}

// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/description/
pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    use std::collections::VecDeque;

    let mut adj = HashMap::new();

    for i in 0..n {
        adj.insert(i, vec![]);
    }

    for c in connections {
        let (from, to) = (c[0], c[1]);
        adj.entry(from).or_insert(vec![]).push((to, 1));
        adj.entry(to).or_insert(vec![]).push((from, 0));
    }

    let mut ans = 0;
    let mut vis = vec![false; n as usize];
    vis[0] = true;
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    while let Some((v, vdir)) = queue.pop_front() {
        ans += vdir;
        for (u, udir) in &adj[&v] {
            if !vis[(*u) as usize] {
                // println!("{} -> {} {}", v, *u, *udir);
                vis[(*u) as usize] = true;
                queue.push_back((*u, *udir));
            }
        }
    }
    ans
}

// https://leetcode.com/problems/distribute-candies-to-people/description/
pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
    let mut candies = candies;
    let mut ans = vec![0; num_people as usize];
    let mut curr = 1;
    let mut i = 0;

    while candies > 0 {
        let to_give = curr.min(candies);
        curr += 1;
        candies -= to_give;
        ans[i] += to_give;
        i = (i + 1) % num_people as usize;
    }

    ans
}