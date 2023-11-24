// https://leetcode.com/problems/maximum-number-of-coins-you-can-get/description
pub fn max_coins_1(piles: Vec<i32>) -> i32 {
    let (mut a, mut b, mut c) = (0, 0, 0);
    let mut piles = piles;
    piles.sort();
    use std::collections::VecDeque;
    let mut queue = piles.into_iter().collect::<VecDeque<_>>();

    while !queue.is_empty() {
        let n = queue.len();
        a += queue.pop_back().unwrap();
        b += queue.pop_back().unwrap();
        c += queue.pop_front().unwrap();
    }

    b
}

pub fn max_coins_2(piles: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut piles = piles;
    piles.sort();
    let mut i = 0 as i32;
    let mut j = piles.len() as i32 - 2;
    while i < j {
        res += piles[j as usize];
        i += 1;
        j -= 2;
    }
    res
}

pub fn max_coins_3(piles: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut piles = piles;
    piles.sort();
    let n = piles.len();
    let mut i = n / 3;
    while i < n {
        res += piles[i];
        i += 2;
    }
    res
}
