package leetcode_grind;

import java.util.Arrays;

public class Day477 {
    // https://leetcode.com/problems/bag-of-tokens/
    static class Solution1 {
        public int bagOfTokensScore(int[] tokens, int power) {
            Arrays.sort(tokens);
            int score = 0, i = 0, j = tokens.length - 1;
            while (i <= j) {
                if (tokens[i] <= power) {
                    power -= tokens[i];
                    score++;
                    i++;
                } else if (i < j && score > 0) {
                    power += tokens[j];
                    score--;
                    j--;
                } else {
                    i++;
                    j--;
                }
            }
            return score;
        }
    }

    // https://leetcode.com/problems/longest-uncommon-subsequence-ii/
    static class Solution2 {
        boolean isSubsequence(String x, String y) {
            int j = 0;
            for (int i = 0; i < y.length() && j < x.length(); i++) {
                if (x.charAt(j) == y.charAt(i)) {
                    j++;
                }
            }
            return j == x.length();
        }

        public int findLUSlength(String[] strs) {
            Arrays.sort(strs, (a, b) -> b.length() - a.length());
            for (int i = 0; i < strs.length; i++) {
                boolean flag = true;
                for (int j = 0; j < strs.length; j++) {
                    if (i == j) {
                        continue;
                    }
                    if (isSubsequence(strs[i], strs[j])) {
                        flag = false;
                        break;
                    }
                }
                if (flag) {
                    return strs[i].length();
                }
            }
            return -1;
        }
    }
}
