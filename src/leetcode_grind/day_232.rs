// https://leetcode.com/problems/maximum-number-of-achievable-transfer-requests/description/
pub fn maximum_requests_backtracking(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    fn dfs(
        requests: &Vec<Vec<i32>>,
        indegree: &mut Vec<i32>,
        n: usize,
        index: usize,
        count: i32,
        answer: &mut i32,
    ) {
        if index == requests.len() {
            for i in 0..n {
                if indegree[i] != 0 {
                    return;
                }
            }
            *answer = (*answer).max(count);
            return;
        }

        // Consider this request, increment and decrement for the buildings involved.
        indegree[requests[index][0] as usize] -= 1;
        indegree[requests[index][1] as usize] += 1;
        // Move on to the next request and also increment the count of requests.
        dfs(requests, indegree, n, index + 1, count + 1, answer);
        // Backtrack to the previous values to move back to the original state before the second recursion.
        indegree[requests[index][0] as usize] += 1;
        indegree[requests[index][1] as usize] -= 1;
        // Ignore this request and move on to the next request without incrementing the count.
        dfs(requests, indegree, n, index + 1, count, answer);
    }

    let mut indegree = vec![0; n as usize];
    let mut answer = 0;
    dfs(&requests, &mut indegree, n as usize, 0, 0, &mut answer);
    answer
}

pub fn maximum_requests_bitcount(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;
    for mask in 0..(1 << requests.len()) as i32 {
        let mut indegree = vec![0; n as usize];
        let mut pos = requests.len() - 1;
        // Number of set bits representing the requests we will consider.
        let bit_count = mask.count_ones();

        // If the request count we're going to consider is less than the maximum request
        // We have considered without violating the constraints; then we can return it cannot be the answer.
        if bit_count <= answer {
            continue;
        }

        // For all the 1's in the number, update the array indegree for the building it involves.
        let mut curr = mask;
        while curr > 0 {
            if (curr & 1) == 1 {
                indegree[requests[pos][0] as usize] -= 1;
                indegree[requests[pos][1] as usize] += 1;
            }
            curr >>= 1;
            pos -= 1;
        }

        let mut flag = true;
        // Check if it doesn;t violates the constraints
        for i in 0..n as usize {
            if indegree[i] != 0 {
                flag = false;
                break;
            }
        }

        if flag {
            answer = bit_count;
        }
    }
    answer as i32
}

// https://leetcode.com/problems/maximum-white-tiles-covered-by-a-carpet/description/
// https://leetcode.com/problems/maximum-white-tiles-covered-by-a-carpet/solutions/2038534/sliding-window/
// https://leetcode.com/problems/maximum-white-tiles-covered-by-a-carpet/solutions/2038512/c-prefix-sum-binary-search-sorting-simple-and-detailed-explanation/
pub fn maximum_white_tiles(mut tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
    tiles.sort();
    let mut res = 0;
    let (mut i, mut j) = (0, 0);
    let mut cover = 0;

    while res < carpet_len && i < tiles.len() {
        if tiles[j][0] + carpet_len > tiles[i][1] {
            cover += tiles[i][1] - tiles[i][0] + 1;
            res = res.max(cover);
            i += 1;
        } else {
            let partial = 0.max(tiles[j][0] + carpet_len - tiles[i][0]);
            res = res.max(cover + partial);
            cover -= (tiles[j][1] - tiles[j][0] + 1);
            j += 1;
        }
    }
    res
}
