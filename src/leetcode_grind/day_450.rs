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

#[test]
fn test_circular_array() {
    let res = circular_array_loop(vec![-1, -3, 9]);
    println!("{res}");
}
