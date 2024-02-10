package leetcode_grind;

public class Day456 {
    // https://leetcode.com/problems/palindromic-substrings/submissions
    static class Solution1 {
        public int countSubstrings(String s) {
            var n = s.length();
            var ans = 0;

            for (int m = 0; m < n; m++) {
                for (int j = 0; j <= 1; j++) {
                    int l = m;
                    int r = m + j;

                    while (0 <= l && r < n && s.charAt(l) == s.charAt(r)) {
                        ans++;
                        l--;
                        r++;
                    }
                }
            }

            return ans;
        }
    }
}
