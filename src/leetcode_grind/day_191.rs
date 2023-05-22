// https://leetcode.com/problems/falling-squares/
// https://leetcode.com/problems/falling-squares/editorial/
pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

// https://leetcode.com/problems/shifting-letters-ii/
pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
    todo!()
}

// https://leetcode.com/problems/top-k-frequent-elements/description/
pub fn top_k_frequent(nums: Vec<i32>, mut k: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut m1 = HashMap::new();
    for i in 0..nums.len() {
        *m1.entry(nums[i]).or_insert(0) += 1;
    }
    let mut m2 = HashMap::new();
    for (num, count) in &m1 {
        m2.entry(*count).or_insert(vec![]).push(*num);
    }
    let mut ans = vec![];
    for freq in (1..=nums.len() as i32).rev() {
        while m2.contains_key(&freq) && k > 0 {
            if k == 0 {
                return ans;
            }
            ans.push(m2.get_mut(&freq).unwrap().pop().unwrap());
            k -= 1;
            if m2[&freq].is_empty() {
                m2.remove(&freq);
            }
        }
    }
    ans
}
