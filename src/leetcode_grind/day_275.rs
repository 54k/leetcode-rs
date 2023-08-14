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
