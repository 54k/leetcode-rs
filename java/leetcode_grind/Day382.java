package leetcode_grind;

public class Day382 {
    // https://leetcode.com/problems/number-of-1-bits/
    public class Solution1 {
        // you need to treat n as an unsigned value
        public int hammingWeight(int n) {
            var ans = 0;
            while (n != 0) {
                ans += n & 1;
                n >>>= 1;
            }
            return ans;
        }
    }
}
