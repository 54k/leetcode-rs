package leetcode_grind;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

public class Day625 {
    // https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/description/?envType=daily-question&envId=2024-08-03
    static class Solution1 {
        public boolean canBeEqual(int[] target, int[] arr) {
            Arrays.sort(target);
            Arrays.sort(arr);
            for (int i = 0; i < arr.length; i++) {
                if (target[i] != arr[i]) {
                    return false;
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/longest-repeating-substring/description/?envType=weekly-question&envId=2024-08-01
    static class Solution2 {
        public int longestRepeatingSubstring(String s) {
            Set<String> seenSubstrings = new HashSet<>();
            int maxLength = s.length() - 1;
            for (int start = 0; start <= s.length(); start++) {
                int end = start;
                if (end + maxLength > s.length()) {
                    if (--maxLength == 0)
                        break;
                    start = -1;
                    seenSubstrings.clear();
                    continue;
                }

                String currentSubstring = s.substring(end, end + maxLength);
                if (!seenSubstrings.add(currentSubstring)) {
                    return maxLength;
                }
            }
            return maxLength;
        }
    }

    static class Solution3 {
        public int longestRepeatingSubstring(String s) {
            int length = s.length(), maxLength = 0;
            Set<String> seenSubstrings = new HashSet<>();

            for (int start = 0; start < length; start++) {
                int end = start;
                if (end + maxLength >= length) {
                    return maxLength;
                }
                String currentSubstring = s.substring(end, end + maxLength + 1);
                if (!seenSubstrings.add(currentSubstring)) {
                    start = -1;
                    seenSubstrings.clear();
                    maxLength++;
                }
            }
            return maxLength;
        }
    }
}
