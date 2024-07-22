// https://leetcode.com/problems/sort-the-people/description/?envType=daily-question&envId=2024-07-22
pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let n = names.len();
    let mut idx = vec![0; n];
    for i in 0..n {
        idx[i] = i;
    }
    idx.sort_by_key(|x| heights[*x]);
    idx.reverse();

    let mut ans = vec![];
    for i in idx {
        ans.push(names[i].clone());
    }
    ans
}
