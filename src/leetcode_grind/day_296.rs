// https://leetcode.com/problems/sort-an-array/description/
pub fn sort_array_qsort(mut nums: Vec<i32>) -> Vec<i32> {
    extern "C" {
        fn random() -> i32;
    }

    fn sort(nums: &mut Vec<i32>, i: i32, j: i32) {
        fn partition(nums: &mut Vec<i32>, i: usize, j: usize) -> i32 {
            let rpivot = unsafe { random() as usize % (j - i + 1) + i };
            nums.swap(j, rpivot);

            let ppoint = nums[j];

            let mut left = i;
            for k in i..j {
                if nums[k] <= ppoint {
                    nums.swap(left, k);
                    left += 1;
                }
            }
            nums.swap(left, j);
            left as i32
        }

        if i >= j {
            return;
        }

        let ppoint = partition(nums, i as usize, j as usize);
        sort(nums, i, ppoint - 1);
        sort(nums, ppoint, j);
    }

    let n = nums.len();
    sort(&mut nums, 0, n as i32 - 1);
    nums
}

pub fn sort_array_merge_sort_i(mut nums: Vec<i32>) -> Vec<i32> {
    fn merge_sort(nums: &mut Vec<i32>, tmp_arr: &mut Vec<i32>, left: usize, right: usize) {
        fn merge(
            nums: &mut Vec<i32>,
            tmp_arr: &mut Vec<i32>,
            left: usize,
            mid: usize,
            right: usize,
        ) {
            let start1 = left;
            let start2 = mid + 1;
            let n1 = mid - left + 1;
            let n2 = right - mid;

            for i in 0..n1 {
                tmp_arr[start1 + i] = nums[start1 + i];
            }
            for i in 0..n2 {
                tmp_arr[start2 + i] = nums[start2 + i];
            }

            let mut i = 0;
            let mut j = 0;
            let mut k = left;
            while i < n1 && j < n2 {
                if tmp_arr[start1 + i] <= tmp_arr[start2 + j] {
                    nums[k] = tmp_arr[start1 + i];
                    i += 1;
                } else {
                    nums[k] = tmp_arr[start2 + j];
                    j += 1;
                }
                k += 1;
            }

            while i < n1 {
                nums[k] = tmp_arr[start1 + i];
                i += 1;
                k += 1;
            }
            while j < n2 {
                nums[k] = tmp_arr[start2 + j];
                j += 1;
                k += 1;
            }
        }

        if left >= right {
            return;
        }

        let mid = (left + right) / 2;
        merge_sort(nums, tmp_arr, left, mid);
        merge_sort(nums, tmp_arr, mid + 1, right);
        merge(nums, tmp_arr, left, mid, right);
    }

    let n = nums.len();
    let mut tmp_arr = vec![0; nums.len()];
    merge_sort(&mut nums, &mut tmp_arr, 0, n - 1);
    nums
}

pub fn sort_array_merge_sort_ii(mut nums: Vec<i32>) -> Vec<i32> {
    fn merge_sort(nums: &mut Vec<i32>) {
        fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
            let mut merged = Vec::new();
            let mut i = 0;
            let mut j = 0;
            while i < left.len() && j < right.len() {
                if left[i] <= right[j] {
                    merged.push(left[i]);
                    i += 1
                } else {
                    merged.push(right[j]);
                    j += 1;
                }
            }
            while i < left.len() {
                merged.push(left[i]);
                i += 1;
            }
            while j < right.len() {
                merged.push(right[j]);
                j += 1;
            }
            merged
        }

        if nums.len() <= 1 {
            return;
        }

        let mid = nums.len() / 2;
        let mut left = vec![];
        let mut right = vec![];
        for k in 0..mid {
            left.push(nums[k]);
        }
        for k in mid..nums.len() {
            right.push(nums[k]);
        }

        merge_sort(&mut left);
        merge_sort(&mut right);

        *nums = merge(&left, &right);
    }

    merge_sort(&mut nums);
    nums
}

