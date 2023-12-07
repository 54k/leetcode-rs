// https://leetcode.com/problems/count-of-smaller-numbers-after-self/description/
pub fn count_smaller_segtree(nums: Vec<i32>) -> Vec<i32> {
    const OFFSET: usize = 10_000;
    const SIZE: usize = 2 * OFFSET + 1;

    let mut tree = vec![0; 2 * SIZE];

    fn set(tree: &mut Vec<i32>, mut index: usize) {
        index += SIZE;
        tree[index] += 1;
        while index > 1 {
            index /= 2;
            tree[index] = tree[index * 2] + tree[index * 2 + 1];
        }
    }

    fn query(tree: &Vec<i32>, mut left: usize, mut right: usize) -> i32 {
        left += SIZE;
        right += SIZE;
        let mut sum = 0;
        while left < right {
            if left % 2 == 1 {
                sum += tree[left];
                left += 1;
            }

            if right % 2 == 1 {
                right -= 1;
                sum += tree[right];
            }

            left /= 2;
            right /= 2;
        }
        sum
    }

    let mut ans = vec![0; nums.len()];
    for i in (0..nums.len()).rev() {
        ans[i] = query(&tree, 0, nums[i] as usize + OFFSET);
        set(&mut tree, nums[i] as usize + OFFSET);
    }
    ans
}

pub fn count_smaller_bit(nums: Vec<i32>) -> Vec<i32> {
    const OFFSET: i32 = 10_000;
    const SIZE: usize = 2 * OFFSET as usize + 2;

    let mut bit = vec![0; SIZE];

    fn set(bit: &mut Vec<i32>, mut index: i32) {
        index += 1;
        while index < bit.len() as i32 {
            bit[index as usize] += 1;
            index += index & -index;
        }
    }

    fn query(bit: &Vec<i32>, mut index: i32) -> i32 {
        index += 1;
        let mut sum = 0;
        while index >= 1 {
            sum += bit[index as usize];
            index -= index & -index;
        }
        sum
    }

    let mut ans = vec![0; nums.len()];
    for i in (0..nums.len()).rev() {
        ans[i] = query(&bit, nums[i] + OFFSET - 1);
        set(&mut bit, nums[i] + OFFSET);
    }
    ans
}
