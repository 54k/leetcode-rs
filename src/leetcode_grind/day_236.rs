// https://leetcode.com/problems/matchsticks-to-square/
pub fn makesquare(matchsticks: Vec<i32>) -> bool {
    pub fn makesquare_dfs(mut matchsticks: Vec<i32>) -> bool {
        if matchsticks.len() == 0 {
            return false;
        }

        let perimeter = matchsticks.iter().copied().sum::<i32>();

        let possible_square_side = perimeter / 4;
        if possible_square_side * 4 != perimeter {
            return false;
        }

        matchsticks.sort();
        matchsticks.reverse();

        let mut sums = vec![0; 4];

        fn dfs(
            index: usize,
            sums: &mut Vec<i32>,
            square_side: i32,
            matchsticks: &Vec<i32>,
        ) -> bool {
            if index == matchsticks.len() {
                return sums[0] == sums[1] && sums[1] == sums[2] && sums[2] == sums[3];
            }

            let element = matchsticks[index];

            for i in 0..4 {
                if sums[i] + element <= square_side {
                    sums[i] += element;
                    if dfs(index + 1, sums, square_side, matchsticks) {
                        return true;
                    }
                    sums[i] -= element;
                }
            }

            false
        }

        dfs(0, &mut sums, possible_square_side, &matchsticks)
    }
    makesquare_dfs(matchsticks)
}

pub fn makesquare_2(mut matchsticks: Vec<i32>) -> bool {
    todo!()
    // let mut memo = vec![vec![None; 4]; 1 << matchsticks.len() + 1];

    // matchsticks.sort();
    // matchsticks.reverse();

    // let p = matchsticks.iter().copied().sum::<i32>();
    // let possible_side = p / 4;

    // if possible_side * 4 != p {
    //     return false;
    // }

    // fn rec(
    //     mask: usize,
    //     mut sides_done: usize,
    //     matchsticks: &Vec<i32>,
    //     memo: &mut Vec<Vec<Option<bool>>>,
    //     possible_side: i32,
    // ) -> bool {
    //     let mut total = 0;
    //     let n = matchsticks.len();

    //     let sides_key = sides_done;

    //     if let Some(v) = memo[mask][sides_key] {
    //         return v;
    //     }

    //     // Find out the sum of matchsticks used till now.
    //     for i in 0..n {
    //         if mask & (i << 1) == 1 {
    //             total += matchsticks[i];
    //         }
    //     }

    //     if total > 0 && total % possible_side == 0 {
    //         sides_done += 1;
    //     }

    //     if sides_done == 3 {
    //         return true;
    //     }

    //     let mut ans = false;

    //     let c = total / possible_side;

    //     // Remaining vlength in the current partially formed side.
    //     let rem = possible_side * (c + 1) - total;

    //     // Try out all remaining options (that are valid)
    //     for i in 0..n {
    //         if matchsticks[i] <= rem && (mask & (1 << i) == 0) {
    //             if rec(
    //                 mask | (1 << i),
    //                 sides_done,
    //                 matchsticks,
    //                 memo,
    //                 possible_side,
    //             ) {
    //                 ans = true;
    //                 break;
    //             }
    //         }
    //     }

    //     memo[mask][sides_done] = Some(true);
    //     ans
    // }

    // rec(0, 0, &matchsticks, &mut memo, possible_side)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        println!("{:?}", makesquare_2(vec![1, 1, 2, 2, 2]));
        println!("{:?}", makesquare_2(vec![3, 3, 3, 3, 4]));
    }
}
