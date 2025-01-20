package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Day793 {
    // https://leetcode.com/problems/first-completely-painted-row-or-column/description/?envType=daily-question&envId=2025-01-20
    static class Solution1 {
        public int firstCompleteIndex(int[] arr, int[][] mat) {
            Map<Integer, Integer> numToIndex = new HashMap<>();
            for (int i = 0; i < arr.length; i++) {
                numToIndex.put(arr[i], i);
            }

            int result = Integer.MAX_VALUE;
            int numRows = mat.length;
            int numCols = mat[0].length;

            for (int row = 0; row < numRows; row++) {
                int lastElementIndex = Integer.MIN_VALUE;
                for (int col = 0; col < numCols; col++) {
                    int indexVal = numToIndex.get(mat[row][col]);
                    lastElementIndex = Math.max(lastElementIndex, indexVal);
                }
                result = Math.min(result, lastElementIndex);
            }

            for (int col = 0; col < numCols; col++) {
                int lastElementIndex = Integer.MIN_VALUE;
                for (int row = 0; row < numRows; row++) {
                    int indexVal = numToIndex.get(mat[row][col]);
                    lastElementIndex = Math.max(lastElementIndex, indexVal);
                }
                result = Math.min(result, lastElementIndex);
            }

            return result;
        }
    }

    // https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/description/
    static class Solution2 {
        public int minCost(int[][] grid) {
            int numRows = grid.length, numCols = grid[0].length;
            int[][] minChanges = new int[numRows][numCols];

            for (int[] row : minChanges) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }
            minChanges[0][0] = 0;

            while (true) {
                int[][] prevState = new int[numRows][numCols];
                for (int row = 0; row < numRows; row++) {
                    prevState[row] = Arrays.copyOf(minChanges[row], numCols);
                }

                for (int row = 0; row < numRows; row++) {
                    for (int col = 0; col < numCols; col++) {
                        if (row > 0) {
                            minChanges[row][col] = Math.min(minChanges[row][col],
                                    minChanges[row - 1][col] + (grid[row - 1][col] == 3 ? 0 : 1));
                        }

                        if (col > 0) {
                            minChanges[row][col] = Math.min(minChanges[row][col],
                                    minChanges[row][col - 1] + (grid[row][col - 1] == 1 ? 0 : 1));
                        }
                    }
                }

                for (int row = numRows - 1; row >= 0; row--) {
                    for (int col = numCols - 1; col >= 0; col--) {
                        if (row < numRows - 1) {
                            minChanges[row][col] = Math.min(minChanges[row][col],
                                    minChanges[row + 1][col] + (grid[row + 1][col] == 4 ? 0 : 1));
                        }

                        if (col < numCols - 1) {
                            minChanges[row][col] = Math.min(minChanges[row][col],
                                    minChanges[row][col + 1] + (grid[row][col + 1] == 2 ? 0 : 1));
                        }
                    }
                }

                if (Arrays.deepEquals(prevState, minChanges)) {
                    break;
                }
            }

            return minChanges[numRows - 1][numCols - 1];
        }
    }

}