pub fn sort_array_counting_sort(mut nums: Vec<i32>) -> Vec<i32> {
    fn count_sort(nums: &mut Vec<i32>) {
        use std::collections::HashMap;
        let mut counts = HashMap::new();
        let mut min_value = nums[0];
        let mut max_value = nums[0];

        for &n in nums.iter() {
            *counts.entry(n).or_insert(0) += 1;
            min_value = min_value.min(n);
            max_value = max_value.max(n);
        }

        let mut index = 0;
        for val in min_value..=max_value {
            while *counts.get(&val).unwrap_or(&0) > 0 {
                nums[index] = val;
                index += 1;
                *counts.entry(val).or_insert(0) -= 1;
            }
        }
    }

    count_sort(&mut nums);
    nums
}

pub fn sort_array_radix_sort(mut nums: Vec<i32>) -> Vec<i32> {
    fn radix_sort(nums: &mut Vec<i32>) {
        fn bucket_sort(nums: &mut Vec<i32>, place_value: i32) {
            let mut buckets = vec![vec![]; 10];

            for &val in nums.iter() {
                let mut digit = val.abs() / place_value;
                digit = digit % 10;
                buckets[digit as usize].push(val);
            }

            let mut index = 0;
            for digit in 0..10 {
                for &val in buckets[digit].iter() {
                    nums[index] = val;
                    index += 1;
                }
            }
        }

        let mut max_element = nums[0];
        for val in nums.iter() {
            max_element = val.abs().max(max_element);
        }

        let mut max_digits = 0;
        while max_element > 0 {
            max_digits += 1;
            max_element /= 10;
        }

        let mut place_value = 1;
        for _ in 0..max_digits {
            bucket_sort(nums, place_value);
            place_value *= 10;
        }

        let mut negatives = vec![];
        let mut positives = vec![];

        for &val in nums.iter() {
            if val < 0 {
                negatives.push(val);
            } else {
                positives.push(val);
            }
        }
        negatives.reverse();

        let mut index = 0;
        for val in negatives {
            nums[index] = val;
            index += 1;
        }
        for val in positives {
            nums[index] = val;
            index += 1;
        }
    }

    radix_sort(&mut nums);
    nums
}

pub fn sort_array_heap_sort(mut nums: Vec<i32>) -> Vec<i32> {
    fn heap_sort(nums: &mut Vec<i32>) {
        fn heapify(nums: &mut Vec<i32>, n: usize, i: usize) {
            let mut largest = i;
            let left = 2 * i + 1;
            let right = 2 * i + 2;

            if left < n && nums[left] > nums[largest] {
                largest = left;
            }

            if right < n && nums[right] > nums[largest] {
                largest = right;
            }

            if largest != i {
                nums.swap(i, largest);
                heapify(nums, n, largest);
            }
        }

        let n = nums.len();
        for i in (0..n / 2).rev() {
            heapify(nums, n, i);
        }

        for i in (0..n).rev() {
            nums.swap(0, i);
            heapify(nums, i, 0);
        }
    }

    heap_sort(&mut nums);
    nums
}

// https://leetcode.com/problems/ternary-expression-parser/description/
pub fn parse_ternary(mut expression: String) -> String {
    let is_atom = |win: &str| -> bool {
        let win = &win.chars().collect::<Vec<_>>()[..];
        ['F', 'T'].contains(&win[0])
            && '?' == win[1]
            && (('0'..='9').contains(&win[2]) || ['F', 'T'].contains(&win[2]))
            && ':' == win[3]
            && (('0'..='9').contains(&win[4]) || ['F', 'T'].contains(&win[4]))
    };

    let solve_atom = |atom: &str| -> String {
        let atom = &atom.chars().collect::<Vec<_>>()[..];
        if atom[0] == 'T' {
            atom[2..3].iter().copied().collect::<String>()
        } else {
            atom[4..5].iter().copied().collect::<String>()
        }
    };

    while expression.len() != 1 {
        let mut j = expression.len() - 1;
        while !is_atom(&expression[j - 4..=j]) {
            j -= 1;
        }

        let first = expression[..j - 4].to_string();
        let second = solve_atom(&expression[j - 4..=j]);
        let third = &expression[j + 1..expression.len()];
        expression = first + second.as_str() + third;
    }

    expression
}
