// https://leetcode.com/problems/sign-of-the-product-of-an-array/
pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut min = 0;
    for num in nums {
        if num < 0 {
            min += 1;
        } else if num == 0 {
            return 0;
        }
    }
    if min % 2 == 1 {
        -1
    } else {
        1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test474() {
        println!("{}", array_sign(vec![-1, 1, -1, 1, -1]));
    }
}
