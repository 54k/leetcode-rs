package leetcode_grind;

import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class Day411 {
    // https://leetcode.com/problems/string-compression-ii/description
    static class Solution1 {
        Map<Integer, Integer> memo = new HashMap<>();
        Set<Integer> add = Set.of(1, 9, 99);

        public int getLengthOfOptimalCompression(String s, int k) {
            return dp(s, 0, (char) ('a' + 26), 0, k);
        }

        int dp(String s, int idx, char lastChar, int lastCharCount, int k) {
            if (k < 0) {
                return Integer.MAX_VALUE / 2;
            }

            if (idx == s.length()) {
                return 0;
            }

            int key = idx * 101 * 27 * 101 + (lastChar - 'a') * 101 * 101 + lastCharCount * 101 + k;

            if (memo.containsKey(key)) {
                return memo.get(key);
            }

            int keepChar;
            int deleteChar = dp(s, idx + 1, lastChar, lastCharCount, k - 1);
            if (s.charAt(idx) == lastChar) {
                keepChar = dp(s, idx + 1, lastChar, lastCharCount + 1, k) + (add.contains(lastCharCount) ? 1 : 0);
            } else {
                keepChar = dp(s, idx + 1, s.charAt(idx), 1, k) + 1;
            }

            int res = Math.min(keepChar, deleteChar);
            memo.put(key, res);
            return res;
        }
    }
}
