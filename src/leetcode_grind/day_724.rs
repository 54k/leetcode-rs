// https://leetcode.com/problems/intersection-of-three-sorted-arrays/description/?envType=weekly-question&envId=2024-11-08
pub fn arrays_intersection(arr1: Vec<i32>, arr2: Vec<i32>, arr3: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    let mut p1 = 0;
    let mut p2 = 0;
    let mut p3 = 0;

    while p1 < arr1.len() && p2 < arr2.len() && p3 < arr3.len() {
        if arr1[p1] == arr2[p2] && arr2[p2] == arr3[p3] {
            ans.push(arr1[p1]);
            p1 += 1;
            p2 += 1;
            p3 += 1;
        } else {
            if arr1[p1] < arr2[p2] {
                p1 += 1;
            } else if arr2[p2] < arr3[p3] {
                p2 += 1;
            } else {
                p3 += 1;
            }
        }
    }
    ans
}
