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
