package leetcode_grind;

import java.util.*;

public class Day736 {
    // https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/description/?envType=daily-question&envId=2024-11-22
    static class Solution1 {
        public int maxEqualRowsAfterFlips(int[][] matrix) {
            int numCols = matrix[0].length;
            int maxIdenticalRows = 0;

            for (int[] currentRow : matrix) {
                int[] flippedRow = new int[numCols];
                int identicalRowCount = 0;

                for (int col = 0; col < numCols; col++) {
                    flippedRow[col] = 1 - currentRow[col];
                }

                for (int[] compareRow : matrix) {
                    if (Arrays.equals(compareRow, currentRow) || Arrays.equals(compareRow, flippedRow)) {
                        identicalRowCount++;
                    }
                }

                maxIdenticalRows = Math.max(maxIdenticalRows, identicalRowCount);
            }

            return maxIdenticalRows;
        }
    }

    static class Solution2 {
        public int maxEqualRowsAfterFlips(int[][] matrix) {
            Map<String, Integer> patternFrequency = new HashMap<>();

            for (int[] currentRow : matrix) {
                StringBuilder patternBuilder = new StringBuilder("");

                for (int col = 0; col < currentRow.length; col++) {
                    if (currentRow[0] == currentRow[col]) {
                        patternBuilder.append("T");
                    } else {
                        patternBuilder.append("F");
                    }
                }

                String rowPattern = patternBuilder.toString();
                patternFrequency.put(rowPattern, patternFrequency.getOrDefault(rowPattern, 0) + 1);
            }

            int maxFrequency = 0;
            for (int frequency : patternFrequency.values()) {
                maxFrequency = Math.max(frequency, maxFrequency);
            }
            return maxFrequency;
        }
    }

    // https://leetcode.com/problems/minimize-maximum-value-in-a-grid/description/?envType=weekly-question&envId=2024-11-22
    static class Solution3 {
        public int[][] minScore(int[][] grid) {
            int rows = grid.length;
            int cols = grid[0].length;

            List<int[]> nums = new ArrayList<>();

            int[] rowMax = new int[rows];
            int[] colMax = new int[cols];
            Arrays.fill(rowMax, 1);
            Arrays.fill(colMax, 1);

            for (int i = 0; i < rows; i++) {
                for (int j = 0; j < cols; j++) {
                    nums.add(new int[] { grid[i][j], i, j });
                }
            }

            nums.sort((a, b) -> Integer.compare(a[0], b[0]));

            for (int[] num : nums) {
                int x = num[1];
                int y = num[2];

                int newValue = Math.max(rowMax[x], colMax[y]);
                grid[x][y] = newValue;

                rowMax[x] = newValue + 1;
                colMax[y] = newValue + 1;
            }

            return grid;
        }
    }

    static class Solution4 {
        public int[][] minScore(int[][] grid) {
            int rowSize = grid.length, colSize = grid[0].length;
            PriorityQueue<int[]> min_heap = new PriorityQueue<>((a, b) -> a[0] - b[0]);

            int[] rows = new int[rowSize];
            int[] cols = new int[colSize];
            for (int i = 0; i < rowSize; i++)
                rows[i] = 1;
            for (int j = 0; j < colSize; j++)
                cols[j] = 1;

            for (int i = 0; i < rowSize; i++) {
                for (int j = 0; j < colSize; j++) {
                    min_heap.offer(new int[] { grid[i][j], i, j });
                }
            }

            while (!min_heap.isEmpty()) {
                int[] element = min_heap.poll();
                int row = element[1];
                int col = element[2];

                int val = Math.max(rows[row], cols[col]);
                grid[row][col] = val;

                rows[row] = val + 1;
                cols[col] = val + 1;
            }

            return grid;
        }
    }
}
