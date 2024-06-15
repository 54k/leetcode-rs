// https://leetcode.com/problems/put-boxes-into-the-warehouse-i/description/
pub fn max_boxes_in_warehouse(boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
    let n = warehouse.len();
    let mut boxes = boxes;
    boxes.sort();
    let mut wh = vec![i32::MAX; warehouse.len()];
    wh[0] = warehouse[0];
    for i in 1..n {
        wh[i] = wh[i - 1].min(warehouse[i]);
    }
    let mut ans = 0;
    let mut j = 0;
    for i in (0..n).rev() {
        if j < boxes.len() && boxes[j] <= wh[i] {
            ans += 1;
            j += 1;
        }
    }

    ans
}
