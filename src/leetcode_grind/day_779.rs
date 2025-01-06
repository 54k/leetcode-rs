// https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/description/?envType=daily-question&envId=2025-01-06
pub fn min_operations(boxes: String) -> Vec<i32> {
    let n = boxes.len();
    let boxes = boxes.chars().collect::<Vec<_>>();
    let mut answer = vec![0; n];

    let mut balls_to_left = 0;
    let mut moves_to_left = 0;

    let mut balls_to_right = 0;
    let mut moves_to_right = 0;

    for i in 0..n {
        answer[i] += moves_to_left;
        balls_to_left += boxes[i] as i32 - '0' as i32;
        moves_to_left += balls_to_left;

        let j = n - 1 - i;
        answer[j] += moves_to_right;
        balls_to_right += boxes[j] as i32 - '0' as i32;
        moves_to_right += balls_to_right;
    }
    answer
}
