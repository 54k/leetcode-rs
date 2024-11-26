// https://leetcode.com/problems/find-champion-ii/description/?envType=daily-question&envId=2024-11-26
pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut in_degree = vec![0; n as usize];
    for edge in edges {
        in_degree[edge[1] as usize] += 1;
    }

    let mut champ = -1;
    let mut champ_count = 0;

    for i in 0..n as usize {
        if in_degree[i] == 0 {
            champ_count += 1;
            champ = i as i32;
        }
    }

    if champ_count > 1 {
        -1
    } else {
        champ as i32
    }
}
