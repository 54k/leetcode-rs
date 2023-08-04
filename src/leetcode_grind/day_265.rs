struct SpatialTree {
    tree: Vec<i32>,
    n: usize,
}

impl SpatialTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n * 2 + 1],
            n,
        }
    }

    fn sum(&self, mut a: usize, mut b: usize) -> i32 {
        a += self.n;
        b += self.n;

        let mut sum = 0;
        while a <= b {
            println!("cur a {} cur b {}", a, b);
            if a % 2 == 1 {
                sum += self.tree[a];
                a += 1;
            }
            if b % 2 == 0 {
                sum += self.tree[b];
                b -= 1;
            }
            a /= 2;
            b /= 2;
            println!("next a {} next b {}", a, b);
        }

        sum
    }

    fn add(&mut self, mut k: usize, v: i32) {
        k += self.n;
        self.tree[k] += v;
        k /= 2;
        while k >= 1 {
            self.tree[k] = self.tree[k * 2] + self.tree[k * 2 + 1];
            k /= 2;
        }
    }
}

// https://leetcode.com/problems/reverse-linked-list/description/
pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut h) = head.take() {
        let tmp = h.next.take();
        h.next = prev;
        prev = Some(h);
        head = tmp;
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_tree() {
        let mut tree = SpatialTree::new(8);
        let arr = vec![5, 8, 6, 3, 2, 7, 2, 6];
        for (i, n) in arr.into_iter().enumerate() {
            tree.add(i, n);
        }

        let sum = tree.sum(3, 4);
        println!("sum {}", sum);
    }
}

// https://leetcode.com/problems/special-binary-string/description/
pub fn make_largest_special(s: String) -> String {
    if s.len() == 0 {
        return s;
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut anchor = 0;
    let mut bal = 0;
    let mut mountains = vec![];

    for i in 0..s.len() {
        bal += if s[i] == '1' { 1 } else { -1 };
        if bal == 0 {
            let m = format!(
                "1{}0",
                make_largest_special(s[anchor + 1..i].iter().copied().collect())
            );
            mountains.push(m);
            anchor = i + 1;
        }
    }

    mountains.sort();
    mountains.reverse();
    mountains.into_iter().collect()
}

// https://leetcode.com/problems/number-of-good-binary-strings/description/
pub fn good_binary_strings(
    min_length: i32,
    max_length: i32,
    one_group: i32,
    zero_group: i32,
) -> i32 {
    const MOD: i64 = 1000000007;
    let mut dp = vec![0; max_length as usize + 1]; // number of good bs of length i
    dp[0] = 1i64;
    // there is one empty goodBS since 0 is a multiple of all numbers
    let mut result = 0i64;

    for i in 1..=max_length as usize {
        if i >= one_group as usize {
            dp[i] = (dp[i] + dp[i - one_group as usize]) % MOD;
        }
        if i >= zero_group as usize {
            dp[i] = (dp[i] + dp[i - zero_group as usize]) % MOD;
        }

        if i >= min_length as usize {
            result = (result + dp[i]) % MOD;
        }
    }
    (result % MOD) as i32
}
