// https://leetcode.com/problems/maximum-number-of-achievable-transfer-requests/description/
pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
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

