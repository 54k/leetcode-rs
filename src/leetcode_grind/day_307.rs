// https://leetcode.com/problems/min-cost-to-connect-all-points/description
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut mst_cost = 0;
    let mut edges_used = 0;

    let mut in_mst = vec![false; n];

    let mut min_dist = vec![i32::MAX; n];
    min_dist[0] = 0;

    while edges_used < n {
        let mut curr_min_edge = i32::MAX;
        let mut curr_node = 0;

        for node in 0..n {
            if !in_mst[node] && curr_min_edge > min_dist[node] {
                curr_min_edge = min_dist[node];
                curr_node = node;
            }
        }

        mst_cost += curr_min_edge;
        edges_used += 1;
        in_mst[curr_node] = true;

        for next_node in 0..n {
            let dist = (points[curr_node][0] - points[next_node][0]).abs()
                + (points[curr_node][1] - points[next_node][1]).abs();
            if !in_mst[next_node] && min_dist[next_node] > dist {
                min_dist[next_node] = dist;
            }
        }
    }

    mst_cost
}
