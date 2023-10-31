// https://leetcode.com/problems/find-the-original-array-of-prefix-xor/description
pub fn find_array_additional_memory(pref: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![pref[0]];
    for i in 1..pref.len() {
        ans.push(pref[i] ^ pref[i - 1]);
    }
    ans
}

pub fn find_array_in_place(pref: Vec<i32>) -> Vec<i32> {
    let mut pref = pref;
    for i in (1..pref.len()).rev() {
        pref[i] ^= pref[i-1];
    }
    pref
}
