// [1,3,3,2,5,5,6,7] --> find median

// taks = [run(), run()]
// while true {
//     tasks.pop()(execute);
// }

// table fio, birth

// | id | fio str | birth   dec |
//    1   BK       3123253523
//
// select fio from users where birth between 13123 and 21312312;
// create index on birth
//
// [0,0,0,0,0,0,0] -> entry val: next
// hash(key) % bucket_size
//
// shard[id % shards_count]

fn find_mid(nums: Vec<i32>) -> i32 {
    fn solution1(nums: Vec<i32>) -> i32 {
        let mut prefix = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix[i + 1] = prefix[i] + nums[i];
        }
        for i in 0..nums.len() {
            let right = prefix[nums.len()] - prefix[i + 1];
            let left = prefix[i];
            if right == left {
                return i as i32;
            }
        }
        -1
    }
    fn solution2(nums: Vec<i32>) -> i32 {
        let mut s1 = 0;
        let mut s2 = nums.iter().copied().sum::<i32>();
        for i in 0..nums.len() {
            if s1 == s2 - nums[i] {
                return i as i32;
            }
            s1 += nums[i];
            s2 -= nums[i];
        }
        -1
    }
    fn solution3(nums: Vec<i32>) -> i32 {
        // only for positive numbers
        use std::cmp::*;
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut s1 = 0;
        let mut s2 = 0;

        while left < right {
            match s1.cmp(&s2) {
                Ordering::Equal => {
                    s1 += nums[left];
                    s2 += nums[right];
                    left += 1;
                    right -= 1;
                }
                Ordering::Less => {
                    s1 += nums[left];
                    left += 1;
                }
                Ordering::Greater => {
                    s2 += nums[right];
                    right -= 1;
                }
            }
        }
        if s1 == s2 {
            left as i32
        } else {
            -1
        }
    }
    solution2(nums)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        println!("{}", find_mid(vec![3, 4, 2, 2, 5])); // 2
        println!("{}", find_mid(vec![3, 4, 2, 3, 1, 2, 5, 5])); // 4
        println!("{}", find_mid(vec![3, 2, 1])); // -1
        println!("{}", find_mid(vec![-3, -2, -1, -5])); // 2
    }
}
