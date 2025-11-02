package leetcode_grind;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class Day1079 {
    // https://leetcode.com/problems/count-unguarded-cells-in-the-grid/description/?envType=daily-question&envId=2025-11-02
    static class Solution1 {
        static final int UNGUARDED = 0;
        static final int GUARDED = 1;
        static final int GUARD = 2;
        static final int WALL = 3;

        void markguarded(int row, int col, int[][] grid) {
            // Traverse upwards
            for (int r = row - 1; r >= 0; r--) {
                if (grid[r][col] == WALL || grid[r][col] == GUARD)
                    break;
                grid[r][col] = GUARDED;
            }
            // Traverse downwards
            for (int r = row + 1; r < grid.length; r++) {
                if (grid[r][col] == WALL || grid[r][col] == GUARD)
                    break;
                grid[r][col] = GUARDED;
            }
            // Traverse leftwards
            for (int c = col - 1; c >= 0; c--) {
                if (grid[row][c] == WALL || grid[row][c] == GUARD)
                    break;
                grid[row][c] = GUARDED;
            }
            // Traverse rightwards
            for (int c = col + 1; c < grid[0].length; c++) {
                if (grid[row][c] == WALL || grid[row][c] == GUARD)
                    break;
                grid[row][c] = GUARDED;
            }
        }

        public int countUnguarded(int m, int n, int[][] guards, int[][] walls) {
            int[][] grid = new int[m][n];
            for (int[] guard : guards) {
                grid[guard[0]][guard[1]] = GUARD;
            }

            for (int[] wall : walls) {
                grid[wall[0]][wall[1]] = WALL;
            }

            for (int[] guard : guards) {
                markguarded(guard[0], guard[1], grid);
            }

            int count = 0;
            for (int[] row : grid) {
                for (int cell : row) {
                    if (cell == UNGUARDED)
                        count++;
                }
            }
            return count;
        }
    }

    // https://leetcode.com/problems/maximum-number-of-ones/description/?envType=weekly-question&envId=2025-11-01
    static class Solution2 {
        public int maximumNumberOfOnes(int width, int height, int sideLength, int maxOnes) {
            List<Integer> count = new ArrayList<>();
            for (int r = 0; r < sideLength; ++r) {
                for (int c = 0; c < sideLength; c++) {
                    count.add((1 + (width - 1 - c) / sideLength) * (1 + (height - 1 - r) / sideLength));
                }
            }

            count.sort(Comparator.reverseOrder());
            int answer = 0;
            for (int i = 0; i < maxOnes; i++) {
                answer += count.get(i);
            }
            return answer;
        }
    }

}
