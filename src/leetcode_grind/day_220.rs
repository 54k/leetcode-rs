// https://leetcode.com/problems/k-radius-subarray-averages/description/
pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ans = vec![-1; nums.len()];
    let (mut start, mut sum) = (0, 0);
    for (end, &num) in nums.iter().enumerate() {
        sum += num as i64;
        if end as i32 >= 2 * k {
            ans[start + k as usize] = (sum / (2 * k + 1) as i64) as i32;
            sum -= nums[start] as i64;
            start += 1;
        }
    }
    ans
}

// https://leetcode.com/problems/bitwise-and-of-numbers-range/description/
pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
    let mut shift = 0;
    while left < right {
        left >>= 1;
        right >>= 1;
        shift += 1;
    }
    right << shift
}

// https://leetcode.com/problems/design-memory-allocator/
struct Allocator {
    memory: Vec<i32>,
}

impl Allocator {
    fn new(n: i32) -> Self {
        Self {
            memory: vec![0; n as usize],
        }
    }
    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let mut cnt = 0;
        for i in 0..self.memory.len() {
            if self.memory[i] == 0 {
                cnt += 1;
                if cnt == size {
                    for j in (i as i32 - size + 1..=i as i32).rev() {
                        self.memory[j as usize] = m_id;
                    }
                    return i as i32 - size + 1;
                }
            } else {
                cnt = 0;
            }
        }
        -1
    }
    fn free(&mut self, m_id: i32) -> i32 {
        let mut ans = 0;
        for i in 0..self.memory.len() {
            if self.memory[i] == m_id {
                ans += 1;
                self.memory[i] = 0;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut alloc = Allocator::new(10);
        println!("{}", alloc.allocate(1, 1));
        println!("{}", alloc.allocate(1, 2));
        println!("{}", alloc.allocate(1, 3));

        println!("{}", alloc.free(2));

        println!("{}", alloc.allocate(3, 4));
        println!("{}", alloc.allocate(1, 1));
        println!("{}", alloc.allocate(1, 1));

        println!("{}", alloc.free(1));

        println!("{}", alloc.allocate(10, 2));
        println!("{}", alloc.free(7));
    }
}
