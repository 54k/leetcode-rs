// pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
//     todo!();
//     let mut k = k as i64;
//     const MOD: i64 = 1000000007;
//     let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
//     let n = nums.len();

//     let upper = nums.iter().copied().max().unwrap() + 1;

//     let mut prime = vec![true; upper as usize];
//     let mut prime_score = vec![0; upper as usize];
//     prime[0] = false;
//     prime[1] = false;

//     for i in 2..upper as usize {
//         if prime[i] {
//             for j in i..upper as usize {
//                 prime_score[j] += 1;
//                 prime[j] = false;
//             }
//         }
//     }

//     let mut next_greater_element = vec![n as i64; n];
//     let mut stack = vec![];
//     for i in (0..n).rev() {
//         while !stack.is_empty()
//             && prime_score[nums[i] as usize] >= prime_score[nums[*stack.last().unwrap()] as usize]
//         {
//             stack.pop();
//         }
//         next_greater_element[i] = if stack.is_empty() {
//             n as i64
//         } else {
//             *stack.last().unwrap() as i64
//         };
//         stack.push(i);
//     }

//     let mut prev_greater_or_eq_element = vec![-1; n];
//     stack.clear();
//     for i in 0..n {
//         while !stack.is_empty()
//             && prime_score[nums[i] as usize] > prime_score[nums[*stack.last().unwrap()] as usize]
//         {
//             stack.pop();
//         }
//         prev_greater_or_eq_element[i] = if stack.is_empty() {
//             -1
//         } else {
//             *stack.last().unwrap() as i64
//         };
//         stack.push(i);
//     }

//     fn pow(mut x: i64, n: i64) -> i64 {
//         let mut res = 1;
//         while x > 0 {
//             if n % 2 == 1 {
//                 res = (res * x) % MOD;
//             }
//             x = (x * x) % MOD;
//             x /= 2;
//         }
//         res
//     }

//     let mut res = 1;
//     let mut tuples = vec![];
//     for i in 0..n {
//         tuples.push((nums[i], i));
//     }
//     tuples.sort_by_key(|x| x.0);
//     tuples.reverse();

//     for i in 0..n {
//         let (num, idx) = (tuples[i].0, tuples[i].1);
//         let ops = (k as i64).min(
//             (idx as i64 - prev_greater_or_eq_element[idx])
//                 * (next_greater_element[idx] - idx as i64),
//         );
//         res = ((res * pow(num as i64, ops as i64)) % MOD) as i64;
//         k -= ops;
//         if k == 0 {
//             return res as i32;
//         }
//     }
//     res as i32
// }

// https://leetcode.com/problems/sort-colors/description/
pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut p0 = 0;
    let mut p2 = nums.len();
    let mut i = 0;
    while i < p2 {
        let el = nums[i];
        if el == 0 {
            nums.swap(i, p0);
            p0 += 1;
            i += 1;
        } else if el == 2 {
            p2 -= 1;
            nums.swap(i, p2);
        } else {
            i += 1;
        }
    }
}

// https://leetcode.com/problems/query-kth-smallest-trimmed-number/description/
pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![];

    for q in queries {
        let mut pq = vec![];
        for i in 0..n {
            pq.push((
                nums[i][nums[i].len() - q[1] as usize..].to_string(),
                i as i32,
            ));
        }
        pq.sort();
        res.push(pq[q[0] as usize - 1].1);
    }
    res
}

// https://leetcode.com/problems/maximum-gap/description/
pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
    if nums.is_empty() || nums.len() < 2 {
        return 0;
    }

    let max_val = nums.iter().copied().max().unwrap();
    let mut exp = 1usize; // 1, 10, 100, 1000
    let radix = 10; // base 10 system

    let mut aux = vec![0; nums.len()];

    while max_val as usize / exp > 0 {
        // go through all digits from LSD to MSD
        let mut count = vec![0; radix];

        for i in 0..nums.len() {
            count[(nums[i] as usize / exp) % radix] += 1;
        }

        for i in 1..count.len() {
            count[i] += count[i - 1];
        }

        for i in (0..nums.len()).rev() {
            count[(nums[i] as usize / exp) % radix] -= 1;
            aux[count[(nums[i] as usize / exp) % radix]] = nums[i];
        }

        for i in 0..nums.len() {
            nums[i] = aux[i];
        }

        exp *= 10;
    }

    let mut max_gap = 0;
    for i in 0..nums.len() - 1 {
        max_gap = max_gap.max(nums[i + 1] - nums[i]);
    }

    max_gap
}

