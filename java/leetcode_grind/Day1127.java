package leetcode_grind;

public class Day1127 {
    // https://leetcode.com/problems/delete-columns-to-make-sorted/description/?envType=daily-question&envId=2025-12-20
    static class Solution1 {
        public int minDeletionSize(String[] strs) {
            // String length.
            int K = strs[0].length();

            // Variable to store the count of columns to be deleted.
            int answer = 0;
            // Iterate over each index in the string.
            for (int col = 0; col < K; col++) {
                // Iterate over the strings.
                for (int row = 1; row < strs.length; row++) {
                    // Characters should be in increasing order,
                    // If not, increment the counter.
                    if (strs[row].charAt(col) < strs[row - 1].charAt(col)) {
                        answer++;
                        break;
                    }
                }
            }

            return answer;
        }
    }
}
