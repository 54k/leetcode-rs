package leetcode_grind;

public class Day641 {
    // https://leetcode.com/problems/2-keys-keyboard/description/?envType=daily-question&envId=2024-08-19
    static class Solution1 {
        public int minSteps(int n) {
            int ans = 0;
            int d = 2;
            while (n > 1) {
                while (n % d == 0) {
                    ans += d;
                    n /= d;
                }
                d++;
            }
            return ans;
        }
    }
}
