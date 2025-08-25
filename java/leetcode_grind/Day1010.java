package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Day1010 {
    // https://leetcode.com/problems/diagonal-traverse/description/?envType=daily-question&envId=2025-08-25
    static class Solution1 {
        public int[] findDiagonalOrder(int[][] matrix) {
            if (matrix == null || matrix.length == 0) {
                return new int[0];
            }

            int N = matrix.length;
            int M = matrix[0].length;

            int[] result = new int[N * M];
            int k = 0;
            List<Integer> intermediate = new ArrayList<>();
            for (int d = 0; d < N + M - 1; d++) {
                intermediate.clear();
                int r = d < M ? 0 : d - M + 1;
                int c = d < M ? d : M - 1;

                while (r < N && c > -1) {
                    intermediate.add(matrix[r][c]);
                    ++r;
                    --c;
                }

                if (d % 2 == 0) {
                    Collections.reverse(intermediate);
                }

                for (int i = 0; i < intermediate.size(); i++) {
                    result[k++] = intermediate.get(i);
                }
            }
            return result;
        }
    }

    static class Solution2 {
        public int[] findDiagonalOrder(int[][] matrix) {
            if (matrix == null || matrix.length == 0) {
                return new int[0];
            }

            int N = matrix.length;
            int M = matrix[0].length;

            int row = 0, column = 0;
            int direction = 1;

            int[] result = new int[N * M];
            int r = 0;

            while (row < N && column < M) {
                result[r++] = matrix[row][column];
                int new_row = row + (direction == 1 ? -1 : 1);
                int new_column = column + (direction == 1 ? 1 : -1);

                if (new_row < 0 || new_row == N || new_column < 0 || new_column == M) {
                    if (direction == 1) {
                        row += (column == M - 1 ? 1 : 0);
                        column += (column < M - 1 ? 1 : 0);
                    } else {
                        column += (row == N - 1 ? 1 : 0);
                        row += (row < N - 1 ? 1 : 0);
                    }

                    direction = 1 - direction;
                } else {
                    row = new_row;
                    column = new_column;
                }
            }

            return result;
        }
    }

}
