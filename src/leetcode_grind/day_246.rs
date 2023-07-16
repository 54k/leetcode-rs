// https://leetcode.com/problems/smallest-sufficient-team/description/
pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
    use std::collections::HashMap;
    let n = people.len();
    let m = req_skills.len();

    let mut skill_id = HashMap::new();
    for i in 0..m {
        skill_id.insert(req_skills[i].clone(), i);
    }

    let mut skills_mask_of_person = vec![0; n];
    for i in 0..n {
        for skill in &people[i] {
            skills_mask_of_person[i] |= 1 << skill_id[skill];
        }
    }

    let mut dp: Vec<u64> = vec![(1 << n) - 1; 1 << m];
    dp[0] = 0;

    for skill_mask in 1..1 << m {
        for i in 0..n {
            let smaller_skills_mask = skill_mask & !skills_mask_of_person[i];
            if smaller_skills_mask == skill_mask {
                continue;
            }
            let people_mask = dp[smaller_skills_mask] | (1 << i);
            if (people_mask as u64).count_ones() < (dp[skill_mask] as u64).count_ones() {
                dp[skill_mask] = people_mask;
            }
        }
    }

    let answer_mask = dp[(1 << m) - 1];
    let mut ans = vec![];

    for i in 0..n {
        if ((answer_mask >> i) & 1) == 1 {
            ans.push(i as i32);
        }
    }

    ans
}
