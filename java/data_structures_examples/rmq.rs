// https://leetcode.com/problems/create-sorted-array-through-instructions/description/
pub fn create_sorted_array_seg_tree(instructions: Vec<i32>) -> i32 {
    const m: usize = 100001 as usize;
    let mut tree = vec![0; m * 2];

    fn update(tree: &mut Vec<i32>, mut index: usize, value: i32) {
        index += m;
        tree[index] += value;
        while index > 0 {
            index /= 2;
            tree[index] = tree[index * 2] + tree[index * 2 + 1];
        }
    }

    fn query(tree: &Vec<i32>, mut left: usize, mut right: usize) -> i32 {
        let mut result = 0;
        left += m;
        right += m;
        while left < right {
            if left % 2 == 1 {
                result += tree[left];
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                result += tree[right];
            }
            left /= 2;
            right /= 2;
        }

        result
    }

    let mut cost = 0;
    let mut MOD: i64 = 1_000_000_007;

    for x in instructions {
        cost += (query(&tree, 0, x as usize) as i64)
            .min(query(&tree, x as usize + 1, m as usize) as i64);
        update(&mut tree, x as usize, 1);
    }

    (cost % MOD) as i32
}

pub fn create_sorted_array_bit(instructions: Vec<i32>) -> i32 {
    let m = 100000;
    let mut bit = vec![0; m + 1];

    fn update(bit: &mut Vec<i32>, mut index: i32, v: i32) {
        while index < bit.len() as i32 {
            bit[index as usize] += v;
            index += index & -index;
        }
    }

    fn query(bit: &Vec<i32>, mut index: i32) -> i32 {
        let mut sum = 0;
        while index > 0 {
            sum += bit[index as usize];
            index -= index & -index;
        }
        sum
    }

    fn range_sum(bit: &Vec<i32>, left: i32, right: i32) -> i32 {
        query(bit, right) - query(bit, left - 1)
    }

    const MOD: i64 = 1_000_000_007;
    let mut count = 0;
    for index in instructions {
        count +=
            (range_sum(&bit, 0, index - 1) as i64).min(range_sum(&bit, index + 1, m as i32) as i64);
        update(&mut bit, index, 1);
    }

    (count % MOD) as i32
}
