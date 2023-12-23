// https://leetcode.com/problems/maximize-value-of-function-in-a-ball-passing-game/description/
pub fn get_max_function_value(receiver: Vec<i32>, k: i64) -> i64 {
    let n = receiver.len();
    // let mut maxd = 0;
    // while (1<<maxd) <= n {
    //     maxd += 1;
    // }
    let maxd = 35;

    let mut kthparent = vec![vec![0; maxd]; n];
    let mut cost = vec![vec![0; maxd]; n];

    for i in 0..maxd {
        for j in 0..n {
            if i == 0 {
                kthparent[j][i] = receiver[j] as usize;
                cost[j][i] = receiver[j] as i64;
            } else {
                kthparent[j][i] = kthparent[kthparent[j][i - 1]][i - 1];
                cost[j][i] = cost[j][i - 1] + cost[kthparent[j][i - 1]][i - 1];
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        let mut node = i;
        let mut sum = i as i64;

        for j in 0..maxd {
            if ((1 << j) & k) != 0 {
                sum += cost[node][j];
                node = kthparent[node][j];
            }
        }

        ans = ans.max(sum);
    }
    ans
}
