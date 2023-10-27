package leetcode_grind;

public class Day349 {
    // https://leetcode.com/problems/matrix-diagonal-sum/description/
    static class Solution {
        public int diagonalSum(int[][] mat) {
            var sum = 0;
            for (int i = 0; i < mat.length; i++) {
                sum += mat[i][i] + mat[mat.length - i - 1][i];
            }
            if (mat.length % 2 == 1) {
                sum -= mat[mat.length / 2][mat.length / 2];
            }
            return sum;
        }
    }
}
