// Подотрезок с суммой X.py
// Дан массив целых чисел, нужно найти непустой подотрезок
// (непрерывную подпоследовательность) с заданной суммой X, либо сказать, что это невозможно.
// Для найденного отрезка (если он существует) следует выдать на выход индексы его концов.
// https://leetcode.com/problems/subarray-sum-equals-k/
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    fn running(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum == k {
                    ans += 1;
                }
            }
        }
        ans
    }
    fn optimized(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut ans = 0;
        let mut sum = 0;
        let mut map = HashMap::new();
        map.insert(0, 1);
        for i in 0..nums.len() {
            sum += nums[i];
            if map.contains_key(&(sum - k)) {
                ans += *map.get(&(sum - k)).unwrap();
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        ans
    }
    optimized(nums, k)
}

// Минимальное произведение ???
// https://leetcode.com/problems/maximum-product-subarray/
pub fn max_product(nums: Vec<i32>) -> i32 {
    fn brute(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                let mut prod = 1;
                for k in i..=j {
                    prod *= nums[k];
                }
                ans = ans.max(prod);
            }
        }
        ans
    }
    fn optimized(nums: Vec<i32>) -> i32 {
        // store the result that is the max we have found so far
        let mut ans = nums[0];
        // imax/imin stores the max/min product of
        // subarray that ends with the current number A[i]
        let mut min = ans;
        let mut max = ans;
        for i in 1..nums.len() {
            // multiplied by a negative makes big number smaller, small number bigger
            // so we redefine the extremums by swapping them
            if nums[i] < 0 {
                std::mem::swap(&mut min, &mut max);
            }
            // max/min product for the current number is either the current number itself
            // or the max/min by the previous number times the current one
            max = nums[i].max(max * nums[i]);
            min = nums[i].min(min * nums[i]);
            ans = ans.max(max);
        }
        ans
    }
    optimized(nums)
}

// Вертикальная ось симметрии.py
// Дан массив точек с целочисленными координатами (x, y).
// Определить, существует ли вертикальная прямая,
// делящая точки на 2 симметричных относительно этой прямой множества.
// Note: Для удобства точку можно представлять не как массив [x, y], а как объект {x, y}

// https://leetcode.com/discuss/interview-experience/365500/amazon-sde1-santiago-aug-2019-offer

// The thing is to create a map, which stores for each value of Y a sorted array of X values for that Y.
// So for each of this Y entries in the map, we can go checking with two pointers
// (one that starts at the beginning and the other at the end) their middle point.
// We store this point and check if the next calls have the same middle point.
// If every two points have the same middle point, it's symmetric.
fn is_vert_sym(points: Vec<(i32, i32)>) -> bool {
    fn leetcode_approach(points: Vec<(i32, i32)>) -> bool {
        // approach:
        // store X coords in the map, with Ys counts
        // find MAX_X --> MAX_X - point.X will find symmetric X
        // find Y and decrement counter, if counter == 0 remove key, if Y map is empty, remove X
        // check if map is empty - then symmetry found
        use std::collections::*;
        let mut map = HashMap::new();
        let mut max_x = i32::MIN;

        for i in 0..points.len() {
            max_x = max_x.max(points[i].0);
            *map.entry(points[i].0)
                .or_insert_with(HashMap::new)
                .entry(points[i].1)
                .or_insert(0) += 1;
        }

        for i in 0..points.len() {
            let k = max_x - points[i].0; // symmetic X for current point
            if !map.contains_key(&k) || !map.get(&k).unwrap().contains_key(&points[i].1) {
                return false;
            }
            let p = map.get_mut(&k).unwrap();
            let cnt = p.get_mut(&points[i].1).unwrap();
            *cnt -= 1;
            if *cnt == 0 {
                p.remove(&points[i].1);
            }
            if p.is_empty() {
                map.remove(&k);
            }
        }

        map.is_empty()
    }

    fn yandex_approach(points: Vec<(i32, i32)>) -> bool {
        use std::collections::*;
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        for i in 0..points.len() {
            min_x = min_x.min(points[i].0);
            max_x = max_x.max(points[i].0);
        }

        let mut deltas = HashMap::new();

        for (x, y) in points {
            let left_dist = x - min_x;
            let right_dist = max_x - x;
            // println!("{} {}", left_dist, right_dist);
            match left_dist.cmp(&right_dist) {
                std::cmp::Ordering::Less => {
                    *deltas.entry((left_dist, y)).or_insert(0) += 1;
                }
                std::cmp::Ordering::Greater => {
                    *deltas.entry((right_dist, y)).or_insert(0) -= 1;
                }
                _ => {}
            }
        }
        for d in deltas.values() {
            if *d != 0 {
                return false;
            }
        }
        true
    }

    assert_eq!(
        yandex_approach(points.clone()),
        leetcode_approach(points.clone())
    );
    yandex_approach(points)
}

// Поиск подстроки в строке без учета порядка букв.py
// Дан текст T и строка S. Требуется найти подстроку S' в T такую,
// что она совпадает с S с точностью до перестановки букв.
// https://leetcode.com/problems/permutation-in-string/description/
// https://leetcode.com/problems/permutation-in-string/solutions/127729/short-permutation-in-a-long-string/?orderBy=most_relevant
fn check_inclusion(s1: String, s2: String) -> bool {
    let slen1 = s1.len();
    let slen2 = s2.len();
    if slen1 > slen2 {
        return false;
    }
    let s2 = s2.chars().collect::<Vec<_>>();
    let mut hash1 = vec![0; 26];
    for c in s1.chars() {
        hash1[c as usize - 'a' as usize] += 1;
    }
    for i in 0..=(slen2 - slen1) {
        let mut hash2 = vec![0; 26];
        for j in i..i + slen1 {
            hash2[s2[j] as usize - 'a' as usize] += 1;
        }
        if hash1 == hash2 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subarray_sum() {
        println!("{:?}", subarray_sum(vec![1, 1, 1], 2)); // 2
        println!("{:?}", subarray_sum(vec![1, 2, 3], 3)); // 2
        println!("{:?}", subarray_sum(vec![-1, -1, 1], 0)); // 1
    }

    #[test]
    fn test_is_vert_sym() {
        println!(
            "{:?}",
            is_vert_sym(vec![(0, 0), (0, 0), (1, 1), (2, 2), (3, 1), (4, 0), (4, 0)]),
        ); // True
        println!(
            "{:?}",
            is_vert_sym(vec![(0, 0), (0, 0), (1, 1), (2, 2), (3, 1), (4, 0)]),
        ); //False
        println!("{:?}", is_vert_sym(vec![])); // True
        println!("{:?}", is_vert_sym(vec![(0, 0)])); // True
        println!("{:?}", is_vert_sym(vec![(0, 0), (10, 0)])); // True
        println!("{:?}", is_vert_sym(vec![(0, 0), (11, 1)])); //False
        println!("{:?}", is_vert_sym(vec![(0, 0), (1, 0), (3, 0)])); //False
    }

    #[test]
    fn test_check_inclusion() {
        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidbaooo".to_string())
        ); // true
        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidboaoo".to_string())
        ); // false
        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidboaoo".to_string())
        ); // false
    }
}
