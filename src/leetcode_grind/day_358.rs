// https://leetcode.com/problems/find-the-winner-of-an-array-game/description
pub fn get_winner_simulate_queue(arr: Vec<i32>, k: i32) -> i32 {
    use std::collections::VecDeque;
    let mut max_element = arr[0];
    let mut queue = VecDeque::new();
    for i in 1..arr.len() {
        max_element = max_element.max(arr[i]);
        queue.push_back(arr[i]);
    }

    let mut curr = arr[0];
    let mut winstreak = 0;

    while !queue.is_empty() {
        let opponent = queue.pop_front().unwrap();

        if curr > opponent {
            queue.push_back(opponent);
            winstreak += 1;
        } else {
            queue.push_back(curr);
            curr = opponent;
            winstreak = 1;
        }

        if winstreak == k || curr == max_element {
            return curr;
        }
    }

    return -1;
}

pub fn get_winner_arr(arr: Vec<i32>, k: i32) -> i32 {
    let mut max_element = arr[0];
    for i in 1..arr.len() {
        max_element = max_element.max(arr[i]);
    }

    let mut curr = arr[0];
    let mut streak = 0;

    for i in 1..arr.len() {
        let opponent = arr[i];
        if curr > opponent {
            streak += 1;
        } else {
            curr = opponent;
            streak = 1;
        }

        if streak == k || curr == max_element {
            return curr;
        }
    }
    -1
}

// https://leetcode.com/problems/defanging-an-ip-address/description/
pub fn defang_i_paddr(address: String) -> String {
    let address = address.chars().collect::<Vec<_>>();
    let mut result = vec!['x'; address.len() + 6];
    let mut ri = result.len() - 1;
    for i in (0..address.len()).rev() {
        if address[i] == '.' {
            result[ri] = ']';
            ri -= 1;
            result[ri] = '.';
            ri -= 1;
            result[ri] = '[';
        } else {
            result[ri] = address[i];
        }
        ri -= 1;
    }
    result.into_iter().collect()
}
