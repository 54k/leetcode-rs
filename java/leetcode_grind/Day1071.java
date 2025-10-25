package leetcode_grind;

public class Day1071 {
    // https://leetcode.com/problems/calculate-money-in-leetcode-bank/description/?envType=daily-question&envId=2025-10-25
    static class Solution1 {
        public int totalMoney(int n) {
            int ans = 0;
            int monday = 1;
            while (n > 0) {
                for (int day = 0; day < Math.min(n, 7); day++) {
                    ans += monday + day;
                }
                n -= 7;
                monday++;
            }
            return ans;
        }
    }
}
