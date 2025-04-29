package leetcode_grind;

import java.util.*;

public class Day892 {
    // https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/description/?envType=daily-question&envId=2025-04-29
    static class Solution1 {
        public long countSubarrays(int[] nums, int k) {
            int maxElement = Arrays.stream(nums).max().getAsInt();
            long ans = 0, start = 0;
            int maxElementsInWindow = 0;

            for (int end = 0; end < nums.length; end++) {
                if (nums[end] == maxElement) {
                    maxElementsInWindow++;
                }
                while (maxElementsInWindow == k) {
                    if (nums[(int) start] == maxElement) {
                        maxElementsInWindow--;
                    }
                    start++;
                }
                ans += start;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/sparse-matrix-multiplication/description/?envType=weekly-question&envId=2025-04-29
    static class Solution2 {
        static class SparseMatrix {
            int cols = 0, rows = 0;
            List<Integer> values = new ArrayList<>();
            List<Integer> colIndex = new ArrayList<>();
            List<Integer> rowIndex = new ArrayList<>();

            SparseMatrix(int[][] matrix) {
                rows = matrix.length;
                cols = matrix[0].length;
                rowIndex.add(0);

                for (int row = 0; row < rows; ++row) {
                    for (int col = 0; col < cols; ++col) {
                        if (matrix[row][col] != 0) {
                            values.add(matrix[row][col]);
                            colIndex.add(col);
                        }
                    }
                    rowIndex.add(values.size());
                }
            }

            SparseMatrix(int[][] matrix, boolean colWise) {
                rows = matrix.length;
                cols = matrix[0].length;
                colIndex.add(0);

                for (int col = 0; col < cols; ++col) {
                    for (int row = 0; row < rows; ++row) {
                        if (matrix[row][col] != 0) {
                            values.add(matrix[row][col]);
                            rowIndex.add(row);
                        }
                    }
                    colIndex.add(values.size());
                }
            }
        }

        public int[][] multiply(int[][] mat1, int[][] mat2) {
            SparseMatrix A = new SparseMatrix(mat1);
            SparseMatrix B = new SparseMatrix(mat2, true);

            int[][] ans = new int[mat1.length][mat2[0].length];

            for (int row = 0; row < ans.length; ++row) {
                for (int col = 0; col < ans[0].length; ++col) {
                    int matrixOneRowStart = A.rowIndex.get(row);
                    int matrixOneRowEnd = A.rowIndex.get(row + 1);

                    int matrixTwoColStart = B.colIndex.get(col);
                    int matrixTwoColEnd = B.colIndex.get(col + 1);

                    while (matrixOneRowStart < matrixOneRowEnd && matrixTwoColStart < matrixTwoColEnd) {
                        if (A.colIndex.get(matrixOneRowStart) < B.rowIndex.get(matrixTwoColStart)) {
                            matrixOneRowStart++;
                        } else if (A.colIndex.get(matrixOneRowStart) > B.rowIndex.get(matrixTwoColStart)) {
                            matrixTwoColStart++;
                        } else {
                            ans[row][col] += A.values.get(matrixOneRowStart) * B.values.get(matrixTwoColStart);
                            matrixOneRowStart++;
                            matrixTwoColStart++;
                        }
                    }
                }
            }
            return ans;
        }
    }
}
