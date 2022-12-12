#[allow(dead_code)]
pub fn jump(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }

    let mut nums = nums.into_iter().map(|x| x as usize).collect::<Vec<usize>>();
    let mut jumps = 0;
    let mut i = 0;

    while i < nums.len() - 1 {
        let mut max = 0;
        let mut next_first = i + 1;
        let next_last = i + nums[i];

        if next_last >= nums.len() - 1 {
            return jumps + 1;
        }

        while next_first <= next_last {
            if max <= nums[next_first] + next_first {
                max = nums[next_first] + next_first;
                i = next_first;
            }
            next_first += 1;
        }
        jumps += 1;
    }

    jumps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test105() {
        println!("{}", jump(vec![2, 3, 1, 1, 4]));
        println!("{}", jump(vec![2, 3, 0, 1, 4]));
        println!("{}", jump(vec![1]));
        println!("{}", jump(vec![2, 0]));
        println!("{}", jump(vec![1, 2, 3]));
        println!("{}", jump(vec![1, 1, 1, 1]));
        println!("{}", jump(vec![1, 2, 1, 1, 1]));
        println!("{}", jump(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 0]));
    }
}
