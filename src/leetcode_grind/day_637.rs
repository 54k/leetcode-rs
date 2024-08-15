// https://leetcode.com/problems/lemonade-change/description/
pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut five_dollars_bills = 0;
    let mut ten_dollars_bills = 0;

    for customer_bill in bills {
        if customer_bill == 5 {
            five_dollars_bills += 1;
        } else if customer_bill == 10 {
            if five_dollars_bills > 0 {
                five_dollars_bills -= 1;
                ten_dollars_bills += 1;
            } else {
                return false;
            }
        } else {
            if ten_dollars_bills > 0 && five_dollars_bills > 0 {
                five_dollars_bills -= 1;
                ten_dollars_bills -= 1;
            } else if five_dollars_bills >= 3 {
                five_dollars_bills -= 3;
            } else {
                return false;
            }
        }
    }
    true
}
