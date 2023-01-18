// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/2868539/maximum-sum-circular-subarray/
pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let mut cur_min = 0;
    let mut min_sum = nums[0];
    let mut cur_max = 0;
    let mut max_sum = nums[0];
    let mut sum = 0;

    for num in nums {
        cur_min = 0.min(cur_min) + num;
        min_sum = min_sum.min(cur_min);

        cur_max = 0.max(cur_max) + num;
        max_sum = max_sum.max(cur_max);

        sum += num;
    }

    if min_sum == sum {
        max_sum
    } else {
        max_sum.max(sum - min_sum)
    }
}

// https://leetcode.com/problems/count-good-triplets-in-an-array/solutions/1783180/python-2-fenwick-trees-solution-explained/?orderBy=most_relevant
pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    use std::collections::*;
    fn fenwick_add(tree: &mut [i64], mut k: i32, num: i64) {
        while (k as usize) < tree.len() {
            tree[k as usize] += num;
            k += k & (-k);
        }
    }
    fn fenwick_sum(tree: &mut [i64], mut k: i32) -> i64 {
        let mut ans = 0i64;
        while k > 0 {
            ans += tree[k as usize] as i64;
            k -= k & -k;
        }
        ans
    }

    let n = nums1.len();
    let mut d = nums1
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();
    let arr = nums2
        .into_iter()
        .map(|x| d.remove(&x).unwrap() as i32)
        .collect::<Vec<_>>();

    let mut bit1 = vec![0; n];
    let mut bit2 = vec![0; n];
    let mut ans = 0;
    for i in arr {
        ans += fenwick_sum(&mut bit2, i);
        fenwick_add(&mut bit1, i + 1, 1);
        let less = fenwick_sum(&mut bit1, i);
        fenwick_add(&mut bit2, i + 1, less);
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test162() {
        println!("{}", max_subarray_sum_circular(vec![5, -3, 5]));
    }

    #[test]
    fn test163() {
        println!("{}", good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3])); // 1
        println!(
            "{}",
            good_triplets(vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3])
        ); // 4
    }
}
