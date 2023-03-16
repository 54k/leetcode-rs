// https://leetcode.com/problems/sort-an-array/
pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    fn partition(arr: &mut [i32], mut left: usize, mut right: usize) -> i32 {
        let p_point = right;
        let pivot = arr[right];
        while left < right {
            while left < right && arr[left] <= pivot {
                left += 1;
            }
            while left < right && arr[right] >= pivot {
                right -= 1;
            }
            if left < right {
                arr.swap(left, right);
            }
        }
        arr.swap(p_point, right);
        right as i32
    }
    fn quick_sort(nums: &mut Vec<i32>, left: i32, right: i32) {
        if left >= right {
            return;
        }
        let mid = partition(nums, left as usize, right as usize);
        quick_sort(nums, left, mid - 1);
        quick_sort(nums, mid + 1, right);
    }
    let n = nums.len();
    quick_sort(&mut nums, 0_i32, n as i32 - 1);
    nums
}

// https://leetcode.com/problems/132-pattern/
// https://leetcode.com/problems/132-pattern/editorial/
pub fn find132pattern(nums: Vec<i32>) -> bool {
    fn brute_force(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                for k in j + 1..nums.len() {
                    if nums[k] > nums[i] && nums[j] > nums[k] {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn optimized_brute_force(nums: Vec<i32>) -> bool {
        let mut min_i = i32::MAX;
        for j in 0..nums.len() - 1 {
            min_i = min_i.min(nums[j]);
            for k in j + 1..nums.len() {
                if nums[k] > min_i && nums[j] > nums[k] {
                    return true;
                }
            }
        }
        false
    }
    fn interval_search(nums: Vec<i32>) -> bool {
        let mut intervals = vec![];
        let mut i = 1;
        let mut s = 0;
        while i < nums.len() {
            if nums[i] < nums[i - 1] {
                if s < i - 1 {
                    intervals.push((nums[s], nums[i - 1]));
                }
                s = i;
            }
            for j in 0..intervals.len() {
                if nums[i] > intervals[j].0 && nums[i] < intervals[j].1 {
                    return true;
                }
            }
            i += 1;
        }
        false
    }
    fn mono_stack(nums: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut mins = vec![0; nums.len()];
        mins[0] = nums[0];
        for i in 1..nums.len() {
            mins[i] = mins[i - 1].min(nums[i]);
        }
        for i in (0..nums.len()).rev() {
            if mins[i] < nums[i] {
                while !stack.is_empty() && stack[stack.len() - 1] <= mins[i] {
                    stack.pop();
                }
                if !stack.is_empty() && stack[stack.len() - 1] < nums[i] {
                    return true;
                }
                stack.push(nums[i]);
            }
        }
        false
    }
    mono_stack(nums)
}

// https://leetcode.com/problems/maximal-rectangle/
pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut ans = 0;

    let mut hist = vec![vec![]; m];
    for i in (0..n).rev() {
        let mut stack = vec![];
        for j in 0..m {
            let height = if matrix[i][j] == '1' {
                hist[j].last().unwrap_or(&0) + 1
            } else {
                0
            };
            hist[j].push(height);
        }

        let mut heights = vec![0; m];
        for j in 0..m {
            heights[j] = *hist[j].last().unwrap();
        }

        for right in 0..=hist.len() {
            while !stack.is_empty()
                && (right == hist.len() || heights[*stack.last().unwrap()] >= heights[right])
            {
                let mid = stack.pop().unwrap();
                let h = heights[mid];
                let left = stack.last().map(|x| *x as i32).unwrap_or(-1);
                let square = h * (right as i32 - left - 1);
                ans = ans.max(square);
            }
            stack.push(right);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test310() {
        println!("{:?}", sort_array(vec![5, 2, 3, 1])); // [1,2,3,5]
        println!("{:?}", sort_array(vec![5, 1, 1, 2, 0, 0])); // [0,0,1,1,2,5]
        println!("{:?}", sort_array(vec![5, 4, 3, 2, 1])); // [1,2,3,4,5]
    }

    #[test]
    fn test311() {
        println!("{}", find132pattern(vec![1, 0, 1, -4, -3])); // false
        println!("{}", find132pattern(vec![1, 2, 3, 4])); // false
        println!("{}", find132pattern(vec![3, 1, 4, 2])); // true
        println!("{}", find132pattern(vec![-1, 3, 2, 0])); // true
    }

    #[test]
    fn test312() {
        println!(
            "{}",
            maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ])
        );
    }
}
