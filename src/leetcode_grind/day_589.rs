// https://leetcode.com/problems/find-center-of-star-graph/description/?envType=daily-question&envId=2024-06-27
pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut degree = HashMap::new();
    for e in &edges {
        *degree.entry(e[0]).or_insert(0) += 1;
        *degree.entry(e[1]).or_insert(0) += 1;
    }
    for (node, v) in degree {
        if v == edges.len() {
            return node;
        }
    }
    -1
}
