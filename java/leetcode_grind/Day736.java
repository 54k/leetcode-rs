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

}
