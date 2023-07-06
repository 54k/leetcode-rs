// https://leetcode.com/problems/matchsticks-to-square/
pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
    if matchsticks.len() == 0 {
        return false;
    }

    let n = matchsticks.len();
    let perimeter = matchsticks.iter().copied().sum::<i32>();

    let possible_square_side = perimeter / 4;
    if possible_square_side * 4 != perimeter {
        return false;
    }

    matchsticks.sort();
    matchsticks.reverse();

    let mut sums = vec![0; 4];

    fn dfs(index: usize, sums: &mut Vec<i32>, square_side: i32, matchsticks: &Vec<i32>) -> bool {
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
