// https://leetcode.com/problems/buy-two-chocolates/description
pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let mut fm = i32::MAX;
    let mut sm = i32::MAX;

    for p in prices {
        if p <= fm {
            std::mem::swap(&mut fm, &mut sm);
            fm = p;
        } else if p <= sm {
            sm = p;
        }
    }
    if fm + sm <= money {
        money - fm - sm
    } else {
        money
    }
}
