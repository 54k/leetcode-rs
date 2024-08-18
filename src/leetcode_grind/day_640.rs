// https://leetcode.com/problems/ugly-number-ii/description/?envType=daily-question&envId=2024-08-18
pub fn nth_ugly_number(n: i32) -> i32 {
    let mut ugly_numbers = vec![0; n as usize];
    ugly_numbers[0] = 1;

    let mut index_multiple_of_2 = 0;
    let mut index_multiple_of_3 = 0;
    let mut index_multiple_of_5 = 0;
    
    let mut next_multiple_of_2 = 2;
    let mut next_multiple_of_3 = 3;
    let mut next_multiple_of_5 = 5;

    for i in 1..n as usize {
        let next_ugly_number = next_multiple_of_2.min(
            next_multiple_of_3.min(
                next_multiple_of_5
            ) 
        );

        ugly_numbers[i] =  next_ugly_number;

        if next_ugly_number == next_multiple_of_2 {
            index_multiple_of_2 += 1;
            next_multiple_of_2 = ugly_numbers[index_multiple_of_2] * 2;
        }

        if next_ugly_number == next_multiple_of_3 {
            index_multiple_of_3 += 1;
            next_multiple_of_3 = ugly_numbers[index_multiple_of_3] * 3;
        }

        if next_ugly_number == next_multiple_of_5 {
            index_multiple_of_5 += 1;
            next_multiple_of_5 = ugly_numbers[index_multiple_of_5] * 5;
        }

    }

    ugly_numbers[n as usize - 1]
}