// https://leetcode.com/problems/find-k-closest-elements/description/
pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    pub fn find_closest_elements_naive(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut sorted_arr = vec![];
        for num in arr {
            sorted_arr.push(num);
        }

        sorted_arr.sort_by(|a, b| (a - x).abs().cmp(&(b - x).abs()));

        let mut sorted_arr = sorted_arr[..k as usize]
            .into_iter()
            .copied()
            .collect::<Vec<_>>();

        sorted_arr.sort();
        sorted_arr
    }

    pub fn find_closest_elements_bin_search_two_pointers(
        arr: Vec<i32>,
        k: i32,
        x: i32,
    ) -> Vec<i32> {
        let mut result = vec![];

        if arr.len() == k as usize {
            return arr;
        }

        let mut left = 0;
        let mut right = arr.len() as i32;

        while left < right {
            let mid = (left + right) / 2;
            if arr[mid as usize] >= x {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        right = left;
        left -= 1;

        while right - left - 1 < k {
            if left == -1 {
                right += 1;
                continue;
            }

            if right == arr.len() as i32
                || (arr[left as usize] - x).abs() <= (arr[right as usize] - x)
            {
                left -= 1;
            } else {
                right += 1;
            }
        }

        for i in left + 1..right {
            result.push(arr[i as usize]);
        }

        result
    }

    pub fn find_closest_elements_binary_search(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = arr.len() as i32 - k;

        while left < right {
            let mid = (left + right) / 2;
            if x - arr[mid as usize] <= arr[mid as usize + k as usize] - x {
                right = mid
            } else {
                left = mid + 1
            }
        }

        arr[left as usize..left as usize + k as usize]
            .into_iter()
            .copied()
            .collect()
    }

    find_closest_elements_binary_search(arr, k, x)
}

// https://leetcode.com/problems/sort-the-matrix-diagonally/description/
pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    pub fn diagonal_sort_hash_map(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        let (m, n) = (mat.len(), mat[0].len());
        let mut map = HashMap::new();

        for r in 0..m {
            for c in 0..n {
                let diag = r as i32 - c as i32;
                map.entry(diag)
                    .or_insert(BinaryHeap::new())
                    .push(-mat[r][c]);
            }
        }

        for r in 0..m {
            for c in 0..n {
                let diag = r as i32 - c as i32;
                mat[r][c] = -map.get_mut(&diag).unwrap().pop().unwrap();
            }
        }
        mat
    }

    pub fn diagonal_sort_one_by_one(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn sort_diagonal(row: usize, col: usize, mat: &mut Vec<Vec<i32>>) {
            use std::collections::BinaryHeap;
            let (m, n) = (mat.len(), mat[0].len());
            let mut diagonal = BinaryHeap::new();
            let diagonal_len = (m - row).min(n - col);

            for i in 0..diagonal_len {
                diagonal.push(-mat[row + i][col + i]);
            }
            for i in 0..diagonal_len {
                mat[row + i][col + i] = -diagonal.pop().unwrap();
            }
        }

        let (m, n) = (mat.len(), mat[0].len());
        for row in 0..m {
            sort_diagonal(row, 0, &mut mat);
        }

        for col in 0..n {
            sort_diagonal(0, col, &mut mat);
        }

        mat
    }

    diagonal_sort_one_by_one(mat)
}

#[test]
fn test_find_closest_elements() {
    let res = find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3);
    println!("{:?}", res);
}

#[test]
fn test_sort_diag_matrix() {
    let res = diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]);
    println!("{:?}", res);
}
