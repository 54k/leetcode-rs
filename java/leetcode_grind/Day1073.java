package leetcode_grind;

public class Day1073 {
    // https://leetcode.com/problems/number-of-laser-beams-in-a-bank/description/
    static class Solution {
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
}
