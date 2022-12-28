#[allow(dead_code)]
pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;
    let mut h = BinaryHeap::new();

    for pile in piles {
        h.push(pile);
    }

    for _ in 0..k {
        let i = h.pop().unwrap();
        let r = i / 2;
        h.push(i - r);
    }

    let mut ans = 0;
    for hh in h {
        ans += hh;
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test131() {
        println!("{}", min_stone_sum(vec![5, 4, 9], 2)); // 12
        println!("{}", min_stone_sum(vec![4, 3, 6, 7], 3)); // 12
    }
}
