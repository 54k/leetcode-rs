// https://leetcode.com/problems/jump-game-iii/description/
pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
    fn rec(arr: &mut Vec<i32>, cur_pos: i32) -> bool {
        if cur_pos < 0 || cur_pos >= arr.len() as i32 {
            return false;
        }
        if arr[cur_pos as usize] == -1 {
            return false;
        }
        if arr[cur_pos as usize] == 0 {
            return true;
        }
        let t = arr[cur_pos as usize];
        arr[cur_pos as usize] = -1;
        let ans = rec(arr, cur_pos + t) || rec(arr, cur_pos - t);
        arr[cur_pos as usize] = t;
        ans
    }

    rec(&mut arr, start)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test211() {
        println!("{}", can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5)); // true
        println!("{}", can_reach(vec![3, 0, 2, 1, 2], 2)); // false
    }
}
