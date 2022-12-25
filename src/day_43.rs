#[allow(dead_code)]
pub fn answer_queries(mut nums: Vec<i32>, mut queries: Vec<i32>) -> Vec<i32> {
    fn bin_search(arr: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0i32;
        let mut right = arr.len() as i32 - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            match arr[mid as usize].cmp(&target) {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Equal => return mid + 1,
                std::cmp::Ordering::Greater => right = mid - 1,
            }
        }

        if arr[left as usize] > target {
            left
        } else {
            left + 1
        }
    }

    nums.sort();
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }

    for i in 0..queries.len() {
        let target = queries[i];
        queries[i] = bin_search(&nums, target);
    }

    queries
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test127() {
        println!("{:?}", answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21])); // [2, 3, 4]
        println!("{:?}", answer_queries(vec![2, 3, 4, 5], vec![1])); // [0]
    }
}
