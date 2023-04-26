use std::collections::HashMap;
use std::ops::{Add, Sub};

// https://leetcode.com/problems/add-digits/
pub fn add_digits(mut num: i32) -> i32 {
    let mut digital_root = 0;
    while num > 0 {
        digital_root += num % 10;
        num /= 10;
        if num == 0 && digital_root > 9 {
            num = digital_root;
            digital_root = 0;
        }
    }
    digital_root
}

// https://leetcode.com/problems/maximum-product-after-k-increments/
pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    for n in nums {
        heap.push(-n);
    }
    for _ in 0..k {
        let new_num = -heap.pop().unwrap() + 1;
        heap.push(-new_num);
    }
    const MOD: i64 = 1000000007;
    let mut ans = 1;
    while !heap.is_empty() {
        ans = (ans * -heap.pop().unwrap() as i64) % MOD;
    }
    ans as i32
}

// https://leetcode.com/problems/find-the-divisibility-array-of-a-string/
pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
    let mut ans = vec![];
    let mut current = 0;
    for ch in word.chars() {
        current = (10 * current + (ch as i64 - '0' as i64)) % m as i64;
        if current == 0 {
            ans.push(1);
        } else {
            ans.push(0);
        }
    }
    ans
}

// https://leetcode.com/problems/reverse-words-in-a-string-iii/
pub fn reverse_words(s: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    let mut left = 0;
    let len = s.len();
    for i in 0..len {
        if s[i] == ' ' || i == len - 1 {
            s[left..if i == len - 1 { i + 1 } else { i }].reverse();
            left = i + 1;
        }
    }
    s.into_iter().collect()
}

// https://leetcode.com/problems/find-the-highest-altitude/description/
pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut cur = 0;
    for i in 0..gain.len() {
        cur += gain[i];
        ans = ans.max(cur);
    }
    ans
}

// https://leetcode.com/problems/find-pivot-index/description/
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum = nums.iter().copied().map(|x| x as i64).sum::<i64>();
    let mut cur = 0;
    for i in 0..nums.len() {
        if sum - cur - nums[i] as i64 == cur {
            return i as i32;
        }
        cur += nums[i] as i64;
    }
    -1
}

// https://leetcode.com/problems/minimum-number-of-people-to-teach/description/
// https://leetcode.com/problems/minimum-number-of-people-to-teach/solutions/1031102/java-o-m-n-solution/
pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
    use std::collections::{HashMap, HashSet};
    let mut language_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (i, language) in languages.iter().enumerate() {
        language_map
            .entry(i as i32 + 1)
            .or_insert(HashSet::new())
            .extend(language);
    }
    let mut already_can = vec![false; friendships.len()];
    for i in 1..=n {
        for j in 0..friendships.len() {
            if language_map[&friendships[j][0]].contains(&i)
                && language_map[&friendships[j][1]].contains(&i)
            {
                already_can[j] = true;
            }
        }
    }
    let mut min_teach = i32::MAX;
    for i in 1..=n {
        let mut teach_set = HashSet::new();
        for j in 0..friendships.len() {
            if already_can[j] {
                continue;
            }
            if !language_map[&friendships[j][0]].contains(&i) {
                teach_set.insert(friendships[j][0]);
            }
            if !language_map[&friendships[j][1]].contains(&i) {
                teach_set.insert(friendships[j][1]);
            }
        }
        min_teach = min_teach.min(teach_set.len() as i32);
    }
    min_teach
}

// https://leetcode.com/problems/reverse-only-letters/description/
pub fn reverse_only_letters(s: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        while i < j && !s[i].is_alphabetic() {
            i += 1;
        }
        while i < j && !s[j].is_alphabetic() {
            j -= 1;
        }
        s.swap(i, j);
        i += 1;
        j -= 1;
    }
    s.into_iter().collect()
}

// https://leetcode.com/problems/path-crossing/
pub fn is_path_crossing(path: String) -> bool {
    use std::collections::{HashMap, HashSet};
    let dir = HashMap::from([
        ('N', Pos((0, 1))),
        ('S', Pos((0, -1))),
        ('E', Pos((1, 0))),
        ('W', Pos((-1, 0))),
    ]);
    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
    struct Pos((i32, i32));
    impl Add<&Self> for Pos {
        type Output = Pos;
        fn add(self, rhs: &Self) -> Self::Output {
            Pos((self.0 .0 + rhs.0 .0, self.0 .1 + rhs.0 .1))
        }
    }
    let mut pos = Pos((0, 0));
    let mut vis = HashSet::from([pos.clone()]);
    for p in path.chars() {
        pos = pos + &dir[&p];
        // println!("{:?}", pos);
        if vis.contains(&pos) {
            return true;
        }
        vis.insert(pos.clone());
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test449() {
        println!("{}", add_digits(38)); // 2
    }

    #[test]
    fn test450() {
        println!("{}", maximum_product(vec![6, 3, 3, 2], 2)); // 216
    }

    #[test]
    fn test451() {
        println!("{:?}", divisibility_array("998244353".to_string(), 3)); // 1,1,0,0,0,1,1,0,0
    }

    #[test]
    fn test452() {
        println!(
            "{}",
            reverse_words("Let's take LeetCode contest".to_string())
        ); // "s'teL ekat edoCteeL tsetnoc"
    }

    #[test]
    fn test453() {
        println!("{}", largest_altitude(vec![-5, 1, 5, 0, -7])); // 1
    }

    #[test]
    fn test454() {
        println!("{}", pivot_index(vec![1, 7, 3, 6, 5, 6])); // 3
    }

    #[test]
    fn test455() {
        println!(
            "{}",
            minimum_teachings(
                2,
                vec![vec![1], vec![2], vec![1, 2]],
                vec![vec![1, 2], vec![1, 3], vec![2, 3]]
            )
        ); // 1
    }

    #[test]
    fn test456() {
        println!(
            "{}",
            reverse_only_letters("Test1ng-Leet=code-Q!".to_string())
        );
    }
    #[test]
    fn test457() {
        println!("{}", is_path_crossing("NESWW".to_string()));
        println!("{}", is_path_crossing("NES".to_string()));
    }
}
