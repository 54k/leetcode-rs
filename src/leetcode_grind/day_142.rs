// https://leetcode.com/problems/boats-to-save-people/
// https://leetcode.com/problems/boats-to-save-people/editorial/
pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort();
    let mut ans = 0;
    let mut i = 0;
    let mut j = people.len() as i32 - 1;
    while i <= j {
        ans += 1;
        if people[i as usize] + people[j as usize] <= limit {
            i += 1;
        }
        j -= 1;
    }
    ans
}

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut ans = vec![0; nums.len()];
    for k in (0..nums.len()).rev() {
        let square = if nums[i].abs() < nums[j].abs() {
            j -= 1;
            nums[j + 1]
        } else {
            i += 1;
            nums[i - 1]
        };
        ans[k] = square * square;
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test391() {
        println!("{}", num_rescue_boats(vec![3, 5, 3, 4], 5)); // 4
    }

    #[test]
    fn test392() {
        println!("{:?}", sorted_squares(vec![-4, -1, 0, 3, 10]));
    }
}
