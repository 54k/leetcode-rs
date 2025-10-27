package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1073 {
    // https://leetcode.com/problems/number-of-laser-beams-in-a-bank/description/
    static class Solution1 {
        public int numberOfBeams(String[] bank) {
            var ans = 0;
            var prev = 0;
            for (var s : bank) {
                var cur = 0;
                for (var ch : s.toCharArray()) {
                    cur += ch - '0';
                }
                if (cur > 0) {
                    ans += prev * cur;
                    prev = cur;
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
    static class Solution2 {
        public int lengthOfLongestSubstring(String s) {
            Map<Character, Integer> chars = new HashMap<>();
            int left = 0;
            int right = 0;
            int res = 0;
            while (right < s.length()) {
                char r = s.charAt(right);
                chars.put(r, chars.getOrDefault(r, 0) + 1);

                while (chars.get(r) > 1) {
                    char l = s.charAt(left);
                    chars.put(l, chars.get(l) - 1);
                    left++;
                }

                res = Math.max(res, right - left + 1);
                right++;
            }
            return res;
        }
    }

    static class Solution3 {
        public int lengthOfLongestSubstring(String s) {
            int n = s.length(), ans = 0;
            Map<Character, Integer> charToNextIndex = new HashMap<>();
            for (int j = 0, i = 0; j < n; j++) {
                if (charToNextIndex.containsKey(s.charAt(j))) {
                    i = Math.max(charToNextIndex.get(s.charAt(j)), i);
                }
                ans = Math.max(ans, j - i + 1);
                charToNextIndex.put(s.charAt(j), j + 1);
            }
            return ans;
        }
    }
}
