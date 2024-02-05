// https://leetcode.com/problems/circular-array-loop/description/
pub fn circular_array_loop(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n == 1 {
        return false;
    }

    use std::collections::HashSet;
    let mut vis = HashSet::new();

    for k in 0..n {
        if vis.contains(&k) {
            continue;
        }

        let mut cycleset = HashSet::new();
        let mut i = k;
        loop {
            // println!("enter {}", i);
            if cycleset.contains(&i) {
                return true;
            }

            vis.insert(i);
            cycleset.insert(i);

            let prev = i;
            i = ((((i as i32 + nums[i]) % n as i32) + n as i32) % n as i32) as usize;
            // println!("{} {}", prev, i);

            if prev == i {
                // println!("prev == i");
                break;
            }

            if (nums[prev] > 0) == (nums[i] < 0) {
                // println!("{} > 0 != {} < 0", nums[prev], nums[i]);
                break;
            }
        }
    }
    false
}

// https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/description/
pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut next = vec![0; nums.len()];
    let (mut minx, mut miny) = (0, i32::MAX);

    'outer: for i in 0..nums.len() {
        for _ in 0..nums[i].len() {
            let (mut min, mut max) = (0, 0);

            for k in 0..next.len() {
                if nums[min][next[min]] > nums[k][next[k]] {
                    min = k;
                }
                if nums[max][next[max]] < nums[k][next[k]] {
                    max = k;
                }
            }

            if nums[max][next[max]] - nums[min][next[min]] < miny - minx {
                minx = nums[min][next[min]];
                miny = nums[max][next[max]];
            }

            next[min] += 1;

            if next[min] == nums[min].len() {
                break 'outer;
            }
        }
    }

    vec![minx, miny]
}

#[test]
fn test_circular_array() {
    let res = circular_array_loop(vec![-1, -3, 9]);
    println!("{res}");
}
