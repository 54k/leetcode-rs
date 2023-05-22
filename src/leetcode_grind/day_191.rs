// https://leetcode.com/problems/falling-squares/
// https://leetcode.com/problems/falling-squares/editorial/
pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

// https://leetcode.com/problems/shifting-letters-ii/
pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
    let letters = ('a'..='z').collect::<Vec<_>>();
    let s = s.chars().map(|x| x as i32 - 'a' as i32).collect::<Vec<_>>();

    let mut diff = vec![s[0]];
    for i in 1..s.len() {
        diff.push(s[i] - s[i - 1]);
    }
    let n = diff.len();

    for sh in shifts {
        let (left, right, val) = (
            sh[0] as usize,
            sh[1] as usize,
            if sh[2] == 0 { -1 } else { 1 },
        );
        diff[left] += val;
        diff[left] %= 26;
        if right + 1 < n {
            diff[right + 1] -= val;
            diff[right] %= 26;
        }
    }

    for i in 1..n {
        diff[i] += diff[i - 1] % 26;
        diff[i] %= 26;
    }

    diff.into_iter()
        .map(|x| letters[((x + 26) % 26) as usize])
        .collect()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test506() {
        println!(
            "{}",
            shifting_letters(
                "abc".to_string(),
                vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]
            )
        );

        println!(
            "{}",
            shifting_letters("dztz".to_string(), vec![vec![0, 0, 0], vec![1, 1, 1]])
        );
    }
}
