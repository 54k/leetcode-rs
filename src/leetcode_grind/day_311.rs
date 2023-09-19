// https://leetcode.com/problems/find-the-duplicate-number/description
pub fn find_duplicate_bin_search(nums: Vec<i32>) -> i32 {
    let mut low = 1;
    let mut high = nums.len() - 1;

    let mut duplicate = -1;

    while low <= high {
        let cur = (low + high) / 2;

        let mut count = 0;
        for &num in &nums {
            if num <= cur as i32 {
                count += 1;
            }
        }

        if count > cur {
            duplicate = cur as i32;
            high = cur - 1;
        } else {
            low = cur + 1;
        }
    }

    duplicate
}

pub fn find_duplicate_sum_of_set_bits(nums: Vec<i32>) -> i32 {
    fn calc_max_bit(mut num: i32) -> i32 {
        let mut bits = 0;
        while num > 0 {
            num /= 2;
            bits += 1;
        }
        bits
    }

    fn find_max_num(nums: &Vec<i32>) -> i32 {
        let mut max_num = 0;
        for i in 0..nums.len() {
            max_num = max_num.max(nums[i]);
        }
        max_num
    }

    let mut duplicate = 0;
    let n = nums.len() - 1;
    let max_num = find_max_num(&nums);
    let max_bit = calc_max_bit(max_num);

    for bit in 0..max_bit {
        let mask = 1 << bit;
        let mut base_count = 0;
        let mut nums_count = 0;

        for i in 0..=n as i32 {
            if i & mask > 0 {
                base_count += 1;
            }

            if (nums[i as usize] & mask) > 0 {
                nums_count += 1;
            }
        }

        if nums_count > base_count {
            duplicate |= mask;
        }
    }

    duplicate
}

pub fn find_duplicate_floyd_algo(nums: Vec<i32>) -> i32 {
    let mut tortoise = nums[0];
    let mut hare = nums[0];

    while {
        tortoise = nums[tortoise as usize];
        hare = nums[nums[hare as usize] as usize];
        tortoise != hare
    } {}

    tortoise = nums[0];
    while tortoise != hare {
        tortoise = nums[tortoise as usize];
        hare = nums[hare as usize];
    }

    hare
}
