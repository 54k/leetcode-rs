// https://leetcode.com/problems/spiral-matrix/description/
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut matrix = matrix;
    const VISITED: i32 = 101;

    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    const DIR: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

    let mut current_direction = 0;

    let mut change_direction = 0;

    let mut row = 0;
    let mut col = 0;

    let mut result = vec![];
    result.push(matrix[0][0]);
    matrix[0][0] = VISITED;

    while change_direction < 2 {
        while row + DIR[current_direction][0] >= 0
            && row + DIR[current_direction][0] < rows
            && col + DIR[current_direction][1] >= 0
            && col + DIR[current_direction][1] < cols
            && matrix[(row + DIR[current_direction][0]) as usize]
                [(col + DIR[current_direction][1]) as usize]
                != VISITED
        {
            change_direction = 0;

            row = row + DIR[current_direction][0];
            col = col + DIR[current_direction][1];

            result.push(matrix[row as usize][col as usize]);
            matrix[row as usize][col as usize] = VISITED;
        }

        current_direction = (current_direction + 1) % 4;
        change_direction += 1;
    }

    result
}
