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

    static class Solution2 {
        public int totalMoney(int n) {
            int k = n / 7;
            int F = 28;
            int L = 28 + (k - 1) * 7;
            int arithmeticSum = k * (F + L) / 2;

            int monday = 1 + k;
            int finalWeek = 0;
            for (int day = 0; day < n % 7; day++) {
                finalWeek += monday + day;
            }
            return arithmeticSum + finalWeek;
        }
    }
}
