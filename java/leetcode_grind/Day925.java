package leetcode_grind;

public class Day925 {
    // https://leetcode.com/problems/distribute-candies-among-children-ii/description/?envType=daily-question&envId=2025-06-01
    static class Solution1 {
        public long distributeCandies(int n, int limit) {
            long ans = 0;
            for (int i = 0; i <= Math.min(limit, n); i++) {
                if (n - i > 2 * limit) {
                    continue;
                }
                ans += Math.min(n - i, limit) - Math.max(0, n - i - limit) + 1;
            }
            return ans;
        }
    }
}
