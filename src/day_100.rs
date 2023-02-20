// https://leetcode.com/problems/sort-array-by-increasing-frequency/
pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
    const SHIFT: i32 = 100;
    let mut freq = vec![0; 201];
    for num in nums.iter() {
        let idx = (num + SHIFT) as usize;
        freq[idx] += 1;
    }
    nums.sort_by(|a, b| {
        let freq_a = freq[(a + SHIFT) as usize];
        let freq_b = freq[(b + SHIFT) as usize];
        if freq_a == freq_b {
            b.cmp(a)
        } else {
            freq_a.cmp(&freq_b)
        }
    });
    nums
}

// https://leetcode.com/problems/percentage-of-letter-in-string/
pub fn percentage_letter(s: String, letter: char) -> i32 {
    let len = s.len();
    let mut count = 0;
    for ch in s.chars() {
        if ch == letter {
            count += 1;
        }
    }
    count * 100 / len as i32
}

// https://leetcode.com/problems/top-k-frequent-elements/description/
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::cmp::*;
    use std::collections::*;
    let mut frequencies = HashMap::new();
    let mut heap = BinaryHeap::new();
    for num in nums {
        *frequencies.entry(num).or_insert(0) += 1;
    }
    for (num, freq) in frequencies {
        heap.push(Reverse((freq, num)));
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    heap.into_vec()
        .into_iter()
        .map(|Reverse((_, v))| v)
        .collect()
}

// https://leetcode.com/problems/kth-largest-element-in-an-array/description/
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    fn with_heap(nums: Vec<i32>, mut k: i32) -> i32 {
        use std::collections::*;
        let mut heap = BinaryHeap::from(nums);
        while k > 1 {
            heap.pop().unwrap();
            k -= 1;
        }
        heap.pop().unwrap()
    }
    fn with_counting_sort(nums: Vec<i32>, mut k: i32) -> i32 {
        const PAD: i32 = 10000;
        let mut sorted = vec![0; PAD as usize * 2 + 1];
        nums.into_iter()
            .for_each(|x| sorted[(x + PAD) as usize] += 1);
        let mut ans = i32::MAX;
        let mut i = PAD as usize * 2;
        while k > 0 {
            if sorted[i] > 0 {
                k -= 1;
                sorted[i] -= 1;
                ans = i as i32 - PAD;
                if sorted[i] == 0 {
                    i -= 1;
                }
            } else {
                i -= 1;
            }
        }
        ans
    }
    with_counting_sort(nums, k)
}

// https://leetcode.com/problems/find-the-middle-index-in-array/description/
pub fn find_middle_index(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.iter().sum::<i32>();
    for i in 0..nums.len() {
        if right - left == nums[i] {
            return i as i32;
        } else {
            left += nums[i];
            right -= nums[i];
        }
    }
    -1
}
// https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/description/
// https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/solutions/260885/c-6-lines-o-n-target-sum/
pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
    let target = arr.iter().sum::<i32>();
    if target % 3 != 0 {
        return false;
    }

    let target = target / 3;
    let mut idx1 = -1;
    let mut idx2 = -1;

    let mut sum = 0;
    for i in 0..arr.len() {
        sum += arr[i];
        if idx1 == -1 && sum == target {
            idx1 = i as i32;
        } else if idx1 != -1 && sum == target * 2 {
            idx2 = i as i32;
            break;
        }
    }
    idx1 != -1 && idx2 != -1 && idx2 as usize != arr.len() - 1
}

// https://leetcode.com/problems/wiggle-sort-ii/solutions/77677/o-n-o-1-after-median-virtual-indexing/
// https://en.wikipedia.org/wiki/Dutch_national_flag_problem#Pseudocode
pub fn wiggle_sort(nums: &mut Vec<i32>) {
    nums.sort();
    let len = nums.len();
    let mid = (len + 1) / 2;
    let (first_half, second_half) = nums.split_at_mut(mid);
    first_half.reverse();
    second_half.reverse();
    let mut j = 0;
    let mut k = 0;
    let mut ans = vec![0; len];
    for i in 0..len {
        if i % 2 == 0 {
            ans[i] = first_half[j];
            j += 1;
        } else {
            ans[i] = second_half[k];
            k += 1;
        }
    }
    nums.copy_from_slice(&ans[0..len]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test254() {
        println!("{:?}", frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1])); // [5,-1,4,4,-6,-6,1,1,1]
    }

    #[test]
    fn test255() {
        println!("{}", percentage_letter("foobar".to_string(), 'o')); // 33
        println!("{}", percentage_letter("sgawtb".to_string(), 's')); // 16
        println!("{}", percentage_letter("ktaseevvqmbylnrljrkfngzpicdffuimyzqzipspmdimvgqmyqltmwbbpeakifiadaaaiepbgtanfxnndxpeiqwycgbvlpihtfbd".to_string(), 'p'));
        // 33
    }

    #[test]
    fn test256() {
        println!("{:?}", top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)); //[1,2]
    }

    #[test]
    fn test257() {
        println!("{}", find_kth_largest(vec![1], 1)); // 5
        println!("{}", find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2)); // 5
        println!("{}", find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)); // 4
    }

    #[test]
    fn test258() {
        println!("{}", find_middle_index(vec![2, 3, -1, 8, 4])); // 3
    }

    #[test]
    fn test259() {
        println!("{}", can_three_parts_equal_sum(vec![0, 0, 0, 0])); // true
        println!(
            "{}",
            can_three_parts_equal_sum(vec![6, 1, 1, 13, -1, 0, -10, 20])
        );
        // false
        println!(
            "{}",
            can_three_parts_equal_sum(vec![
                29, 31, 27, -10, -67, 22, 15, -1, -16, -29, 59, -18, 48,
            ])
        ); // true
        println!(
            "{}",
            can_three_parts_equal_sum(vec![
                -6, -5, 1, -3, -5, -10, 6, -2, 7, -7, -1, -2, 1, 9, 10, 8, -1, -2, -7, -2, 5, -9,
                -2, -1, 9, -10, 0, -2, -9, -1, -9, 1, -2, -9, -6, -6, 9, 9, -5, -7, 1, 8
            ])
        ); // false
    }

    #[test]
    fn test260() {
        let mut vec = vec![1, 5, 1, 1, 6, 4];
        wiggle_sort(&mut vec);
        println!("{:?}", vec); // [1,6,1,5,1,4]
        let mut vec = vec![1, 3, 2, 2, 3, 1];
        wiggle_sort(&mut vec);
        println!("{:?}", vec); // [2,3,1,3,1,2]
        let mut vec = vec![1, 1, 2, 1, 2, 2, 1];
        wiggle_sort(&mut vec);
        println!("{:?}", vec); // [2,3,1,3,1,2]
    }
}
