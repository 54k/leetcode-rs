package leetcode_grind;

public class Day910 {
    // https://leetcode.com/problems/sort-colors/description/?envType=daily-question&envId=2025-05-17
    static class Solution1 {
        public void sortColors(int[] nums) {
            int p0 = 0, curr = 0;
            int p2 = nums.length - 1;
            int tmp;
            while (curr <= p2) {
                if (nums[curr] == 0) {
                    tmp = nums[p0];
                    nums[p0++] = nums[curr];
                    nums[curr++] = tmp;
                } else if (nums[curr] == 2) {
                    tmp = nums[curr];
                    nums[curr] = nums[p2];
                    nums[p2--] = tmp;
                } else {
                    curr++;
                }
            }
        }
    }

    // https://leetcode.com/problems/bomb-enemy/description/
    static class Solution2 {
        public int maxKilledEnemies(char[][] grid) {
            if (grid.length == 0) {
                return 0;
            }
            int rows = grid.length;
            int cols = grid[0].length;

            int maxCount = 0, rowHits = 0;
            int[] colHits = new int[cols];

            for (int row = 0; row < rows; ++row) {
                for (int col = 0; col < cols; ++col) {

                    if (col == 0 || grid[row][col - 1] == 'W') {
                        rowHits = 0;
                        for (int k = col; k < cols; ++k) {
                            if (grid[row][k] == 'W') {
                                break;
                            } else if (grid[row][k] == 'E') {
                                rowHits += 1;
                            }
                        }
                    }

                    if (row == 0 || grid[row - 1][col] == 'W') {
                        colHits[col] = 0;
                        for (int k = row; k < rows; ++k) {
                            if (grid[k][col] == 'W') {
                                break;
                            } else if (grid[k][col] == 'E') {
                                colHits[col] += 1;
                            }
                        }
                    }

                    if (grid[row][col] == '0') {
                        maxCount = Math.max(maxCount, rowHits + colHits[col]);
                    }
                }
            }
            return maxCount;
        }
    }
}
