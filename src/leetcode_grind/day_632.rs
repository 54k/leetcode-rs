// https://leetcode.com/problems/regions-cut-by-slashes/description/?envType=daily-question&envId=2024-08-10
pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
    fn find(parent_array: &mut Vec<i32>, node: i32) -> i32 {
        if parent_array[node as usize] == -1 {
            return node;
        }
        parent_array[node as usize] = find(parent_array, parent_array[node as usize]);
        parent_array[node as usize]
    }

    fn union(parent_array: &mut Vec<i32>, node1: usize, node2: usize) -> i32 {
        let parent1 = find(parent_array, node1 as i32);
        let parent2 = find(parent_array, node2 as i32);
        if parent1 == parent2 {
            return 1;
        }

        parent_array[parent2 as usize] = parent1;
        0
    }

    let grid_size = grid.len();
    let grid = grid
        .into_iter()
        .map(|x| x.chars().clone().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let points_per_side = grid_size + 1;
    let total_points = points_per_side * points_per_side;

    let mut parent_array = vec![-1; total_points];

    for i in 0..points_per_side {
        for j in 0..points_per_side {
            if i == 0 || j == 0 || i == points_per_side - 1 || j == points_per_side - 1 {
                let point = i * points_per_side + j;
                parent_array[point] = 0;
            }
        }
    }

    parent_array[0] = -1;
    let mut region_count = 1;

    for i in 0..grid_size {
        for j in 0..grid_size {
            if grid[i][j] == '/' {
                let top_right = i * points_per_side + (j + 1);
                let bottom_left = (i + 1) * points_per_side + j;
                region_count += union(&mut parent_array, top_right, bottom_left);
            } else if grid[i][j] == '\\' {
                let top_left = i * points_per_side + j;
                let bottom_right = (i + 1) * points_per_side + (j + 1);
                region_count += union(&mut parent_array, top_left, bottom_right);
            }
        }
    }

    region_count
}
