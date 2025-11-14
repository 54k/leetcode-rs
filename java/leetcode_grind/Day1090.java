package leetcode_grind;

public class Day1090 {
    // https://leetcode.com/problems/maximum-number-of-operations-to-move-ones-to-the-end/description/?envType=daily-question&envId=2025-11-13
    static class Solution1 {
        public int maxOperations(String s) {
            int countOne = 0;
            int ans = 0;
            int i = 0;
            while (i < s.length()) {
                if (s.charAt(i) == '0') {
                    while (i + 1 < s.length() && s.charAt(i + 1) == '0') {
                        i++;
                    }
                    ans += countOne;
                } else {
                    countOne++;
                }
                i++;
            }
            return ans;
        }
    }
}
