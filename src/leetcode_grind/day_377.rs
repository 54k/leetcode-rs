// https://leetcode.com/problems/maximum-number-of-coins-you-can-get/description
pub fn max_coins_1(piles: Vec<i32>) -> i32 {
    let (mut a, mut b, mut c) = (0, 0, 0);
    let mut piles = piles;
    piles.sort();
    use std::collections::VecDeque;
    let mut queue = piles.into_iter().collect::<VecDeque<_>>();

    while !queue.is_empty() {
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

pub fn max_coins_4(piles: Vec<i32>) -> i32 {
    fn sort(piles: &mut Vec<i32>) {
        let mut max = *piles.iter().max().unwrap();
        let mut cnt = 0;
        while max > 0 {
            cnt += 1;
            max /= 10;
        }

        for i in 0..cnt {
            let mut buckets = vec![vec![]; 10];
            for num in piles.iter() {
                let k = (*num / 10_i32.pow(i) % 10) as usize;
                buckets[k].push(*num);
            }

            let mut z = 0;
            for j in 0..=9 {
                for &num in &buckets[j] {
                    piles[z] = num;
                    z += 1;
                }
            }
        }
    }

    let mut res = 0;
    let mut piles = piles;
    sort(&mut piles);

    let n = piles.len();
    let mut i = n / 3;
    while i < n {
        res += piles[i];
        i += 2;
    }

    res
}
