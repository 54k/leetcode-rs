// https://leetcode.com/problems/number-complement/description/?envType=daily-question&envId=2024-08-22
pub fn find_complement_i(mut num: i32) -> i32 {
    let mut todo = num;
    let mut bit = 1;
    while todo != 0 {
        num = num ^ bit;
        bit = bit << 1;
        todo = todo >> 1;
    }
    num
}

pub fn find_complement_ii(mut num: i32) -> i32 {
    let mut bm = num;
    bm |= bm >> 1;
    bm |= bm >> 2;
    bm |= bm >> 4;
    bm |= bm >> 8;
    bm |= bm >> 16;
    bm ^ num
}
