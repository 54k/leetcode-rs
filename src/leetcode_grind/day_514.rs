// https://leetcode.com/problems/reveal-cards-in-increasing-order/description
pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();

    let mut deck = deck;
    deck.sort();

    for i in 0..deck.len() {
        queue.push_back(i);
    }

    let mut res = vec![0; deck.len()];

    for i in 0..deck.len() {
        res[queue.pop_front().unwrap()] = deck[i];
        if queue.len() > 0 {
            let front = queue.pop_front().unwrap();
            queue.push_back(front);
        }
    }

    res
}

// https://leetcode.com/problems/4-keys-keyboard/description
pub fn max_a(n: i32) -> i32 {
    if n < 3 {
        return n;
    }
    let mut dp = vec![0; n as usize + 1];
    for i in 0..=n as usize {
        dp[i] = i;
    }

    for i in 0..=n as usize - 3 {
        for j in i + 3..=(n as usize).min(i + 6) {
            dp[j] = dp[j].max((j - i - 1) * dp[i]);
        }
    }
    dp[n as usize] as i32
}
